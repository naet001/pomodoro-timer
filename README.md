# Pomodoro Timer

A small desktop Pomodoro timer built with Rust and [egui](https://github.com/emilk/egui). It offers focused work and break sessions with simple controls and no setup beyond Rust.

## Icon

![Pomodoro Timer icon](icon.png)

## Features
- Work and break sessions
- Start, pause, reset, and skip controls
- Manual phase changes: each phase waits for you to start it
- Accurate countdown based on a monotonic clock

## Install

Users on macOS, Linux, and Windows can install the app without installing Rust. Download the appropriate installer from the [GitHub Releases page](https://github.com/naet001/pomodoro-timer/releases) and follow the platform‑specific steps below, or run the one‑line script:

### macOS (one‑line)

```bash
curl -fsSL https://raw.githubusercontent.com/naet001/pomodoro-timer/main/install.sh | sh
```

Downloads the latest `.dmg`, mounts it, copies `Pomodoro Timer.app` to `/Applications`, unmounts the disk, and reminds the user about the unsigned‑app warning (see below).

### Linux (one‑line)

```bash
curl -fsSL https://raw.githubusercontent.com/naet001/pomodoro-timer/main/install.sh | sh
```

Downloads the latest `.AppImage`, installs it to `~/.local/bin/pomodoro-timer`, writes a `.desktop` launcher with the app icon, and creates a `~/.local/share/icons/hicolor/512x512/apps/pomodoro-timer.png` entry.

### Windows

Download the `Pomodoro Timer (x86_64).exe` installer from the **Assets** section of the latest release and run it. It creates a Start‑menu shortcut and an “Add to desktop” option. On first launch Windows SmartScreen may warn that the publisher cannot be verified — click **More info** → **Run anyway**.

---

#### Unsigned‑app warnings

**macOS Gatekeeper** – The first time you open the app you’ll see “Pomodoro Timer can’t be opened because the developer cannot be verified.” To launch it:

1. Open **System Settings** → **Privacy & Security** → **Security** section.
2. Click **Open Anyway** next to the Pomodoro Timer entry, or Control‑click the app in `/Applications` and choose **Open**.

Alternatively, remove the quarantine flag from the command line:

```bash
xattr -d com.apple.quarantine /Applications/Pomodoro\ Timer.app
```

**Windows SmartScreen** – The first launch may show “Windows protected your PC.” To run it:

1. Click **More info** → **Run anyway**.

The unsigned installer warning exists because the app is not code‑signed or notarized by Apple or Microsoft. Signing and notarization would remove these prompts but requires an Apple Developer membership ($99/yr) and a code‑signing certificate from a trusted Windows vendor. If you wish to add signing later, see the [cargo-bundle signing docs][bundle-signing].

[bundle-signing]: https://github.com/burtonageo/cargo-bundle#windows-signing

## Run from source

Clone the repository and start the app:

```bash
git clone https://github.com/naet001/pomodoro-timer.git
cd pomodoro-timer
cargo run
```

Requires Rust with Cargo and a desktop environment with X11 or Wayland.

## Test

Run the timer behavior tests and check the project:

```bash
cargo test
cargo check
cargo fmt --check
```

## Defaults

| Phase | Duration |
| --- | ---: |
| Work | 25 minutes |
| Short break | 5 minutes |
| Long break | 15 minutes |

The timer offers a long break after every four completed work sessions. Settings are not saved between runs.