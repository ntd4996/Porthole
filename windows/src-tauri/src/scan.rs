//! Windows port + tunnel scan. Replaces the Linux `ss`/`/proc` layer; everything
//! downstream reuses the platform-agnostic `porthole-core` crate.

use porthole_core::{
    cloudflared_parser, localtunnel_parser, ngrok_parser, port_assembler,
    process_name_heuristic, tailscale_parser, PortInfo, ProjectInfo, RawListen, TunnelInfo,
    TunnelProvider,
};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::time::Duration;

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

struct Proc {
    name: String,
    cmdline: Option<String>,
    exepath: Option<String>,
}

pub fn scan() -> Vec<PortInfo> {
    let Some(root) = query_windows() else {
        return vec![];
    };

    // pid -> process details
    let mut procs: HashMap<i32, Proc> = HashMap::new();
    if let Some(arr) = root.get("procs").and_then(|v| v.as_array()) {
        for p in arr {
            let Some(pid) = p.get("ProcessId").and_then(|v| v.as_i64()) else { continue };
            procs.insert(
                pid as i32,
                Proc {
                    name: p.get("Name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    cmdline: p.get("CommandLine").and_then(|v| v.as_str()).map(str::to_string),
                    exepath: p.get("ExecutablePath").and_then(|v| v.as_str()).map(str::to_string),
                },
            );
        }
    }

    // Listening sockets -> RawListen (dedup on pid+port).
    let mut seen: HashSet<(i32, u16)> = HashSet::new();
    let mut listens: Vec<RawListen> = Vec::new();
    if let Some(arr) = root.get("conns").and_then(|v| v.as_array()) {
        for c in arr {
            let Some(port) = c.get("LocalPort").and_then(|v| v.as_u64()) else { continue };
            let pid = c.get("OwningProcess").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let port = port as u16;
            if !seen.insert((pid, port)) {
                continue;
            }
            let command = procs.get(&pid).map(|p| trim_exe(&p.name)).unwrap_or_default();
            listens.push(RawListen { pid, command, port });
        }
    }

    // Enrich: project + display name.
    let pids: HashSet<i32> = listens.iter().map(|l| l.pid).filter(|&p| p > 0).collect();
    let mut projects: HashMap<i32, ProjectInfo> = HashMap::new();
    let mut names: HashMap<i32, String> = HashMap::new();
    for &pid in &pids {
        if let Some(p) = procs.get(&pid) {
            if let Some(info) =
                crate::project_win::resolve(p.cmdline.as_deref(), p.exepath.as_deref())
            {
                projects.insert(pid, info);
            }
            let cmd = trim_exe(&p.name);
            let full = p.cmdline.clone().unwrap_or_else(|| cmd.clone());
            names.insert(pid, process_name_heuristic::display_name(&cmd, &full));
        }
    }

    let tunnels = scan_tunnels(&procs);
    port_assembler::assemble(listens, &projects, &names, &tunnels)
}

/// One PowerShell call returning `{conns:[...], procs:[...]}` as JSON.
fn query_windows() -> Option<Value> {
    let script = "$ErrorActionPreference='SilentlyContinue';\
        $c=@(Get-NetTCPConnection -State Listen | Select-Object LocalPort,OwningProcess);\
        $p=@(Get-CimInstance Win32_Process | Select-Object ProcessId,Name,CommandLine,ExecutablePath);\
        [pscustomobject]@{conns=$c;procs=$p} | ConvertTo-Json -Depth 3 -Compress";
    let out = powershell(script)?;
    serde_json::from_str(&out).ok()
}

fn scan_tunnels(procs: &HashMap<i32, Proc>) -> Vec<TunnelInfo> {
    let mut tunnels: Vec<TunnelInfo> = Vec::new();

    if let Some(ngrok) = fetch_ngrok() {
        tunnels.extend(ngrok);
    }

    for p in procs.values() {
        let Some(cmd) = p.cmdline.as_deref() else { continue };
        if cmd.contains("cloudflared") {
            if let Some(port) = cloudflared_parser::target_port_from_cmdline(cmd) {
                tunnels.push(TunnelInfo {
                    provider: TunnelProvider::Cloudflare,
                    public_url: None,
                    target_port: port,
                });
            }
        }
        if is_localtunnel(cmd) {
            if let Some(lt) = localtunnel_parser::parse_from_cmdline(cmd) {
                tunnels.push(lt);
            }
        }
    }

    if let Some(out) = powershell_bin("tailscale", &["serve", "status"]) {
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

fn cloudflared_config_tunnels() -> Vec<TunnelInfo> {
    let Some(home) = dirs::home_dir() else { return vec![] };
    let path = home.join(".cloudflared").join("config.yml");
    match std::fs::read_to_string(&path) {
        Ok(yaml) => cloudflared_parser::ingress_from_config_yaml(&yaml),
        Err(_) => vec![],
    }
}

fn is_localtunnel(cmd: &str) -> bool {
    cmd.split_whitespace().any(|t| t == "lt" || t == "lt.cmd" || t.ends_with("\\lt") || t.ends_with("/lt"))
}

/// Strip a trailing `.exe`/`.EXE` so display names match the Linux/macOS command.
fn trim_exe(name: &str) -> String {
    let lower = name.to_lowercase();
    if let Some(stripped) = lower.strip_suffix(".exe") {
        name[..stripped.len()].to_string()
    } else {
        name.to_string()
    }
}

fn powershell(script: &str) -> Option<String> {
    let out = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", script])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    if text.trim().is_empty() {
        None
    } else {
        Some(text)
    }
}

fn powershell_bin(program: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(program)
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    if text.trim().is_empty() && !out.status.success() {
        None
    } else {
        Some(text)
    }
}
