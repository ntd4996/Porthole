//! Pure port/tunnel detection logic for Porthole (Linux port).
//! Each module mirrors the corresponding file in the macOS `PortholeCore` target
//! and carries the same unit tests, so behaviour stays in lock-step.

pub mod cloudflared_parser;
pub mod ignore_rules;
pub mod localtunnel_parser;
pub mod models;
pub mod ngrok_parser;
pub mod port_assembler;
pub mod process_name_heuristic;
pub mod project_resolver;
pub mod ss_parser;
pub mod tailscale_parser;

pub use ignore_rules::IgnoreRules;
pub use models::*;
