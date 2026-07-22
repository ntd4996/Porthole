//! Data types shared across the detection logic. Mirrors `PortholeCore/Models.swift`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectKind {
    Node,
    Python,
    Go,
    Ruby,
    Rust,
    Unknown,
}

impl ProjectKind {
    /// Lowercase tag used as a badge label (matches Swift `rawValue`, but Title-cased in UI).
    pub fn label(self) -> &'static str {
        match self {
            ProjectKind::Node => "node",
            ProjectKind::Python => "python",
            ProjectKind::Go => "go",
            ProjectKind::Ruby => "ruby",
            ProjectKind::Rust => "rust",
            ProjectKind::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TunnelProvider {
    Cloudflare,
    Ngrok,
    Tailscale,
    Localtunnel,
}

impl TunnelProvider {
    pub fn label(self) -> &'static str {
        match self {
            TunnelProvider::Cloudflare => "cloudflare",
            TunnelProvider::Ngrok => "ngrok",
            TunnelProvider::Tailscale => "tailscale",
            TunnelProvider::Localtunnel => "localtunnel",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectInfo {
    pub path: String,
    pub name: String,
    pub kind: ProjectKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TunnelInfo {
    pub provider: TunnelProvider,
    pub public_url: Option<String>,
    pub target_port: u16,
}

/// One LISTEN socket as parsed from `ss`, before enrichment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawListen {
    pub pid: i32,
    pub command: String,
    pub port: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortInfo {
    pub port: u16,
    pub pid: i32,
    pub command: String,
    pub display_name: String,
    pub project: Option<ProjectInfo>,
    pub tunnels: Vec<TunnelInfo>,
}

impl PortInfo {
    pub fn id(&self) -> String {
        format!("{}-{}", self.pid, self.port)
    }
}
