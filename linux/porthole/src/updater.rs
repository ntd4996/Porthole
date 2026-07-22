//! In-app update check. Linux counterpart of Sparkle on macOS.
//!
//! Queries the GitHub Releases API for the newest `linux-v*` tag. When the app
//! runs as an AppImage (`$APPIMAGE` set) it can download and swap the binary in
//! place; otherwise it just reports availability and points at the releases page
//! (deb/flatpak update through the package manager).

use std::time::Duration;

const RELEASES_API: &str = "https://api.github.com/repos/ntd4996/Porthole/releases";
pub const RELEASES_PAGE: &str = "https://github.com/ntd4996/Porthole/releases";

#[derive(Debug)]
pub enum Outcome {
    UpToDate,
    /// A newer version exists; `applied` is true when an AppImage was swapped in.
    Available { version: String, applied: bool },
    Error(String),
}

pub fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Blocking: run from a background thread.
pub fn check() -> Outcome {
    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent(format!("Porthole/{}", current_version()))
        .build()
    {
        Ok(c) => c,
        Err(e) => return Outcome::Error(e.to_string()),
    };

    let releases: serde_json::Value = match client
        .get(RELEASES_API)
        .send()
        .and_then(|r| r.error_for_status())
        .and_then(|r| r.json())
    {
        Ok(v) => v,
        Err(e) => return Outcome::Error(e.to_string()),
    };

    // Newest Linux release = first entry whose tag starts with `linux-v`.
    let Some(release) = releases.as_array().and_then(|arr| {
        arr.iter().find(|r| {
            r.get("tag_name")
                .and_then(|t| t.as_str())
                .map(|t| t.starts_with("linux-v"))
                .unwrap_or(false)
        })
    }) else {
        return Outcome::Error("no linux release found".into());
    };

    let tag = release.get("tag_name").and_then(|t| t.as_str()).unwrap_or("");
    let latest = tag.trim_start_matches("linux-v");
    if !is_newer(latest, current_version()) {
        return Outcome::UpToDate;
    }

    // Newer version available. Try to self-update if running as AppImage.
    if let Ok(appimage) = std::env::var("APPIMAGE") {
        match download_appimage(&client, release, &appimage) {
            Ok(()) => Outcome::Available { version: latest.to_string(), applied: true },
            Err(e) => Outcome::Error(e),
        }
    } else {
        Outcome::Available { version: latest.to_string(), applied: false }
    }
}

fn download_appimage(
    client: &reqwest::blocking::Client,
    release: &serde_json::Value,
    target: &str,
) -> Result<(), String> {
    let asset = release
        .get("assets")
        .and_then(|a| a.as_array())
        .and_then(|arr| {
            arr.iter().find(|a| {
                a.get("name")
                    .and_then(|n| n.as_str())
                    .map(|n| n.ends_with(".AppImage"))
                    .unwrap_or(false)
            })
        })
        .ok_or("no AppImage asset in release")?;
    let url = asset
        .get("browser_download_url")
        .and_then(|u| u.as_str())
        .ok_or("asset has no download url")?;

    let bytes = client
        .get(url)
        .send()
        .and_then(|r| r.error_for_status())
        .and_then(|r| r.bytes())
        .map_err(|e| e.to_string())?;

    // Write next to the current AppImage, then atomically rename over it.
    let tmp = format!("{target}.new");
    std::fs::write(&tmp, &bytes).map_err(|e| e.to_string())?;
    make_executable(&tmp)?;
    std::fs::rename(&tmp, target).map_err(|e| e.to_string())?;
    Ok(())
}

fn make_executable(path: &str) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path).map_err(|e| e.to_string())?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms).map_err(|e| e.to_string())
}

/// Compare dotted numeric versions. `a > b` -> true.
fn is_newer(a: &str, b: &str) -> bool {
    let parse = |s: &str| -> Vec<u32> {
        s.split('.').map(|p| p.parse::<u32>().unwrap_or(0)).collect()
    };
    let (va, vb) = (parse(a), parse(b));
    let n = va.len().max(vb.len());
    for i in 0..n {
        let x = va.get(i).copied().unwrap_or(0);
        let y = vb.get(i).copied().unwrap_or(0);
        if x != y {
            return x > y;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::is_newer;

    #[test]
    fn compares_versions() {
        assert!(is_newer("0.2.0", "0.1.0"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(!is_newer("0.1.0", "0.1.0"));
        assert!(!is_newer("0.1.0", "0.2.0"));
        assert!(is_newer("0.1.1", "0.1.0"));
    }
}
