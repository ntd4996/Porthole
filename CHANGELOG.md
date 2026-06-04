# Changelog

All notable changes to Porthole are documented here. Format loosely follows
[Keep a Changelog](https://keepachangelog.com/).

## [0.1.0] - 2026-06-04

First public release.

### Added
- Menu bar list of listening TCP dev ports, grouped by the project that owns each port.
- Project resolution from the process working directory (git root / `package.json` /
  `go.mod` / `pyproject.toml` / `requirements.txt` / `Gemfile` / `Cargo.toml`).
- Friendly process names for common dev tools (vite, next, webpack, uvicorn, rails, …).
- Tunnel detection for ngrok, Cloudflare Tunnel, Tailscale, and localtunnel, with public
  URLs shown as clickable pills (best-effort for Cloudflare quick tunnels).
- Per-port quick actions: open in browser, copy URL, kill process.
- Ignore tab: hide ports by process name or port number, seeded with common macOS system
  services; running-ignored ports shown in detail, with a rule manager.
- Native popover UI (NSStatusItem + NSPopover) with open animation; accessory app (no Dock icon).

[0.1.0]: https://github.com/ntd4996/Porthole/releases/tag/v0.1.0
