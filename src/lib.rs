//! Refresh a Windows folder icon in Explorer.
//!
//! After a folder's `desktop.ini` is changed, Explorer may keep showing the old
//! icon until its shell cache is notified. This addon exposes two levels:
//!
//! - `refreshFolder(path)` — soft notify via Win32 `SHChangeNotify`
//!   (the same API the native "Change icon" dialog uses), non-destructive.
//! - `clearIconCache()` — stronger fallback that rebuilds the icon cache via
//!   `ie4uinit.exe -ClearIconCache`, without killing Explorer.
//!
//! Both are native (they touch OS APIs / shell). On non-Windows they are safe
//! no-ops returning `false`, so cross-platform Node code can call them
//! unconditionally.

#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

/// Notify the Windows shell that a folder changed so Explorer refreshes its
/// icon. Returns `true` if a notification was sent (Windows only), else `false`.
#[napi]
pub fn refresh_folder(path: String) -> bool {
    #[cfg(windows)]
    {
        windows_impl::refresh(&path)
    }
    #[cfg(not(windows))]
    {
        let _ = path;
        false
    }
}

/// Rebuild the Windows icon cache via `ie4uinit.exe -ClearIconCache` (does not
/// restart Explorer). Stronger than `refreshFolder` when the shell keeps a
/// stale icon. Returns `true` if the command was launched (Windows only).
#[napi]
pub fn clear_icon_cache() -> bool {
    #[cfg(windows)]
    {
        windows_impl::clear_icon_cache()
    }
    #[cfg(not(windows))]
    {
        false
    }
}

#[cfg(windows)]
mod windows_impl {
    use windows::core::HSTRING;
    use windows::Win32::UI::Shell::{
        ILCreateFromPathW, ILFree, SHChangeNotify, SHCNE_ASSOCCHANGED, SHCNE_UPDATEDIR,
        SHCNE_UPDATEITEM, SHCNF_FLUSH, SHCNF_IDLIST, SHCNF_PATHW,
    };

    pub fn refresh(path: &str) -> bool {
        let wide = HSTRING::from(path);
        unsafe {
            // SHCNF_FLUSH makes the shell process the notification right away
            // instead of queueing/coalescing it.

            // 1. Notify the exact item by its PIDL (like the shell does on an
            //    icon change).
            let pidl = ILCreateFromPathW(&wide);
            if !pidl.is_null() {
                SHChangeNotify(SHCNE_UPDATEITEM, SHCNF_IDLIST | SHCNF_FLUSH, Some(pidl as _), None);
                ILFree(Some(pidl));
            }

            // 2. Notify the item / its directory by path.
            SHChangeNotify(SHCNE_UPDATEITEM, SHCNF_PATHW | SHCNF_FLUSH, Some(wide.as_ptr() as _), None);
            SHChangeNotify(SHCNE_UPDATEDIR, SHCNF_PATHW | SHCNF_FLUSH, Some(wide.as_ptr() as _), None);

            // 3. Global association/icon change broadcast — this is what makes
            //    already-open Explorer windows re-read icons (closest to F5).
            SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_FLUSH, None, None);
        }
        true
    }

    pub fn clear_icon_cache() -> bool {
        use std::process::Command;
        // ie4uinit ships with Windows; -ClearIconCache rebuilds the cache
        // without restarting Explorer. Best-effort.
        Command::new("ie4uinit.exe")
            .arg("-ClearIconCache")
            .spawn()
            .map(|mut c| {
                let _ = c.wait();
            })
            .is_ok()
    }
}
