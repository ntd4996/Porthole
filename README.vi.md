<div align="center">

<img src="assets/icon-1024.png" width="120" alt="Biểu tượng Porthole" />

# Porthole

**Xem cổng dev nào đang chạy, dự án nào sở hữu cổng đó, và tunnel nào đang trỏ vào đâu, ngay trên thanh menu.**

[![CI](https://github.com/ntd4996/Porthole/actions/workflows/ci.yml/badge.svg)](https://github.com/ntd4996/Porthole/actions/workflows/ci.yml)
[![Download](https://img.shields.io/github/v/release/ntd4996/Porthole?label=download&color=0E7490)](https://github.com/ntd4996/Porthole/releases/latest)
[![Platform](https://img.shields.io/badge/macOS-14%2B-blue)](https://github.com/ntd4996/Porthole/releases/latest)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

[English](README.md) · **Tiếng Việt** · [中文](README.zh.md)

<img src="assets/hero.png" width="640" alt="Popover Porthole" />

</div>

## Porthole làm gì

Khi bạn chạy cả tá dev server trên nhiều dự án, gõ `lsof -i` thủ công rất nhanh chán. Porthole giữ một danh sách trực tiếp ngay trên thanh menu:

- **Cổng dev đang chạy** cùng tiến trình đứng sau mỗi cổng (`vite`, `next`, `prisma`, `uvicorn`…).
- **Dự án nào sở hữu cổng**, xác định từ thư mục làm việc của tiến trình (git root / `package.json` / `go.mod` / `pyproject.toml`…), nhóm theo từng dự án.
- **Tunnel đang trỏ vào cổng**, phát hiện từ ngrok, Cloudflare Tunnel, Tailscale và localtunnel, với URL công khai chỉ cách một cú nhấp.
- **Hành động nhanh** cho mỗi cổng: mở `localhost:PORT` trong trình duyệt, sao chép URL, hoặc tắt tiến trình.
- **Danh sách bỏ qua** để ẩn các dịch vụ hệ thống gây nhiễu (ControlCenter, rapportd, …) để bạn chỉ theo dõi cổng dev thật. Có sẵn mặc định hợp lý, chỉnh sửa thoải mái.

## Cài đặt

### Homebrew (khuyến nghị)

```bash
brew install --cask ntd4996/tap/porthole
```

### Tải trực tiếp

Lấy file `.dmg` mới nhất đã ký và notarize từ [trang releases](https://github.com/ntd4996/Porthole/releases/latest), mở nó và kéo Porthole vào Applications.

Porthole nằm trên thanh menu (không có icon Dock). Nhấp vào biểu tượng porthole để mở bảng điều khiển.

## Cách hoạt động

Porthole gọi các công cụ chuẩn và phân tích output của chúng, không cần kernel extension, không cần quyền nâng cao:

- `lsof -nP -iTCP -sTCP:LISTEN` cho socket đang lắng nghe, và `lsof … -d cwd` để tìm thư mục của từng tiến trình.
- ngrok local API (`127.0.0.1:4040`), dòng lệnh `cloudflared` / `lt`, `~/.cloudflared/config.yml`, và `tailscale serve status` cho tunnel. URL công khai là best-effort (URL quick-tunnel của Cloudflare không phải lúc nào cũng có).

Ứng dụng không sandbox (cần thiết để chạy `lsof`/`ps`) và được phát hành dưới dạng bản build Developer ID đã notarize.

## Build từ mã nguồn

```bash
git clone https://github.com/ntd4996/Porthole.git
cd Porthole
swift build
swift test
swift run porthole          # chạy app thanh menu
./scripts/build-app.sh      # tạo build/Porthole.app
```

Yêu cầu macOS 14+ và Swift toolchain mới. Logic phát hiện nằm trong target `PortholeCore` (thuần, có unit test); giao diện thanh menu SwiftUI nằm trong target `porthole`.

## Giấy phép

[MIT](LICENSE) © Dat Nguyen
