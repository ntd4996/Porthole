#!/usr/bin/env bash
# Add the Porthole APT repository and install Porthole in one command:
#
#   curl -fsSL https://raw.githubusercontent.com/ntd4996/Porthole/main/linux/setup-apt.sh | sudo bash
#
# After this, `sudo apt upgrade` keeps Porthole up to date like any other package.
set -euo pipefail

if [ "$(id -u)" -ne 0 ]; then
  echo "error: run as root, e.g.  curl -fsSL <url> | sudo bash" >&2
  exit 1
fi

KEYRING="/usr/share/keyrings/porthole.gpg"
LIST="/etc/apt/sources.list.d/porthole.list"
BASE="https://porthole.thenightwatcher.online/apt"

echo "==> Installing prerequisites"
apt-get update -qq
apt-get install -y -qq ca-certificates curl gnupg

echo "==> Adding Porthole signing key + repository"
curl -fsSL "$BASE/key.asc" | gpg --dearmor -o "$KEYRING"
echo "deb [signed-by=$KEYRING] $BASE stable main" > "$LIST"

echo "==> Installing Porthole"
apt-get update -qq
apt-get install -y porthole

echo
echo "Done. Launch 'Porthole' from your app menu (it lives in the system tray)."
echo "Future updates come with: sudo apt upgrade"
echo "GNOME users: enable the AppIndicator/StatusNotifierItem extension to see the tray icon."
