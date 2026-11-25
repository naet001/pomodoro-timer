#!/bin/sh
set -eu

REPOSITORY="naet001/pomodoro-timer"
API_URL="https://api.github.com/repos/${REPOSITORY}/releases/latest"

fail() {
    printf 'Pomodoro Timer installer: %s\n' "$1" >&2
    exit 1
}

command -v curl >/dev/null 2>&1 || fail "curl is required to download the installer."
command -v python3 >/dev/null 2>&1 || fail "Python 3 is required to select the release download."

os=$(uname -s)
arch=$(uname -m)
case "$arch" in
    x86_64|amd64) arch_label=x86_64 ;;
    aarch64|arm64) arch_label=aarch64 ;;
    *) fail "unsupported CPU architecture: $arch" ;;
esac

case "$os" in
    Darwin)
        asset_suffix="macos-${arch_label}.dmg"
        ;;
    Linux)
        asset_suffix="linux-${arch_label}.AppImage"
        ;;
    *)
        fail "this installer supports macOS and Linux; Windows users should download the Windows installer from GitHub Releases."
        ;;
esac

release_json=$(curl -fsSL -H 'Accept: application/vnd.github+json' "$API_URL") || fail "could not fetch the latest GitHub release."
asset_url=$(printf '%s' "$release_json" | python3 -c '
import json, sys
suffix = sys.argv[1]
release = json.load(sys.stdin)
for asset in release.get("assets", []):
    if asset.get("name", "").endswith(suffix):
        print(asset["browser_download_url"])
        break
' "$asset_suffix")
[ -n "$asset_url" ] || fail "the latest release has no ${asset_suffix} installer yet."

work_dir=$(mktemp -d "${TMPDIR:-/tmp}/pomodoro-timer.XXXXXX") || fail "could not create a temporary directory."
cleanup() {
    rm -rf "$work_dir"
}
trap cleanup EXIT HUP INT TERM

asset_path="$work_dir/$(basename "$asset_url")"
curl -fL --progress-bar "$asset_url" -o "$asset_path" || fail "download failed."

if [ "$os" = Darwin ]; then
    command -v hdiutil >/dev/null 2>&1 || fail "hdiutil is required to install the macOS app."
    command -v ditto >/dev/null 2>&1 || fail "ditto is required to install the macOS app."

    mount_point="$work_dir/mounted"
    mkdir "$mount_point"
    hdiutil attach "$asset_path" -nobrowse -quiet -mountpoint "$mount_point" || fail "could not open the downloaded disk image."
    app_path="$mount_point/Pomodoro Timer.app"
    [ -d "$app_path" ] || {
        hdiutil detach "$mount_point" -quiet >/dev/null 2>&1 || true
        fail "the disk image does not contain Pomodoro Timer.app."
    }
    ditto "$app_path" "/Applications/Pomodoro Timer.app" || {
        hdiutil detach "$mount_point" -quiet >/dev/null 2>&1 || true
        fail "could not copy the app to /Applications; try running the command with sudo."
    }
    hdiutil detach "$mount_point" -quiet || fail "the app was installed, but the disk image could not be detached."
    printf '\nPomodoro Timer was installed to /Applications.\n'
    printf 'Because this app is unsigned, macOS may block its first launch. Open System Settings > Privacy & Security and choose Open Anyway, or Control-click the app and choose Open.\n'
else
    install_dir="${HOME}/.local/bin"
    icon_dir="${HOME}/.local/share/icons/hicolor/512x512/apps"
    desktop_dir="${HOME}/.local/share/applications"
    mkdir -p "$install_dir" "$icon_dir" "$desktop_dir"
    install -m 755 "$asset_path" "$install_dir/pomodoro-timer"
    install -m 644 "$(dirname "$0")/icon.png" "$icon_dir/pomodoro-timer.png" 2>/dev/null || {
        curl -fsSL "https://raw.githubusercontent.com/${REPOSITORY}/main/icon.png" -o "$icon_dir/pomodoro-timer.png" || fail "could not install the application icon."
    }
    cat > "$desktop_dir/pomodoro-timer.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Pomodoro Timer
Comment=A simple desktop Pomodoro timer
Exec=${install_dir}/pomodoro-timer
Icon=pomodoro-timer
Categories=Office;Utility;
Terminal=false
EOF
    printf '\nPomodoro Timer was installed to %s.\n' "$install_dir"
    case ":${PATH}:" in
        *":${install_dir}:"*) ;;
        *) printf 'Add %s to your PATH to launch it from a terminal.\n' "$install_dir" ;;
    esac
    printf 'It should also appear in your desktop application launcher.\n'
fi
