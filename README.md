# Porthole

macOS menu bar app: see which dev ports are running, which project each belongs to,
and which tunnels (Cloudflare, ngrok, Tailscale, localtunnel) point to which port.

## Develop

    swift build
    swift test
    swift run porthole

## How it works

- Lists LISTEN sockets via `lsof`, resolves each process's project from its cwd.
- Detects tunnels from the ngrok local API, the cloudflared/localtunnel command lines,
  `~/.cloudflared/config.yml`, and `tailscale serve status`. Public URLs are best-effort
  (cloudflared quick-tunnel URLs may not be available).

Requires macOS 14+. Non-sandboxed (needs to spawn `lsof`/`ps`). Distributed as a notarized
Developer ID build.
