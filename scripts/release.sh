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

echo "==> Building DMG"
rm -f "$DMG"
STAGE="build/dmg-stage"
rm -rf "$STAGE"; mkdir -p "$STAGE"
cp -R "$APP" "$STAGE/"
ln -s /Applications "$STAGE/Applications"
hdiutil create -volname "Porthole" -srcfolder "$STAGE" -ov -format UDZO "$DMG" >/dev/null
rm -rf "$STAGE"

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
