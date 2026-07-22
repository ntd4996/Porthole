//! Mirror of `PortholeCore/CloudflaredParser.swift`.

use crate::models::{TunnelInfo, TunnelProvider};

/// Extract the target port from a `--url <addr>` flag, if present.
pub fn target_port_from_cmdline(cmd: &str) -> Option<u16> {
    let tokens: Vec<&str> = cmd.split(' ').collect();
    let i = tokens.iter().position(|&t| t == "--url")?;
    let arg = tokens.get(i + 1)?;
    port_from_url_like(arg)
}

/// Parse named-tunnel ingress rules (hostname -> localhost:port) from config.yml.
pub fn ingress_from_config_yaml(yaml: &str) -> Vec<TunnelInfo> {
    let mut result = Vec::new();
    let mut pending_host: Option<String> = None;

    for raw in yaml.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(host) = value(line, "- hostname").or_else(|| value(line, "hostname")) {
            pending_host = Some(host);
        } else if let Some(service) = value(line, "service") {
            if let (Some(host), Some(port)) = (pending_host.take(), port_from_url_like(&service)) {
                result.push(TunnelInfo {
                    provider: TunnelProvider::Cloudflare,
                    public_url: Some(format!("https://{host}")),
                    target_port: port,
                });
            } else {
                pending_host = None;
            }
        }
    }
    result
}

fn value(line: &str, key: &str) -> Option<String> {
    let rest = line.strip_prefix(key)?;
    let rest = rest.strip_prefix(':')?;
    let v = rest.trim();
    if v.is_empty() {
        None
    } else {
        Some(v.to_string())
    }
}

fn port_from_url_like(s: &str) -> Option<u16> {
    let colon = s.rfind(':')?;
    let tail: String = s[colon + 1..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    tail.parse::<u16>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_port_from_command_line() {
        assert_eq!(
            target_port_from_cmdline("cloudflared tunnel --url http://localhost:3000"),
            Some(3000)
        );
        assert_eq!(
            target_port_from_cmdline("cloudflared tunnel --url 127.0.0.1:8787 run"),
            Some(8787)
        );
        assert_eq!(target_port_from_cmdline("cloudflared tunnel run mytun"), None);
    }

    #[test]
    fn ingress_from_config() {
        let yaml = "\
tunnel: abc-123
ingress:
  - hostname: app.example.com
    service: http://localhost:3000
  - hostname: api.example.com
    service: http://localhost:8080
  - service: http_status:404";
        assert_eq!(
            ingress_from_config_yaml(yaml),
            vec![
                TunnelInfo {
                    provider: TunnelProvider::Cloudflare,
                    public_url: Some("https://app.example.com".into()),
                    target_port: 3000,
                },
                TunnelInfo {
                    provider: TunnelProvider::Cloudflare,
                    public_url: Some("https://api.example.com".into()),
                    target_port: 8080,
                },
            ]
        );
    }
}
