//! Mirror of `PortholeCore/LocaltunnelParser.swift`.

use crate::models::{TunnelInfo, TunnelProvider};

pub fn parse_from_cmdline(cmd: &str) -> Option<TunnelInfo> {
    let tokens: Vec<&str> = cmd.split(' ').collect();
    let flag = |name: &str| -> Option<String> {
        let i = tokens.iter().position(|&t| t == name)?;
        tokens.get(i + 1).map(|s| s.to_string())
    };
    let port_str = flag("--port").or_else(|| flag("-p"))?;
    let port = port_str.parse::<u16>().ok()?;
    let url = flag("--subdomain").map(|s| format!("https://{s}.loca.lt"));
    Some(TunnelInfo {
        provider: TunnelProvider::Localtunnel,
        public_url: url,
        target_port: port,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_port_and_subdomain() {
        assert_eq!(
            parse_from_cmdline("node lt --port 3000 --subdomain foo"),
            Some(TunnelInfo {
                provider: TunnelProvider::Localtunnel,
                public_url: Some("https://foo.loca.lt".into()),
                target_port: 3000,
            })
        );
    }

    #[test]
    fn parses_port_only() {
        assert_eq!(
            parse_from_cmdline("lt --port 8080"),
            Some(TunnelInfo {
                provider: TunnelProvider::Localtunnel,
                public_url: None,
                target_port: 8080,
            })
        );
    }

    #[test]
    fn returns_none_without_port() {
        assert_eq!(parse_from_cmdline("lt --help"), None);
    }
}
