//! Port actions. Linux counterpart of `App/Actions.swift`.

use gtk::prelude::*;

pub fn open_in_browser(port: u16) {
    open_url(&format!("http://localhost:{port}"));
}

pub fn open_url(url: &str) {
    // `xdg-open` picks the user's default browser (via the portal under Flatpak);
    // ignore failure (no browser installed).
    let _ = std::process::Command::new("xdg-open").arg(url).spawn();
}

pub fn copy(text: &str) {
    if let Some(display) = gtk::gdk::Display::default() {
        display.clipboard().set_text(text);
    }
}

/// SIGTERM the process. Returns false if the call fails.
pub fn kill(pid: i32) -> bool {
    // Safe: kill(2) with SIGTERM has no memory effects.
    unsafe { libc::kill(pid, libc::SIGTERM) == 0 }
}
