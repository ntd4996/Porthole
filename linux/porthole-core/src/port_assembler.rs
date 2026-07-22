//! Mirror of `PortholeCore/PortAssembler.swift`.

use crate::models::{PortInfo, ProjectInfo, RawListen, TunnelInfo};
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn assemble(
    listens: Vec<RawListen>,
    projects: &HashMap<i32, ProjectInfo>,
    display_names: &HashMap<i32, String>,
    tunnels: &[TunnelInfo],
) -> Vec<PortInfo> {
    let mut tunnels_by_port: HashMap<u16, Vec<TunnelInfo>> = HashMap::new();
    for t in tunnels {
        tunnels_by_port.entry(t.target_port).or_default().push(t.clone());
    }

    let mut ports: Vec<PortInfo> = listens
        .into_iter()
        .map(|listen| PortInfo {
            port: listen.port,
            pid: listen.pid,
            display_name: display_names
                .get(&listen.pid)
                .cloned()
                .unwrap_or_else(|| listen.command.clone()),
            command: listen.command,
            project: projects.get(&listen.pid).cloned(),
            tunnels: tunnels_by_port.get(&listen.port).cloned().unwrap_or_default(),
        })
        .collect();

    ports.sort_by(|a, b| {
        let an = a.project.as_ref().map(|p| &p.name);
        let bn = b.project.as_ref().map(|p| &p.name);
        match (an, bn) {
            (Some(x), Some(y)) if x != y => x.cmp(y),
            (None, Some(_)) => Ordering::Greater, // Other after named projects
            (Some(_), None) => Ordering::Less,
            _ => a.port.cmp(&b.port),
        }
    });
    ports
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ProjectKind, TunnelProvider};

    fn project(name: &str) -> ProjectInfo {
        ProjectInfo { path: format!("/p/{name}"), name: name.into(), kind: ProjectKind::Node }
    }

    #[test]
    fn assembles_and_attaches_tunnels_by_port() {
        let listens = vec![
            RawListen { pid: 10, command: "node".into(), port: 3000 },
            RawListen { pid: 20, command: "redis-server".into(), port: 6379 },
        ];
        let mut projects = HashMap::new();
        projects.insert(10, project("roomify"));
        let mut names = HashMap::new();
        names.insert(10, "vite".to_string());
        names.insert(20, "redis-server".to_string());
        let tunnels = vec![TunnelInfo {
            provider: TunnelProvider::Ngrok,
            public_url: Some("https://a.ngrok.io".into()),
            target_port: 3000,
        }];

        let result = assemble(listens, &projects, &names, &tunnels);
        assert_eq!(result.len(), 2);
        let p3000 = result.iter().find(|p| p.port == 3000).unwrap();
        assert_eq!(p3000.display_name, "vite");
        assert_eq!(p3000.project.as_ref().unwrap().name, "roomify");
        assert_eq!(p3000.tunnels, tunnels);
        let p6379 = result.iter().find(|p| p.port == 6379).unwrap();
        assert!(p6379.project.is_none());
        assert!(p6379.tunnels.is_empty());
    }

    #[test]
    fn sorts_projects_then_other_last() {
        let listens = vec![
            RawListen { pid: 20, command: "redis-server".into(), port: 6379 },
            RawListen { pid: 10, command: "node".into(), port: 3000 },
        ];
        let mut projects = HashMap::new();
        projects.insert(10, project("roomify"));
        let result = assemble(listens, &projects, &HashMap::new(), &[]);
        assert_eq!(result.iter().map(|p| p.port).collect::<Vec<_>>(), vec![3000, 6379]);
    }
}
