//! Persists the ignore list as JSON in `%APPDATA%\Porthole\ignore.json`.
//! Mirrors the Linux/macOS ignore store, seeded with Windows system services.

use porthole_core::{IgnoreRules, PortInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

const SEED_VERSION: u32 = 1;

/// Windows system processes that hold ports but are never dev servers.
const DEFAULT_PROCESSES: &[&str] = &[
    "System",
    "svchost",
    "lsass",
    "services",
    "wininit",
    "spoolsv",
    "System Idle Process",
];

#[derive(Serialize, Deserialize, Default)]
struct Persisted {
    #[serde(default)]
    processes: Vec<String>,
    #[serde(default)]
    ports: Vec<u16>,
    #[serde(default)]
    seed_version: u32,
}

pub struct IgnoreStore {
    pub rules: IgnoreRules,
    path: PathBuf,
}

impl IgnoreStore {
    pub fn load() -> Self {
        let path = config_path();
        match std::fs::read(&path).ok().and_then(|d| serde_json::from_slice::<Persisted>(&d).ok()) {
            None => {
                let rules = IgnoreRules::new(
                    DEFAULT_PROCESSES.iter().map(|s| s.to_string()).collect(),
                    HashSet::new(),
                );
                let mut store = Self { rules, path };
                store.persist();
                store
            }
            Some(saved) => {
                let mut rules = IgnoreRules::new(
                    saved.processes.into_iter().collect(),
                    saved.ports.into_iter().collect(),
                );
                let mut changed = false;
                if saved.seed_version < SEED_VERSION {
                    for p in DEFAULT_PROCESSES {
                        rules.processes.insert(p.to_string());
                    }
                    changed = true;
                }
                let mut store = Self { rules, path };
                if changed {
                    store.persist();
                }
                store
            }
        }
    }

    pub fn ignore_process(&mut self, name: &str) {
        if !name.is_empty() {
            self.rules.processes.insert(name.to_string());
            self.persist();
        }
    }

    pub fn ignore_port(&mut self, port: u16) {
        self.rules.ports.insert(port);
        self.persist();
    }

    pub fn unignore_process(&mut self, name: &str) {
        self.rules.processes.remove(name);
        self.persist();
    }

    pub fn unignore_port(&mut self, port: u16) {
        self.rules.ports.remove(&port);
        self.persist();
    }

    /// Remove whichever rule (port or process name) hides this port.
    pub fn unignore_matching(&mut self, port: u16, command: &str, display_name: &str) {
        self.rules.ports.remove(&port);
        let doomed: Vec<String> = self
            .rules
            .processes
            .iter()
            .filter(|n| n.eq_ignore_ascii_case(command) || n.eq_ignore_ascii_case(display_name))
            .cloned()
            .collect();
        for n in doomed {
            self.rules.processes.remove(&n);
        }
        self.persist();
    }

    pub fn is_ignored(&self, port: &PortInfo) -> bool {
        self.rules.is_ignored(port)
    }

    fn persist(&self) {
        let data = Persisted {
            processes: self.rules.processes.iter().cloned().collect(),
            ports: self.rules.ports.iter().copied().collect(),
            seed_version: SEED_VERSION,
        };
        if let Some(parent) = self.path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_vec_pretty(&data) {
            let _ = std::fs::write(&self.path, json);
        }
    }
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Porthole")
        .join("ignore.json")
}
