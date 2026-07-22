#!/usr/bin/env bash
# Install Porthole (Linux AppImage) without cloning the repo.
#
#   curl -fsSL https://raw.githubusercontent.com/ntd4996/Porthole/main/linux/install.sh | bash
#
# Downloads the latest `linux-v*` AppImage into ~/.local/bin and registers a
# desktop entry so it shows up in your app menu.
set -euo pipefail

REPO="ntd4996/Porthole"
BIN_DIR="${XDG_BIN_HOME:-$HOME/.local/bin}"
DATA_DIR="${XDG_DATA_HOME:-$HOME/.local/share}"
APP_DIR="$DATA_DIR/applications"
ICON_DIR="$DATA_DIR/icons/hicolor/scalable/apps"

echo "==> Finding latest Linux release"
TAG="$(curl -fsSL "https://api.github.com/repos/$REPO/releases" \
  | grep -oE '"tag_name": *"linux-v[^"]+"' | head -1 \
  | sed -E 's/.*"(linux-v[^"]+)"/\1/')"
[ -n "${TAG:-}" ] || { echo "error: no linux-v* release found" >&2; exit 1; }

URL="https://github.com/$REPO/releases/download/$TAG/Porthole-x86_64.AppImage"
echo "==> Downloading $TAG"
mkdir -p "$BIN_DIR" "$APP_DIR" "$ICON_DIR"
curl -fL "$URL" -o "$BIN_DIR/porthole.AppImage"
chmod +x "$BIN_DIR/porthole.AppImage"

curl -fsSL "https://raw.githubusercontent.com/$REPO/main/linux/data/icons/hicolor/scalable/apps/org.datnt.Porthole.svg" \
  -o "$ICON_DIR/org.datnt.Porthole.svg" 2>/dev/null || true

cat > "$APP_DIR/org.datnt.Porthole.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Porthole
Comment=See which dev ports are running, and which tunnels point where
Exec=$BIN_DIR/porthole.AppImage
Icon=org.datnt.Porthole
Categories=Utility;Development;Network;
Terminal=false
EOF

echo "==> Installed: $BIN_DIR/porthole.AppImage ($TAG)"
case ":$PATH:" in
  *":$BIN_DIR:"*) ;;
  *) echo "note: add $BIN_DIR to your PATH to run 'porthole.AppImage' directly" ;;
esac
echo "Launch 'Porthole' from your app menu, or run: $BIN_DIR/porthole.AppImage"
echo "GNOME users: enable the AppIndicator/StatusNotifierItem extension to see the tray icon."
