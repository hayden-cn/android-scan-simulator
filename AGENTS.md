# AGENTS.md

## Project

Rust GUI app (eframe/egui) that simulates barcode scanning on Android PDA devices via `adb shell am broadcast`.

## Build & Run

```bash
cargo build        # debug build
cargo run          # run the app
cargo build --release  # optimized build
```

No test suite exists.

## Architecture

Single-crate, four modules:

- `src/main.rs` — entry point, configures eframe window (420×340)
- `src/app.rs` — egui UI: device selector, action/key/barcode inputs, send button, status display
- `src/adb.rs` — `list_devices()` parses `adb devices`; `send_broadcast()` runs `adb -s <device> shell am broadcast -a <action> --es <key> <value>`
- `src/config.rs` — TOML config persistence at binary's sibling, named after the binary (`<binary-stem>.toml`, e.g. `android-scan-simulator.toml`); defaults: `action = "android.intent.action.SCAN_RESULT"`, `key = "SCAN_BARCODE"`

## Dependencies

- `eframe 0.32` (egui framework, glow renderer on Windows)
- `serde` + `toml` for config

## Gotchas

- Edition 2024 (Rust 1.86+), resolver v2 required
- `adb` must be in PATH at runtime
- Config file path is relative to the compiled binary, not CWD
