//! Persists the ignore list as JSON in `~/.config/porthole/ignore.json`.
//! Linux counterpart of `App/IgnoreStore.swift` (which used UserDefaults).

use porthole_core::{IgnoreRules, PortInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

/// Bump when new entries are added to `DEFAULT_PROCESSES` so existing installs
/// pick them up once (without clobbering the user's own removals afterwards).
const SEED_VERSION: u32 = 1;

/// Noisy Linux system services that hold ports but are never dev servers.
const DEFAULT_PROCESSES: &[&str] = &[
    "systemd-resolve",
    "systemd-resolved",
    "avahi-daemon",
    "cupsd",
    "cups-browsed",
    "dnsmasq",
    "chronyd",
    "containerd",
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
                // First launch: seed defaults.
                let rules = IgnoreRules::new(
                    DEFAULT_PROCESSES.iter().map(|s| s.to_string()).collect(),
                    HashSet::new(),
                );
                let mut store = Self { rules, path };
                store.persist_with_seed(SEED_VERSION);
                store
            }
            Some(saved) => {
                let mut rules = IgnoreRules::new(
                    saved.processes.into_iter().collect(),
                    saved.ports.into_iter().collect(),
                );
                let mut changed = false;
                if saved.seed_version < SEED_VERSION {
                    // One-time merge of newly-added defaults for older installs.
                    for p in DEFAULT_PROCESSES {
                        rules.processes.insert(p.to_string());
                    }
                    changed = true;
                }
                let mut store = Self { rules, path };
                if changed {
                    store.persist_with_seed(SEED_VERSION);
                }
                store
            }
        }
    }

    pub fn ignore_process(&mut self, name: &str) {
        if name.is_empty() {
            return;
        }
        self.rules.processes.insert(name.to_string());
        self.persist();
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

    /// Removes whichever rule (port number or process name) currently hides this port.
    pub fn unignore_matching(&mut self, port: &PortInfo) {
        self.rules.ports.remove(&port.port);
        let doomed: Vec<String> = self
            .rules
            .processes
            .iter()
            .filter(|name| {
                name.eq_ignore_ascii_case(&port.command)
                    || name.eq_ignore_ascii_case(&port.display_name)
            })
            .cloned()
            .collect();
        for name in doomed {
            self.rules.processes.remove(&name);
        }
        self.persist();
    }

    fn persist(&mut self) {
        self.persist_with_seed(SEED_VERSION);
    }

    fn persist_with_seed(&mut self, seed_version: u32) {
        let data = Persisted {
            processes: self.rules.processes.iter().cloned().collect(),
            ports: self.rules.ports.iter().copied().collect(),
            seed_version,
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
        .join("porthole")
        .join("ignore.json")
}
