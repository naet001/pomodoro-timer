# Pomodoro Timer — Agent Instructions

This is a Rust + egui desktop Pomodoro timer. The project builds with Cargo and
produces cross-platform installers via `cargo-bundle`.

## Quick commands (run from the repository root)

| Action | Command |
|--------|---------|
| Format check | `cargo fmt --check` |
| Lint / type check | `cargo check --locked` |
| Run tests | `cargo test --locked` |
| Release build | `cargo build --release` |
| In‑app window icon | Set via `eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png"))` in `src/main.rs` |
| Bundle macOS DMG | `cargo bundle --format dmg --release --locked` (macOS only) |
| Bundle Linux AppImage | `cargo bundle --format appimage --release --locked` |
| Bundle Linux deb | `cargo bundle --format deb --release --locked` |
| Bundle Windows exe | `cargo bundle --format exe --release --locked` |
| Bundle Windows msi | `cargo bundle --format msi --release --locked` |

## Repository layout (key directories)

- `src/` — Rust source: `main.rs`, `app.rs`, `lib.rs`, `timer.rs`
- `assets/` — canonical PNG source for bundle icon conversions; `icon.ico` (Windows)
- `install.sh` — one‑command macOS/Linux installer (downloads latest release asset)
- `installer/windows.iss` — Inno Setup script for Windows EXE installer
- `.github/workflows/release.yml` — CI matrix: Linux AppImage+deb, macOS DMG, Windows exe+msi
- `Cargo.toml` — package metadata, `[package.metadata.bundle]` for `cargo-bundle`
- `README.md` — user‑facing docs; install section first, unsigned‑app warnings
- `target/` — build artifacts (avoid editing)

## Non‑obvious conventions

- The in‑app window icon is **embedded at compile time** via `include_bytes!` and
  `eframe::icon_data::from_png_bytes`; the same PNG works on macOS dock, Windows
  taskbar, and Linux Alt‑Tab.
- Bundle metadata (`[package.metadata.bundle]`) drives `cargo-bundle` to produce
  DMG, AppImage, deb, exe, and msi from a single source tree.
- The app is **unsigned** — macOS Gatekeeper and Windows SmartScreen will block
  the first launch. See `README.md` for exact dismissal steps (`Open Anyway`,
  `xattr -d com.apple.quarantine`, `More info` → `Run anyway`).
- CI builds on tag `v*`; the `publish` job creates a GitHub Release with all
  installers and SHA256 checksums.

## Commit & PR guidance (inferred from repo policy)

- Keep commits small and focused — one change per commit (e.g., "add bundle
  metadata", "rewrite README install section").
- Do **not** modify `.github/workflows/release.yml` without the `workflow`
  OAuth scope; that file is the CI gate for release publishing.
- If adding a new binary or asset, place it under `assets/` or `installer/` and
  reference it from `Cargo.toml` or `install.sh` — never leave loose files at
  repo root.
- Tests live in `src/` modules; `cargo test --locked` must pass before a PR is
  considered ready.
- Do not commit `target/`, `.git/`, or any generated installer artifacts.

## Adding a new platform installer

1. Add the desired `cargo bundle --format <format>` step to
   `.github/workflows/release.yml` (matrix under `bundle-*` jobs).
2. If a new CLI one‑command script is needed, write `install.sh` with OS/
   arch detection and `curl ... | sh` download of the latest release asset.
3. Update `Cargo.toml`'s `[package.metadata.bundle.icon]` if the icon set
   changes (currently `["icon.png"]`).
4. Test locally with `cargo bundle --format <format> --release --locked`.