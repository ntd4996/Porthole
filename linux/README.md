# Porthole for Linux

Native Linux port of Porthole, built with **Rust + GTK4 + libadwaita**. Full
feature parity with the macOS app: live dev-port list, project grouping, tunnel
detection (ngrok / Cloudflare / Tailscale / localtunnel), ignore list, actions
(open / copy / kill), i18n (en/vi/zh), and in-app updates for the AppImage.

## Layout

```
porthole-core/   pure detection logic, unit-tested (mirrors macOS PortholeCore)
porthole/        GTK4 tray app (scan runtime + UI)
data/            .desktop, AppStream metainfo, icon
packaging/       flatpak manifest, AppImage build script
```

## How it maps to macOS

| macOS | Linux |
|---|---|
| `lsof -iTCP -sTCP:LISTEN` | `ss -tlnpH` (+ `/proc` fallback) |
| `lsof -d cwd` / `ps` | `/proc/PID/cwd`, `/proc/PID/cmdline` |
| ngrok API, `tailscale serve status`, `~/.cloudflared/config.yml` | same |
| `NSStatusItem` + `NSPopover` | StatusNotifierItem (`ksni`) + GTK window |
| Sparkle | GitHub Releases self-update (AppImage) |
| UserDefaults | `~/.config/porthole/ignore.json` |

## Build from source

Requires a Rust toolchain and GTK4 + libadwaita dev packages.

```bash
# Debian/Ubuntu
sudo apt install libgtk-4-dev libadwaita-1-dev iproute2

cargo test  --manifest-path linux/Cargo.toml      # runs pure-logic tests
cargo run   --manifest-path linux/Cargo.toml -p porthole
```

## Packaging

```bash
# AppImage (bundles GTK)
linux/packaging/appimage/build-appimage.sh

# .deb
cargo install cargo-deb
cargo deb --manifest-path linux/porthole/Cargo.toml

# Flatpak (requests host access; see the manifest header)
flatpak-builder --user --install --force-clean build-dir \
    linux/packaging/flatpak/org.datnt.Porthole.yml
```

## Install via APT

One line adds the signed repo and installs Porthole:

```bash
curl -fsSL https://raw.githubusercontent.com/ntd4996/Porthole/main/linux/setup-apt.sh | sudo bash
```

Or do it by hand:

```bash
curl -fsSL https://porthole.thenightwatcher.online/apt/key.asc | sudo gpg --dearmor -o /usr/share/keyrings/porthole.gpg
echo "deb [signed-by=/usr/share/keyrings/porthole.gpg] https://porthole.thenightwatcher.online/apt stable main" | sudo tee /etc/apt/sources.list.d/porthole.list
sudo apt update && sudo apt install porthole
```

The repo is regenerated and signed by the [`APT repo` workflow](../.github/workflows/apt.yml) on every `linux-v*` release, so `sudo apt upgrade` picks up new versions.

## Notes

- `ss` shows the PID/name only for processes owned by the current user (same as
  `lsof` on macOS). Ports from other users still appear, without a name.
- **GNOME** needs the AppIndicator/StatusNotifierItem extension for the tray icon.
- Under **Flatpak** the app escapes the sandbox with `flatpak-spawn --host` to
  read the host's `/proc` and run `ss`/`tailscale`; that host access makes it a
  poor Flathub candidate as-is.
- No code-signing (not required on Linux the way it is on macOS/Windows).

## Releasing

Tag `linux-vX.Y.Z` and push. The [`Linux` workflow](../.github/workflows/linux.yml)
tests, builds the AppImage + `.deb`, and attaches them to the matching GitHub
release. Bump `version` in `linux/Cargo.toml` first (it feeds the update check).
