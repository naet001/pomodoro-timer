# Pomodoro Timer

A small desktop Pomodoro timer built with Rust and [egui](https://github.com/emilk/egui). It offers focused work and break sessions with simple controls and no setup beyond Rust.

## Features

- Work and break sessions
- Start, pause, reset, and skip controls
- Manual phase changes: each phase waits for you to start it
- Accurate countdown based on a monotonic clock

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) with Cargo
- A desktop environment with X11 or Wayland

## Run

Clone the repository and start the app:

```bash
git clone https://github.com/naet001/pomodoro-timer.git
cd pomodoro-timer
cargo run
```

The app opens in a native window. Select **Start** to begin the work timer. When a phase ends, the app shows the next phase; select **Start** when you are ready to continue.

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
