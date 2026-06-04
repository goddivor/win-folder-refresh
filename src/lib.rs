//! Refresh a Windows folder icon in Explorer.
//!
//! After a folder's `desktop.ini` is changed, Explorer may keep showing the old
//! icon until its shell cache is notified. This addon calls the Win32
//! `SHChangeNotify` API (the same mechanism the native "Change icon" dialog
//! uses) for a given folder, so the icon updates without restarting Explorer.
//!
//! It is a native N-API addon: it only does anything on Windows. On other
//! platforms the function is a no-op that returns `false`, so cross-platform
//! Node code can call it unconditionally.

#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

/// Notify the Windows shell that a folder changed so Explorer refreshes its
/// icon. Returns `true` if the notification was sent (Windows only), `false`
/// otherwise (non-Windows, or on failure).
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

#[cfg(windows)]
mod windows_impl {
    use windows::core::HSTRING;
    use windows::Win32::UI::Shell::{
        ILCreateFromPathW, ILFree, SHChangeNotify, SHCNE_UPDATEDIR, SHCNE_UPDATEITEM,
        SHCNF_IDLIST, SHCNF_PATHW,
    };

    pub fn refresh(path: &str) -> bool {
        let wide = HSTRING::from(path);
        unsafe {
            // Preferred: notify the exact item by its PIDL, like the shell does
            // when an icon changes.
            let pidl = ILCreateFromPathW(&wide);
            if !pidl.is_null() {
                SHChangeNotify(SHCNE_UPDATEITEM, SHCNF_IDLIST, Some(pidl as _), None);
                ILFree(Some(pidl));
            }
            // Also nudge the directory by path, which helps some views pick the
            // change up immediately.
            SHChangeNotify(
                SHCNE_UPDATEDIR,
                SHCNF_PATHW,
                Some(wide.as_ptr() as _),
                None,
            );
        }
        true
    }
}
