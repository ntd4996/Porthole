//! Mirror of `PortholeCore/NgrokParser.swift`. Parses the ngrok local API
//! (`127.0.0.1:4040/api/tunnels`) JSON body.

use crate::models::{TunnelInfo, TunnelProvider};

pub fn parse(data: &[u8]) -> Vec<TunnelInfo> {
    let Ok(obj) = serde_json::from_slice::<serde_json::Value>(data) else {
        return vec![];
    };
    let Some(tunnels) = obj.get("tunnels").and_then(|t| t.as_array()) else {
        return vec![];
    };
    tunnels
        .iter()
        .filter_map(|tunnel| {
            let addr = tunnel.get("config")?.get("addr")?.as_str()?;
            let port = port_from_addr(addr)?;
            Some(TunnelInfo {
                provider: TunnelProvider::Ngrok,
                public_url: tunnel
                    .get("public_url")
                    .and_then(|u| u.as_str())
                    .map(|s| s.to_string()),
                target_port: port,
            })
        })
        .collect()
}

fn port_from_addr(addr: &str) -> Option<u16> {
    match addr.rfind(':') {
        Some(colon) => addr[colon + 1..].parse::<u16>().ok(),
        None => addr.parse::<u16>().ok(), // bare port, e.g. "3000"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_tunnels() {
        let json = br#"
        {"tunnels":[
          {"public_url":"https://ab12.ngrok.io","config":{"addr":"http://localhost:3000"}},
          {"public_url":"tcp://1.tcp.ngrok.io:24000","config":{"addr":"localhost:5432"}}
        ]}"#;
        assert_eq!(
            parse(json),
            vec![
                TunnelInfo {
                    provider: TunnelProvider::Ngrok,
                    public_url: Some("https://ab12.ngrok.io".into()),
                    target_port: 3000,
                },
                TunnelInfo {
                    provider: TunnelProvider::Ngrok,
                    public_url: Some("tcp://1.tcp.ngrok.io:24000".into()),
                    target_port: 5432,
                },
            ]
        );
    }

    #[test]
    fn parses_bare_port_addr() {
        let json = br#"{"tunnels":[{"public_url":"https://x.ngrok.io","config":{"addr":"3000"}}]}"#;
        assert_eq!(
            parse(json),
            vec![TunnelInfo {
                provider: TunnelProvider::Ngrok,
                public_url: Some("https://x.ngrok.io".into()),
                target_port: 3000,
            }]
        );
    }

    #[test]
    fn handles_empty_and_bad_json() {
        assert_eq!(parse(br#"{"tunnels":[]}"#), vec![]);
        assert_eq!(parse(b"not json"), vec![]);
    }
}
