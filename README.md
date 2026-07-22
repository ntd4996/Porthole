<div align="center">

<img src="assets/icon-1024.png" width="120" alt="Porthole icon" />

# Porthole

**See which dev ports are running, which project owns each one, and which tunnels point where, right from your menu bar.**

[![CI](https://github.com/ntd4996/Porthole/actions/workflows/ci.yml/badge.svg)](https://github.com/ntd4996/Porthole/actions/workflows/ci.yml)
[![Download](https://img.shields.io/github/v/release/ntd4996/Porthole?label=download&color=0E7490)](https://github.com/ntd4996/Porthole/releases/latest)
[![Platform](https://img.shields.io/badge/macOS-14%2B-blue)](https://github.com/ntd4996/Porthole/releases/latest)
[![Linux](https://img.shields.io/badge/Linux-GTK4-orange)](https://github.com/ntd4996/Porthole/releases)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

**English** · [Tiếng Việt](README.vi.md) · [中文](README.zh.md)

<img src="assets/hero.png" width="640" alt="Porthole popover" />

</div>

## What it does

When you run a dozen dev servers across projects, `lsof -i` gets old fast. Porthole keeps a live list in your menu bar:

- **Running dev ports** with the process behind each one (`vite`, `next`, `prisma`, `uvicorn`…).
- **Which project owns the port**, resolved from the process working directory (git root / `package.json` / `go.mod` / `pyproject.toml`…), grouped per project.
- **Tunnels pointing at a port**, detected from ngrok, Cloudflare Tunnel, Tailscale, and localtunnel, with the public URL one click away.
- **Quick actions** per port: open `localhost:PORT` in the browser, copy the URL, or kill the process.
- **Ignore list** to hide noisy system services (ControlCenter, rapportd, …) so you only watch real dev ports. Seeded with sensible defaults; fully editable.

## Install

### Homebrew (recommended)

```bash
brew install --cask ntd4996/tap/porthole
```

### Direct download

Grab the latest signed & notarized `.dmg` from the [releases page](https://github.com/ntd4996/Porthole/releases/latest), open it, and drag Porthole to Applications.

Porthole lives in the menu bar (no Dock icon). Click the porthole icon to open the panel.

### Linux

A native Linux port (Rust + GTK4/libadwaita) lives in [`linux/`](linux/). Grab a build from the [releases page](https://github.com/ntd4996/Porthole/releases) (tags `linux-v*`):

```bash
# AppImage (any distro): make it executable and run
chmod +x Porthole-x86_64.AppImage
./Porthole-x86_64.AppImage

# Debian / Ubuntu / Mint
sudo apt install ./porthole_*.deb
```

Porthole shows up as a system-tray icon. On GNOME you need the [AppIndicator/StatusNotifierItem extension](https://extensions.gnome.org/extension/615/appindicator-support/) for the tray icon to appear (KDE and most other desktops support it out of the box). See [`linux/README.md`](linux/README.md) to build from source or a Flatpak.

## How it works

Porthole shells out to standard tools and parses their output, no kernel extensions, no elevated privileges:

- `lsof -nP -iTCP -sTCP:LISTEN` for listening sockets, and `lsof … -d cwd` to find each process's directory.
- The ngrok local API (`127.0.0.1:4040`), the `cloudflared` / `lt` command lines, `~/.cloudflared/config.yml`, and `tailscale serve status` for tunnels. Public URLs are best-effort (Cloudflare quick-tunnel URLs are not always available).

It is non-sandboxed (required to spawn `lsof`/`ps`) and distributed as a notarized Developer ID build.

## Build from source

```bash
git clone https://github.com/ntd4996/Porthole.git
cd Porthole
swift build
swift test
swift run porthole          # run the menu bar app
./scripts/build-app.sh      # produce build/Porthole.app
```

Requires macOS 14+ and a recent Swift toolchain. The detection logic lives in the `PortholeCore` target (pure, unit-tested); the SwiftUI menu bar UI lives in the `porthole` app target.

## License

[MIT](LICENSE) © Dat Nguyen
