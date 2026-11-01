//! This build never contacts the internet: there is no update check. Kept as a stub so the rest of the app is unchanged.

use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::{PCWSTR, w};

#[allow(dead_code)]
pub struct Release {
    pub version: String,
    pub url: String,
}

/// Always "nothing new": no request is made.
pub fn check() -> Result<Option<Release>, String> {
    Ok(None)
}

#[allow(dead_code)]
pub fn open(url: &str) {
    let url: Vec<u16> = url.encode_utf16().chain([0]).collect();
    unsafe {
        ShellExecuteW(None, w!("open"), PCWSTR(url.as_ptr()), PCWSTR::null(), PCWSTR::null(), SW_SHOWNORMAL);
    }
}
