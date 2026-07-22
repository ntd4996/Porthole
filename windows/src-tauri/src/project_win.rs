//! Best-effort project resolution on Windows. Unlike Linux (`/proc/PID/cwd`) we
//! can't cheaply read another process's working directory, so we mine the
//! command line (and the executable path) for a filesystem path and let the
//! shared `project_resolver` walk its parents for a project marker.

use porthole_core::{project_resolver, ProjectInfo};

pub fn resolve(cmdline: Option<&str>, exepath: Option<&str>) -> Option<ProjectInfo> {
    for cand in candidates(cmdline, exepath) {
        // `project_resolver::resolve` walks parents lexically, so a leaf that
        // doesn't exist (e.g. `...\node_modules\.bin\vite`) still finds the
        // package.json above it.
        if let Some(info) = project_resolver::resolve(&cand) {
            return Some(info);
        }
    }
    None
}

fn candidates(cmdline: Option<&str>, exepath: Option<&str>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    if let Some(cl) = cmdline {
        // Quoted args capture paths that contain spaces.
        for part in cl.split('"') {
            let t = part.trim();
            if looks_like_path(t) {
                out.push(t.to_string());
            }
        }
        // Unquoted whitespace-separated args.
        for part in cl.split_whitespace() {
            if looks_like_path(part) {
                out.push(part.to_string());
            }
        }
    }
    if let Some(ep) = exepath {
        if looks_like_path(ep) {
            out.push(ep.to_string());
        }
    }
    out
}

/// A drive-letter absolute path like `C:\...` or `D:/...`.
fn looks_like_path(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() >= 3 && b[0].is_ascii_alphabetic() && b[1] == b':' && (b[2] == b'\\' || b[2] == b'/')
}
