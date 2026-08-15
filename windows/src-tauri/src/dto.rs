//! Serializable shapes sent to the web frontend.

use porthole_core::{PortInfo, TunnelInfo};
use serde::Serialize;

#[derive(Serialize)]
pub struct TunnelDTO {
    pub provider: String,
    pub public_url: Option<String>,
    pub target_port: u16,
}

#[derive(Serialize)]
pub struct ProjectDTO {
    pub name: String,
    pub kind: String,
    pub path: String,
}

#[derive(Serialize)]
pub struct PortDTO {
    pub id: String,
    pub port: u16,
    pub pid: i32,
    pub command: String,
    pub display_name: String,
    pub project: Option<ProjectDTO>,
    pub tunnels: Vec<TunnelDTO>,
    pub ignored: bool,
}

#[derive(Serialize)]
pub struct RulesDTO {
    pub processes: Vec<String>,
    pub ports: Vec<u16>,
}

#[derive(Serialize)]
pub struct ScanResult {
    pub ports: Vec<PortDTO>,
    pub rules: RulesDTO,
}

fn tunnel(t: &TunnelInfo) -> TunnelDTO {
    TunnelDTO {
        provider: t.provider.label().to_string(),
        public_url: t.public_url.clone(),
        target_port: t.target_port,
    }
}

pub fn port_dto(p: &PortInfo, ignored: bool) -> PortDTO {
    PortDTO {
        id: p.id(),
        port: p.port,
        pid: p.pid,
        command: p.command.clone(),
        display_name: p.display_name.clone(),
        project: p.project.as_ref().map(|pr| ProjectDTO {
            name: pr.name.clone(),
            kind: pr.kind.label().to_string(),
            path: pr.path.clone(),
        }),
        tunnels: p.tunnels.iter().map(tunnel).collect(),
        ignored,
    }
}
