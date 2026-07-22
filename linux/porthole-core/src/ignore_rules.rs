//! Mirror of `PortholeCore/IgnoreRules.swift`.

use crate::models::PortInfo;
use std::collections::HashSet;

/// Rules deciding which ports to hide from the main list. A port is ignored when
/// its number is in `ports`, or its process command/display name matches an entry
/// in `processes` (case-insensitive).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IgnoreRules {
    pub processes: HashSet<String>,
    pub ports: HashSet<u16>,
}

impl IgnoreRules {
    pub fn new(processes: HashSet<String>, ports: HashSet<u16>) -> Self {
        Self { processes, ports }
    }

    pub fn is_ignored(&self, port: &PortInfo) -> bool {
        if self.ports.contains(&port.port) {
            return true;
        }
        self.processes.iter().any(|name| {
            name.eq_ignore_ascii_case(&port.command) || name.eq_ignore_ascii_case(&port.display_name)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn port(p: u16, command: &str, display_name: Option<&str>) -> PortInfo {
        PortInfo {
            port: p,
            pid: 1,
            command: command.into(),
            display_name: display_name.unwrap_or(command).into(),
            project: None,
            tunnels: vec![],
        }
    }

    fn rules(processes: &[&str], ports: &[u16]) -> IgnoreRules {
        IgnoreRules::new(
            processes.iter().map(|s| s.to_string()).collect(),
            ports.iter().copied().collect(),
        )
    }

    #[test]
    fn ignores_by_port() {
        let r = rules(&[], &[7000]);
        assert!(r.is_ignored(&port(7000, "systemd-resolve", None)));
        assert!(!r.is_ignored(&port(3000, "node", None)));
    }

    #[test]
    fn ignores_by_process_case_insensitive_on_command_or_display_name() {
        let r = rules(&["avahi-daemon"], &[]);
        assert!(r.is_ignored(&port(5353, "avahi-daemon", None)));
        assert!(r.is_ignored(&port(5353, "AVAHI-DAEMON", None)));
        let r2 = rules(&["vite"], &[]);
        assert!(r2.is_ignored(&port(5173, "node", Some("vite"))));
    }

    #[test]
    fn not_ignored_when_no_match() {
        let r = rules(&["cupsd"], &[7000]);
        assert!(!r.is_ignored(&port(3000, "node", Some("vite"))));
    }
}
