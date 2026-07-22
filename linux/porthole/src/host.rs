//! Runs commands and reads `/proc` on the host, transparently escaping the
//! Flatpak sandbox via `flatpak-spawn --host` when needed.
//!
//! Inside a Flatpak sandbox the app has its own PID namespace, so the host's
//! `/proc` and tools like `ss`/`tailscale` are invisible. We detect the sandbox
//! (`/.flatpak-info`) and forward every scan command out to the host, which the
//! manifest permits with `--talk-name=org.freedesktop.Flatpak`.

use std::process::Command;
use std::sync::OnceLock;

fn in_flatpak() -> bool {
    static FLAG: OnceLock<bool> = OnceLock::new();
    *FLAG.get_or_init(|| std::path::Path::new("/.flatpak-info").exists())
}

/// Build a `Command` that runs `program args...` on the host.
pub fn command(program: &str, args: &[&str]) -> Command {
    if in_flatpak() {
        let mut c = Command::new("flatpak-spawn");
        c.arg("--host").arg(program).args(args);
        c
    } else {
        let mut c = Command::new(program);
        c.args(args);
        c
    }
}

/// Run a command and return its stdout as a `String`, or `None` on any failure
/// (missing binary, non-zero exit is tolerated as long as there is output).
pub fn run(program: &str, args: &[&str]) -> Option<String> {
    let output = command(program, args).output().ok()?;
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    if text.is_empty() && !output.status.success() {
        None
    } else {
        Some(text)
    }
}

/// Read a host file (a `/proc` path or a config file). Uses `cat` through the
/// host when sandboxed, direct filesystem access otherwise.
pub fn read_file(path: &str) -> Option<String> {
    if in_flatpak() {
        run("cat", &[path])
    } else {
        std::fs::read_to_string(path).ok()
    }
}

/// Read a symlink target (e.g. `/proc/PID/cwd`).
pub fn read_link(path: &str) -> Option<String> {
    if in_flatpak() {
        run("readlink", &[path]).map(|s| s.trim_end().to_string())
    } else {
        std::fs::read_link(path).ok().map(|p| p.to_string_lossy().into_owned())
    }
}

/// List entries of a host directory (e.g. `/proc`). Returns bare names.
pub fn list_dir(path: &str) -> Vec<String> {
    if in_flatpak() {
        run("ls", &["-1", path])
            .map(|s| s.lines().map(|l| l.trim().to_string()).collect())
            .unwrap_or_default()
    } else {
        std::fs::read_dir(path)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default()
    }
}
