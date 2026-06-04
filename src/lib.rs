//! Refresh a Windows folder icon in Explorer.
//!
//! Setting a folder icon writes `desktop.ini`, but Explorer often keeps showing
//! the cached icon. Notifying the shell isn't always enough — the icon cache
//! has to be rebuilt. This addon exposes several strategies so callers can pick
//! the one that works best on their machine.
//!
//! All functions are native (they touch OS APIs / shell / the icon cache).
//! On non-Windows they are safe no-ops returning `false`.

#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

/// Soft refresh: notify the shell via SHChangeNotify (UPDATEITEM/UPDATEDIR +
/// ASSOCCHANGED). Non-destructive. Often not enough on its own.
#[napi]
pub fn refresh_folder(path: String) -> bool {
    #[cfg(windows)]
    { windows_impl::notify(&path) }
    #[cfg(not(windows))]
    { let _ = path; false }
}

/// Strategy A: `ie4uinit.exe -show`. Lightweight refresh of the icon cache.
#[napi]
pub fn refresh_ie4uinit_show() -> bool {
    #[cfg(windows)]
    { windows_impl::run("ie4uinit.exe", &["-show"]) }
    #[cfg(not(windows))]
    { false }
}

/// Strategy B: `ie4uinit.exe -ClearIconCache`. Rebuilds the icon cache.
#[napi]
pub fn refresh_ie4uinit_clear() -> bool {
    #[cfg(windows)]
    { windows_impl::run("ie4uinit.exe", &["-ClearIconCache"]) }
    #[cfg(not(windows))]
    { false }
}

/// Strategy C: delete the per-user IconCache.db files, then notify the shell.
/// More forceful than ie4uinit; does NOT restart Explorer.
#[napi]
pub fn refresh_clear_cache_files(path: String) -> bool {
    #[cfg(windows)]
    { windows_impl::clear_cache_files(&path) }
    #[cfg(not(windows))]
    { let _ = path; false }
}

/// Strategy D: notify + delete cache files + ie4uinit. The most thorough soft
/// approach (still no Explorer restart).
#[napi]
pub fn refresh_all(path: String) -> bool {
    #[cfg(windows)]
    {
        let a = windows_impl::notify(&path);
        let b = windows_impl::clear_cache_files(&path);
        let c = windows_impl::run("ie4uinit.exe", &["-ClearIconCache"]);
        a || b || c
    }
    #[cfg(not(windows))]
    { let _ = path; false }
}

#[cfg(windows)]
mod windows_impl {
    use std::process::Command;
    use windows::core::HSTRING;
    use windows::Win32::UI::Shell::{
        ILCreateFromPathW, ILFree, SHChangeNotify, SHCNE_ASSOCCHANGED, SHCNE_UPDATEDIR,
        SHCNE_UPDATEITEM, SHCNF_FLUSH, SHCNF_IDLIST, SHCNF_PATHW,
    };

    pub fn notify(path: &str) -> bool {
        let wide = HSTRING::from(path);
        unsafe {
            let pidl = ILCreateFromPathW(&wide);
            if !pidl.is_null() {
                SHChangeNotify(SHCNE_UPDATEITEM, SHCNF_IDLIST | SHCNF_FLUSH, Some(pidl as _), None);
                ILFree(Some(pidl));
            }
            SHChangeNotify(SHCNE_UPDATEITEM, SHCNF_PATHW | SHCNF_FLUSH, Some(wide.as_ptr() as _), None);
            SHChangeNotify(SHCNE_UPDATEDIR, SHCNF_PATHW | SHCNF_FLUSH, Some(wide.as_ptr() as _), None);
            SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_FLUSH, None, None);
        }
        true
    }

    pub fn run(program: &str, args: &[&str]) -> bool {
        Command::new(program)
            .args(args)
            .spawn()
            .map(|mut c| { let _ = c.wait(); })
            .is_ok()
    }

    pub fn clear_cache_files(path: &str) -> bool {
        let local = match std::env::var("LOCALAPPDATA") {
            Ok(v) => v,
            Err(_) => return false,
        };
        let dir = format!(r"{local}\Microsoft\Windows\Explorer");
        // iconcache_*.db live in Explorer\, plus a legacy IconCache.db in LocalAppData.
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for e in entries.flatten() {
                let name = e.file_name();
                let name = name.to_string_lossy().to_lowercase();
                if name.starts_with("iconcache") && name.ends_with(".db") {
                    let _ = std::fs::remove_file(e.path());
                }
            }
        }
        let _ = std::fs::remove_file(format!(r"{local}\IconCache.db"));
        notify(path)
    }
}
