//! Parse `ss -tlnpH` output into unique LISTEN sockets.
//! Linux counterpart of `PortholeCore/LsofParser.swift` (which parsed `lsof -F pcn`).

use crate::models::RawListen;
use std::collections::HashSet;

/// Parse `ss --tcp --listening --numeric --processes --no-header` output.
///
/// Each line looks like:
/// `LISTEN 0 511 0.0.0.0:3000 0.0.0.0:* users:(("node",pid=1234,fd=20))`
///
/// Sockets are de-duplicated on `(pid, port)` so the IPv4 and IPv6 bindings of
/// the same server collapse to one row. Lines without a process block still
/// yield a row (`pid = 0`, empty command) so the port stays visible.
pub fn parse_listens(output: &str) -> Vec<RawListen> {
    let mut seen: HashSet<(i32, u16)> = HashSet::new();
    let mut result = Vec::new();

    for line in output.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        // State Recv-Q Send-Q Local Peer [Process]
        if fields.first() != Some(&"LISTEN") || fields.len() < 4 {
            continue;
        }
        let Some(port) = port_from_addr(fields[3]) else {
            continue;
        };
        let (pid, command) = fields
            .get(5)
            .and_then(|p| process_info(p))
            .unwrap_or((0, String::new()));

        if seen.insert((pid, port)) {
            result.push(RawListen { pid, command, port });
        }
    }
    result
}

/// Extract the port from an ss local address such as `*:6379`, `0.0.0.0:3000`,
/// `127.0.0.1:3000`, `[::]:3000`, `[::1]:3000`.
fn port_from_addr(addr: &str) -> Option<u16> {
    let tail = addr.rsplit(':').next()?;
    tail.parse::<u16>().ok()
}

/// Pull `(pid, name)` from the first entry of `users:(("name",pid=N,fd=M),...)`.
fn process_info(field: &str) -> Option<(i32, String)> {
    let name_start = field.find("(\"")? + 2;
    let name_end = name_start + field[name_start..].find('"')?;
    let name = field[name_start..name_end].to_string();

    let pid_start = field.find("pid=")? + 4;
    let pid_str: String = field[pid_start..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    let pid = pid_str.parse::<i32>().ok()?;
    Some((pid, name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiple_processes_and_dedups_ipv4_ipv6() {
        let output = "\
LISTEN 0 511 0.0.0.0:3000 0.0.0.0:* users:((\"node\",pid=1234,fd=20))
LISTEN 0 511 [::]:3000 [::]:* users:((\"node\",pid=1234,fd=21))
LISTEN 0 511 127.0.0.1:6379 0.0.0.0:* users:((\"redis-ser\",pid=5678,fd=6))";
        assert_eq!(
            parse_listens(output),
            vec![
                RawListen { pid: 1234, command: "node".into(), port: 3000 },
                RawListen { pid: 5678, command: "redis-ser".into(), port: 6379 },
            ]
        );
    }

    #[test]
    fn keeps_socket_without_process_as_bare_port() {
        let output = "LISTEN 0 128 127.0.0.1:631 0.0.0.0:*";
        assert_eq!(
            parse_listens(output),
            vec![RawListen { pid: 0, command: String::new(), port: 631 }]
        );
    }

    #[test]
    fn ignores_malformed_and_empty_input() {
        assert_eq!(parse_listens(""), vec![]);
        assert_eq!(parse_listens("garbage\nESTAB 0 0 x y"), vec![]);
    }
}
