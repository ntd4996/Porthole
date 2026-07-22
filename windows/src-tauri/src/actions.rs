//! Port actions on Windows. Windows counterpart of `Actions.swift` / Linux `actions.rs`.

use std::os::windows::process::CommandExt;
use std::process::Command;

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn open_url(url: &str) {
    // `cmd /C start "" <url>` opens the default browser. The empty title arg is
    // required so a quoted URL isn't treated as the window title.
    let _ = Command::new("cmd")
        .args(["/C", "start", "", url])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn();
}

/// Terminate the process (and its children). Windows has no SIGTERM; `taskkill`
/// is the standard equivalent. Returns false if the call fails.
pub fn kill(pid: i32) -> bool {
    Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/T", "/F"])
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
