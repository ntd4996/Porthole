# Changelog

All notable changes to Porthole are documented here. Format loosely follows
[Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

### Added
- Localization in English, Vietnamese (vi), and Simplified Chinese (zh-Hans).
  The app picks the language from the system, the landing page from the
  browser locale (with an EN/VI/中 switcher), and the README ships in all three.

## [1.0.2] - 2026-06-06

### Fixed
- Sparkle no longer offers the same release over and over. The bundle's
  `CFBundleVersion` was hardcoded to `1` while the appcast advertised the full
  version, so Sparkle always thought a newer build was available even right after
  updating. `CFBundleVersion` now matches the appcast version.

## [1.0.1] - 2026-06-05

### Added
- Right-click the menu bar icon for a quick menu (Open, Refresh, Check for Updates, Quit).
- A loading state while the first scan runs, so an empty list no longer looks like
  "no ports running" before the scan finishes.

### Changed
- Icon buttons now have a hover highlight and a press animation (with tooltips), so
  it's clear they're clickable and that a click registered.
- "Check for Updates" moved into the popover footer (always visible).
- All in-app text is English.

## [1.0.0] - 2026-06-04

First stable release.

### Added
- Sparkle auto-update for the direct `.dmg` build: a "Check for Updates…" item in
  the menu plus automatic background checks against the appcast. Homebrew installs
  keep updating through `brew upgrade`. Universal binary (Apple Silicon + Intel).

## [0.1.1] - 2026-06-04

### Fixed
- Popover now closes when you click anywhere outside it (a global mouse monitor,
  the same approach AgentPet uses, since a transient popover can miss outside
  clicks for a non-activating menu bar app).

### Changed
- App icon now follows the macOS icon grid (rounded squircle with margin) so it
  sits correctly among other apps instead of filling the tile edge to edge.
- DMG opens to a styled "drag Porthole to Applications" window with a background.

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

[1.0.2]: https://github.com/ntd4996/Porthole/releases/tag/v1.0.2
[1.0.1]: https://github.com/ntd4996/Porthole/releases/tag/v1.0.1
[1.0.0]: https://github.com/ntd4996/Porthole/releases/tag/v1.0.0
[0.1.1]: https://github.com/ntd4996/Porthole/releases/tag/v0.1.1
[0.1.0]: https://github.com/ntd4996/Porthole/releases/tag/v0.1.0
