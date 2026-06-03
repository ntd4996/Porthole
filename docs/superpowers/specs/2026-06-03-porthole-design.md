# Porthole — Design Spec

Ngày: 2026-06-03
Trạng thái: Approved (chờ review spec trước khi viết plan)

## Tóm tắt

Porthole là app menu bar macOS native: phát hiện các dev port đang LISTEN, suy ra dự án nào
đang chiếm port, và phát hiện tunnel (Cloudflare/ngrok/Tailscale/localtunnel) đang trỏ public
URL vào port nào. Kèm action nhanh: mở localhost trong browser, copy URL/port, kill process.

Tên: **Porthole** (cửa sổ tròn trên tàu + pun "port").

## Mục tiêu v1

- Liệt kê mọi TCP port đang LISTEN của user hiện tại, kèm process (command + tên hiển thị suy đoán).
- Map port -> dự án qua cwd của process (git root / package.json / marker file).
- Map tunnel -> port (best-effort URL).
- Action nhanh per-port: mở browser, copy, kill (có confirm).

Ngoài phạm vi v1: poll nền + badge số, mở project trong editor, hỗ trợ UDP, đa-user/root scan.

## Stack & ràng buộc

- SwiftUI menu bar app, `MenuBarExtra` style `.window`, min macOS 13.
- SPM (`Package.swift`) giống AgentPet.
- **Non-sandboxed**, Developer ID notarized, Sparkle auto-update (giống AgentPet/PlugTalk).
  Non-sandbox là bắt buộc để spawn `Process` (lsof/ps/tailscale).
- Approach engine: **A** — shell ra `lsof`/`ps` + CLI/API tunnel (không dùng libproc/sysctl ở v1).

## Kiến trúc

```
PortholeApp (MenuBarExtra)
   └─ ScanCoordinator (timer + refresh-on-open)
        ├─ PortScanner      lsof -> [PortInfo]
        ├─ ProjectResolver  pid cwd -> project root + name (cache theo PID)
        └─ TunnelDetector   ngrok API + process scan + tailscale -> [TunnelInfo]
   └─ AppState (@Observable) -> ContentView (list UI)
```

Mỗi detector độc lập; lỗi một cái không kéo cả scan.

## Data model

```swift
struct PortInfo {
    let port: Int
    let pid: Int32
    let command: String        // "node", "python3.12", "ruby"...
    let displayName: String    // "vite", "next dev" suy từ cmdline nếu được
    var project: ProjectInfo?
    var tunnels: [TunnelInfo]  // join theo target port
}

struct ProjectInfo {
    let path: String           // git root hoặc thư mục chứa package.json
    let name: String           // package.json "name" || basename(path)
    let kind: ProjectKind      // node / python / go / ruby / unknown
}

struct TunnelInfo {
    let provider: TunnelProvider  // cloudflare / ngrok / tailscale / localtunnel
    let publicURL: String?        // nil nếu không lấy được (vd cloudflared quick-tunnel)
    let targetPort: Int
}
```

UI nhóm theo project; port không gắn project gom vào "Other".

## Detection chi tiết

### PortScanner
- `lsof -nP -iTCP -sTCP:LISTEN -F pcnPt` (machine-readable, field-per-line).
- Parse `p`=pid, `c`=command, `n`=addr:port. Gom theo (pid, port), khử trùng socket.
- Mặc định hiện tất cả LISTEN của user hiện tại.

### ProjectResolver (per PID, cache)
- `lsof -a -p PID -d cwd -F n` -> cwd process.
- Walk up từ cwd tìm marker: `.git` (root), `package.json` (đọc `name`), `go.mod`,
  `pyproject.toml`/`requirements.txt`, `Gemfile` -> name + kind.
- Cache theo PID (process sống thì cwd cố định).
- `displayName`: `ps -p PID -o command=`, heuristic match vite/next/webpack/rails/flask/uvicorn...

### TunnelDetector (song song, best-effort)
- **ngrok**: GET `http://127.0.0.1:4040/api/tunnels` -> `public_url` + `config.addr` (suy port). URL đầy đủ.
- **cloudflared**: scan process `cloudflared`, parse cmdline `--url http://localhost:PORT` -> target port.
  Quick-tunnel URL (`*.trycloudflare.com`) best-effort đọc log mặc định; không có thì URL = nil.
  Named tunnel: đọc `~/.cloudflared/config.yml` ingress nếu có. **Chấp nhận best-effort.**
- **tailscale**: `tailscale serve status` / `funnel status` -> mapping URL->port.
- **localtunnel**: process `lt --port PORT` (+ `--subdomain`) -> suy URL.
- Join vào PortInfo theo `targetPort`.

## UI

`MenuBarExtra` style `.window`, panel ~320pt. Icon porthole (vòng tròn), tạm SF Symbol.

```
┌─────────────────────────────┐
│ Porthole          ⟳  ⚙︎      │
├─────────────────────────────┤
│ ▾ roomify  (node)            │
│    :3000  next dev   🌐 ↗ ⧉ ✕ │
│      └ ngrok https://ab.ngrok.io
│    :5555  prisma     ↗ ⧉ ✕   │
│ ▾ rophim  (node)             │
│    :5173  vite       ↗ ⧉ ✕   │
│ ▾ Other                      │
│    :6379  redis-server   ✕   │
├─────────────────────────────┤
│ 12 ports · 2 tunnels    Quit │
└─────────────────────────────┘
```

Actions per-port: 🌐 có tunnel (URL dòng dưới), ↗ mở `http://localhost:PORT`, ⧉ copy URL/port,
✕ kill (`kill PID`, confirm). Empty state: "Không có dev port nào đang chạy".

## Refresh

- Refresh ngay khi menu mở (`isPresented` binding).
- Menu mở: poll lại mỗi **4s**.
- Menu đóng: dừng timer, không poll nền, không badge.
- Scan off-main (async), cập nhật AppState trên main, diff nhẹ tránh nhấp nháy.

## Error handling

- lsof/ps fail/thiếu -> trạng thái lỗi nhẹ ở footer, không crash.
- ngrok API timeout 500ms -> bỏ qua.
- Detector độc lập, lỗi cục bộ không lan.
- Kill fail (process user khác/đã chết) -> toast nhỏ.

## Testing

- Unit test parser: feed output mẫu `lsof -F`, `ps`, ngrok JSON, `tailscale serve status` -> assert model.
- ProjectResolver: thư mục tạm có `package.json`/`.git` -> assert name/kind.

## Repo

GitHub private: https://github.com/ntd4996/Porthole (remote `origin`, SSH).
