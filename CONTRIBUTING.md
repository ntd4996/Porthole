# Contributing

Thanks for your interest in Porthole.

## Development

```bash
swift build
swift test
swift run porthole
```

The codebase is split in two:

- **`Sources/PortholeCore`**: pure logic with no UI or process side effects, the lsof / ps /
  ngrok / cloudflared / tailscale / localtunnel parsers, project resolution, ignore rules, and
  the assembler that joins it all into `PortInfo`. Everything here is unit-tested in
  `Tests/PortholeCoreTests`.
- **`Sources/App`**: the SwiftUI menu bar UI plus the thin layer that actually shells out
  (`CommandRunner`, `ScanService`).

When adding detection for a new tunnel provider or process type, put the parsing in
`PortholeCore` with a test that feeds it sample tool output, then wire it into `ScanService`.

## Pull requests

- Keep changes focused and match the surrounding style.
- Add or update tests for any logic change (`swift test` must pass).
- Describe what you changed and why.

## Reporting bugs

Open an issue with your macOS version, what you expected, and what happened. For detection
issues, the raw output of the relevant command (e.g. `lsof -nP -iTCP -sTCP:LISTEN`) helps a lot.
