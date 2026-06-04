#!/usr/bin/env bash
# Full release: build -> sign -> notarize -> staple app -> DMG -> notarize -> staple DMG.
#
# Requires:
#   - Developer ID Application cert in keychain
#   - notarytool keychain profile (default: plugtalk-notary)
set -euo pipefail
cd "$(dirname "$0")/.."

VERSION="${VERSION:-0.1.0}"
IDENT="Developer ID Application: Dat Nguyen (9D7HY2JCGN)"
PROFILE="${NOTARY_PROFILE:-plugtalk-notary}"
APP="build/Porthole.app"
DMG="build/Porthole-$VERSION.dmg"

VERSION="$VERSION" ./scripts/build-app.sh

echo "==> Signing app (hardened runtime, timestamp)"
codesign --force --options runtime --timestamp --sign "$IDENT" "$APP"
codesign --verify --strict --verbose=2 "$APP" 2>&1 | tail -2

echo "==> Notarizing app"
ZIP="build/Porthole-notarize.zip"
rm -f "$ZIP"
/usr/bin/ditto -c -k --keepParent "$APP" "$ZIP"
OUT=$(xcrun notarytool submit "$ZIP" --keychain-profile "$PROFILE" --wait 2>&1)
echo "$OUT"
STATUS=$(echo "$OUT" | grep -E "^\s*status:" | tail -1 | awk '{print $2}')
if [[ "$STATUS" != "Accepted" ]]; then
  ID=$(echo "$OUT" | grep -E "^\s*id:" | head -1 | awk '{print $2}')
  echo "❌ App notarization failed ($STATUS)"; xcrun notarytool log "$ID" --keychain-profile "$PROFILE" || true
  exit 1
fi
xcrun stapler staple "$APP"

echo "==> Building DMG (drag-to-Applications background)"
rm -f "$DMG"
rsvg-convert -w 660  -h 420 assets/dmg-bg.svg -o build/dmg-bg.png
rsvg-convert -w 1320 -h 840 assets/dmg-bg.svg -o build/dmg-bg@2x.png
tiffutil -cathidpicheck build/dmg-bg.png build/dmg-bg@2x.png -out build/dmg-bg.tiff >/dev/null 2>&1
STAGE="build/dmg-src"
rm -rf "$STAGE"; mkdir -p "$STAGE"
cp -R "$APP" "$STAGE/"
create-dmg \
  --volname "Porthole" \
  --background "build/dmg-bg.tiff" \
  --window-pos 200 120 \
  --window-size 660 420 \
  --icon-size 120 \
  --icon "Porthole.app" 165 215 \
  --app-drop-link 495 215 \
  --hide-extension "Porthole.app" \
  --no-internet-enable \
  "$DMG" "$STAGE" || true
rm -rf "$STAGE"
[[ -f "$DMG" ]] || { echo "❌ DMG not created"; exit 1; }

echo "==> Signing + notarizing DMG"
codesign --force --sign "$IDENT" "$DMG"
OUT=$(xcrun notarytool submit "$DMG" --keychain-profile "$PROFILE" --wait 2>&1)
echo "$OUT"
STATUS=$(echo "$OUT" | grep -E "^\s*status:" | tail -1 | awk '{print $2}')
if [[ "$STATUS" != "Accepted" ]]; then
  ID=$(echo "$OUT" | grep -E "^\s*id:" | head -1 | awk '{print $2}')
  echo "❌ DMG notarization failed ($STATUS)"; xcrun notarytool log "$ID" --keychain-profile "$PROFILE" || true
  exit 1
fi
xcrun stapler staple "$DMG"
xcrun stapler validate "$DMG"

SHA=$(shasum -a 256 "$DMG" | awk '{print $1}')
echo ""
echo "✅ Release ready:"
echo "   $DMG"
echo "   sha256: $SHA"
