//! Mirror of `PortholeCore/ProjectResolver.swift`. Walks up from a working
//! directory looking for a project marker.

use crate::models::{ProjectInfo, ProjectKind};
use std::path::{Path, PathBuf};

pub fn resolve(cwd: &str) -> Option<ProjectInfo> {
    let mut dir = PathBuf::from(cwd);
    loop {
        if let Some(info) = marker(&dir) {
            return Some(info);
        }
        match dir.parent() {
            Some(parent) if parent != dir => dir = parent.to_path_buf(),
            _ => return None, // reached root
        }
    }
}

fn marker(dir: &Path) -> Option<ProjectInfo> {
    let has = |name: &str| dir.join(name).exists();
    let path = dir.to_string_lossy().to_string();
    let basename = dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    if has("package.json") {
        let name = package_name(&dir.join("package.json")).unwrap_or(basename);
        return Some(ProjectInfo { path, name, kind: ProjectKind::Node });
    }
    if has("go.mod") {
        return Some(ProjectInfo { path, name: basename, kind: ProjectKind::Go });
    }
    if has("pyproject.toml") || has("requirements.txt") {
        return Some(ProjectInfo { path, name: basename, kind: ProjectKind::Python });
    }
    if has("Gemfile") {
        return Some(ProjectInfo { path, name: basename, kind: ProjectKind::Ruby });
    }
    if has("Cargo.toml") {
        return Some(ProjectInfo { path, name: basename, kind: ProjectKind::Rust });
    }
    if has(".git") {
        return Some(ProjectInfo { path, name: basename, kind: ProjectKind::Unknown });
    }
    None
}

fn package_name(path: &Path) -> Option<String> {
    let data = std::fs::read(path).ok()?;
    let obj: serde_json::Value = serde_json::from_slice(&data).ok()?;
    let name = obj.get("name")?.as_str()?;
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn reads_name_from_package_json() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("package.json"), r#"{"name":"roomify"}"#).unwrap();
        let sub = root.path().join("src/server");
        fs::create_dir_all(&sub).unwrap();

        let info = resolve(sub.to_str().unwrap()).unwrap();
        assert_eq!(info.name, "roomify");
        assert_eq!(info.kind, ProjectKind::Node);
        assert_eq!(info.path, root.path().to_string_lossy());
    }

    #[test]
    fn git_root_fallback_uses_basename() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join(".git")).unwrap();

        let info = resolve(root.path().to_str().unwrap()).unwrap();
        assert_eq!(info.name, root.path().file_name().unwrap().to_string_lossy());
        assert_eq!(info.kind, ProjectKind::Unknown);
    }

    #[test]
    fn returns_none_without_marker() {
        let root = tempfile::tempdir().unwrap();
        assert_eq!(resolve(root.path().to_str().unwrap()), None);
    }
}
