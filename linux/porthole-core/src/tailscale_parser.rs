//! Mirror of `PortholeCore/TailscaleParser.swift`. Parses `tailscale serve status`.

use crate::models::{TunnelInfo, TunnelProvider};

pub fn parse(output: &str) -> Vec<TunnelInfo> {
    let mut result = Vec::new();
    let mut current_host: Option<String> = None;

    for raw in output.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("https://") {
            current_host = line.split(' ').next().map(|s| s.to_string());
        } else if line.contains("proxy ") {
            if let (Some(host), Some(port)) = (current_host.as_ref(), proxy_port(line)) {
                result.push(TunnelInfo {
                    provider: TunnelProvider::Tailscale,
                    public_url: Some(host.clone()),
                    target_port: port,
                });
            }
        }
    }
    result
}

fn proxy_port(line: &str) -> Option<u16> {
    let idx = line.find("proxy ")? + "proxy ".len();
    let target = &line[idx..];
    let colon = target.rfind(':')?;
    let tail: String = target[colon + 1..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    tail.parse::<u16>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_serve_status() {
        let output = "\
https://machine.tail1234.ts.net (tailnet only)
|-- / proxy http://127.0.0.1:3000

https://machine.tail1234.ts.net:8443 (Funnel on)
|-- / proxy http://127.0.0.1:8080";
        assert_eq!(
            parse(output),
            vec![
                TunnelInfo {
                    provider: TunnelProvider::Tailscale,
                    public_url: Some("https://machine.tail1234.ts.net".into()),
                    target_port: 3000,
                },
                TunnelInfo {
                    provider: TunnelProvider::Tailscale,
                    public_url: Some("https://machine.tail1234.ts.net:8443".into()),
                    target_port: 8080,
                },
            ]
        );
    }

    #[test]
    fn handles_no_serve() {
        assert_eq!(parse("No serve config\n"), vec![]);
    }
}
