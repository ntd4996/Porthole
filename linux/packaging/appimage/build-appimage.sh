#!/usr/bin/env bash
# Build a self-contained Porthole AppImage (bundles GTK4 + libadwaita).
#
# Prereqs (x86_64 Linux):
#   - Rust toolchain, GTK4 + libadwaita dev packages installed
#   - linuxdeploy + linuxdeploy-plugin-gtk (downloaded below if missing)
#
# Usage: linux/packaging/appimage/build-appimage.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
LINUX="$ROOT/linux"
WORK="$LINUX/target/appimage"
APPDIR="$WORK/Porthole.AppDir"
TOOLS="$WORK/tools"

echo "==> Building release binary"
cargo build --release --manifest-path "$LINUX/Cargo.toml" -p porthole

echo "==> Staging AppDir"
rm -rf "$APPDIR"
install -Dm755 "$LINUX/target/release/porthole" "$APPDIR/usr/bin/porthole"
install -Dm644 "$LINUX/data/org.datnt.Porthole.desktop" \
    "$APPDIR/usr/share/applications/org.datnt.Porthole.desktop"
install -Dm644 "$LINUX/data/org.datnt.Porthole.metainfo.xml" \
    "$APPDIR/usr/share/metainfo/org.datnt.Porthole.metainfo.xml"
install -Dm644 "$LINUX/data/icons/hicolor/scalable/apps/org.datnt.Porthole.svg" \
    "$APPDIR/usr/share/icons/hicolor/scalable/apps/org.datnt.Porthole.svg"

echo "==> Fetching linuxdeploy (if needed)"
mkdir -p "$TOOLS"
fetch() { # url dest
    [ -x "$TOOLS/$2" ] || { curl -fL "$1" -o "$TOOLS/$2"; chmod +x "$TOOLS/$2"; }
}
fetch "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage" linuxdeploy
fetch "https://raw.githubusercontent.com/linuxdeploy/linuxdeploy-plugin-gtk/master/linuxdeploy-plugin-gtk.sh" linuxdeploy-plugin-gtk

echo "==> Packing AppImage"
export OUTPUT="$LINUX/target/Porthole-x86_64.AppImage"
cd "$WORK"
"$TOOLS/linuxdeploy" \
    --appdir "$APPDIR" \
    --plugin gtk \
    --desktop-file "$APPDIR/usr/share/applications/org.datnt.Porthole.desktop" \
    --icon-file "$APPDIR/usr/share/icons/hicolor/scalable/apps/org.datnt.Porthole.svg" \
    --output appimage

echo "==> Done: $OUTPUT"
