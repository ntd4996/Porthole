//! Runtime port + tunnel scan. Linux counterpart of `App/ScanService.swift`:
//! shells out to `ss` / `tailscale`, reads `/proc`, hits the ngrok local API,
//! and assembles everything through the pure `porthole-core` logic.

use crate::host;
use porthole_core::{
    cloudflared_parser, localtunnel_parser, ngrok_parser, port_assembler,
    process_name_heuristic, project_resolver, ss_parser, tailscale_parser,
    PortInfo, ProjectInfo, TunnelInfo, TunnelProvider,
};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

/// Perform a full scan. Blocking: call from a background thread.
pub fn scan() -> Vec<PortInfo> {
    let listens = list_listens();
    let pids: HashSet<i32> = listens.iter().map(|l| l.pid).filter(|&p| p > 0).collect();

    let mut projects: HashMap<i32, ProjectInfo> = HashMap::new();
    let mut names: HashMap<i32, String> = HashMap::new();
    for &pid in &pids {
        if let Some(cwd) = host::read_link(&format!("/proc/{pid}/cwd")) {
            if let Some(info) = project_resolver::resolve(&cwd) {
                projects.insert(pid, info);
            }
        }
        if let Some(full) = full_command(pid) {
            let cmd = listens
                .iter()
                .find(|l| l.pid == pid)
                .map(|l| l.command.clone())
                .unwrap_or_else(|| full.clone());
            names.insert(pid, process_name_heuristic::display_name(&cmd, &full));
        }
    }

    let tunnels = scan_tunnels();
    port_assembler::assemble(listens, &projects, &names, &tunnels)
}

fn list_listens() -> Vec<porthole_core::RawListen> {
    match host::run("ss", &["-tlnpH"]) {
        Some(out) => ss_parser::parse_listens(&out),
        None => vec![],
    }
}

/// `/proc/PID/cmdline` is NUL-separated; join into a readable command line.
fn full_command(pid: i32) -> Option<String> {
    let raw = host::read_file(&format!("/proc/{pid}/cmdline"))?;
    let joined = raw
        .split('\0')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let trimmed = joined.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

fn scan_tunnels() -> Vec<TunnelInfo> {
    let mut tunnels = Vec::new();
    if let Some(ngrok) = fetch_ngrok() {
        tunnels.extend(ngrok);
    }
    tunnels.extend(scan_proc_tunnels());
    if let Some(out) = host::run("tailscale", &["serve", "status"]) {
        tunnels.extend(tailscale_parser::parse(&out));
    }
    tunnels.extend(cloudflared_config_tunnels());
    tunnels
}

fn fetch_ngrok() -> Option<Vec<TunnelInfo>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(500))
        .build()
        .ok()?;
    let resp = client.get("http://127.0.0.1:4040/api/tunnels").send().ok()?;
    let bytes = resp.bytes().ok()?;
    Some(ngrok_parser::parse(&bytes))
}

/// Walk every `/proc/<pid>/cmdline` for cloudflared quick tunnels and localtunnel.
fn scan_proc_tunnels() -> Vec<TunnelInfo> {
    let mut result = Vec::new();
    for entry in host::list_dir("/proc") {
        let Ok(pid) = entry.parse::<i32>() else {
            continue;
        };
        let Some(cmd) = full_command(pid) else {
            continue;
        };
        if cmd.contains("cloudflared") {
            if let Some(port) = cloudflared_parser::target_port_from_cmdline(&cmd) {
                result.push(TunnelInfo {
                    provider: TunnelProvider::Cloudflare,
                    public_url: None,
                    target_port: port,
                });
            }
        }
        if is_localtunnel(&cmd) {
            if let Some(lt) = localtunnel_parser::parse_from_cmdline(&cmd) {
                result.push(lt);
            }
        }
    }
    result
}

/// True when the command line invokes the localtunnel CLI (`lt` token or `.../lt` path).
fn is_localtunnel(cmd: &str) -> bool {
    cmd.split(' ').any(|t| t == "lt" || t.ends_with("/lt"))
}

fn cloudflared_config_tunnels() -> Vec<TunnelInfo> {
    let Some(home) = dirs::home_dir() else {
        return vec![];
    };
    let path = home.join(".cloudflared/config.yml");
    match host::read_file(&path.to_string_lossy()) {
        Some(yaml) => cloudflared_parser::ingress_from_config_yaml(&yaml),
        None => vec![],
    }
}
