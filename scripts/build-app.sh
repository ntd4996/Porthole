#!/usr/bin/env bash
# Build Porthole.app (Release) from the SPM executable, with icon + Info.plist.
# Output: build/Porthole.app
set -euo pipefail
cd "$(dirname "$0")/.."

APP_NAME="Porthole"
BUNDLE_ID="com.datnt.porthole"
# CFBundleVersion must match the appcast's sparkle:version (release.sh writes $VERSION),
# otherwise Sparkle keeps offering the same release forever. Keep them equal.
VERSION="${VERSION:-0.1.0}"
BUILD="${BUILD:-$VERSION}"
EXEC="porthole"

echo "==> [1/5] swift build -c release (universal arm64 + x86_64)"
swift build -c release --arch arm64 --arch x86_64
BINDIR="$(swift build -c release --arch arm64 --arch x86_64 --show-bin-path)"
BIN="$BINDIR/$EXEC"

APP="build/$APP_NAME.app"
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"

echo "==> [2/5] Copy binary"
cp "$BIN" "$APP/Contents/MacOS/$APP_NAME"
chmod +x "$APP/Contents/MacOS/$APP_NAME"

echo "==> [3/5] Embed Sparkle.framework"
mkdir -p "$APP/Contents/Frameworks"
ditto "$BINDIR/Sparkle.framework" "$APP/Contents/Frameworks/Sparkle.framework"
install_name_tool -add_rpath "@executable_path/../Frameworks" "$APP/Contents/MacOS/$APP_NAME" 2>/dev/null || true

echo "==> [4/5] Generate AppIcon.icns"
ICONSET="build/AppIcon.iconset"
rm -rf "$ICONSET"; mkdir -p "$ICONSET"
SRC="assets/icon-1024.png"
if [[ ! -f "$SRC" ]]; then
  rsvg-convert -w 1024 -h 1024 assets/icon.svg -o "$SRC"
fi
for s in 16 32 64 128 256 512 1024; do
  sips -z $s $s "$SRC" --out "$ICONSET/icon_${s}x${s}.png" >/dev/null
done
# @2x variants
cp "$ICONSET/icon_32x32.png"   "$ICONSET/icon_16x16@2x.png"
cp "$ICONSET/icon_64x64.png"   "$ICONSET/icon_32x32@2x.png"
cp "$ICONSET/icon_256x256.png" "$ICONSET/icon_128x128@2x.png"
cp "$ICONSET/icon_512x512.png" "$ICONSET/icon_256x256@2x.png"
cp "$ICONSET/icon_1024x1024.png" "$ICONSET/icon_512x512@2x.png"
rm -f "$ICONSET/icon_64x64.png" "$ICONSET/icon_1024x1024.png"
iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/AppIcon.icns"

echo "==> [5/5] Write Info.plist"
cat > "$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>$APP_NAME</string>
  <key>CFBundleDisplayName</key><string>$APP_NAME</string>
  <key>CFBundleIdentifier</key><string>$BUNDLE_ID</string>
  <key>CFBundleExecutable</key><string>$APP_NAME</string>
  <key>CFBundleIconFile</key><string>AppIcon</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>$VERSION</string>
  <key>CFBundleVersion</key><string>$BUILD</string>
  <key>LSMinimumSystemVersion</key><string>14.0</string>
  <key>LSUIElement</key><true/>
  <key>NSHighResolutionCapable</key><true/>
  <key>NSHumanReadableCopyright</key><string>Copyright © 2026 Dat Nguyen. MIT Licensed.</string>
  <key>SUEnableAutomaticChecks</key><true/>
  <key>SUFeedURL</key><string>https://ntd4996.github.io/Porthole/appcast.xml</string>
  <key>SUPublicEDKey</key><string>2wE8euYD3TUI39c6UhWHbYlhHpXkeM5NiJTJywrm0xQ=</string>
</dict>
</plist>
PLIST

echo "✅ Built $APP (v$VERSION build $BUILD)"
