<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# src/

## Purpose

Main source directory containing the `steelseries_gg` library crate and the `ssgg` CLI binary. All device control logic, RGB engine, GameSense server, profile management, and audio integration live here.

## Key Files

| File | Description |
|------|-------------|
| `lib.rs` | Library crate root — re-exports all public modules; conditionally enables `audio`/`sonar` feature modules |
| `main.rs` | CLI binary entry point (~3500+ LOC) — clap derive subcommands, tokio async runtime, FmtSubscriber logging init |
| `error.rs` | Typed error enum via `thiserror`; `Result<T>` alias used throughout library code |
| `device_state.rs` | Tracks per-device connection state and hot-plug events |
| `validation.rs` | Input validation (colors, brightness range, actuation values) at CLI boundary |
| `performance.rs` | `PerformanceMonitor` ring buffer (60 frames), `PerformanceManager` adaptive refresh, `RgbTimingMetrics` |
| `pollrate.rs` | USB polling rate read/write (requires root; uses `/sys/bus/usb/`) |
| `diagnostics_export.rs` | `bug-report` subcommand — gathers device info, logs, sysinfo into JSON report |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `devices/` | Device abstraction layer — HID reports, discovery, keyboards, headsets (see `devices/AGENTS.md`) |
| `rgb/` | RGB color/effect engine — Color, Effect, EffectEngine, RgbController (see `rgb/AGENTS.md`) |
| `gamesense/` | GameSense HTTP server on port 27301 (Axum + CORS) (see `gamesense/AGENTS.md`) |
| `profiles/` | Profile load/save/list via JSON files in `~/.config/ssgg/profiles/` (see `profiles/AGENTS.md`) |
| `config/` | Main config file handling `~/.config/ssgg/config.toml` (see `config/AGENTS.md`) |
| `audio/` | PulseAudio mixer + Sonar API — feature-gated (`audio`/`sonar`) (see `audio/AGENTS.md`) |
| `bin/` | Standalone utility binaries for protocol research and benchmarking (see `bin/AGENTS.md`) |

## For AI Agents

### Working In This Directory

- **Library code** (`lib.rs` and all submodules): errors via `crate::error::{Error, Result}` + `?` operator — never `.unwrap()` or `.expect()`
- **Binary code** (`main.rs`, `bin/`): errors via `anyhow::Context` + `?` — never `.unwrap()` or `.expect()`
- Logging: `tracing::{debug, info, warn}` in library code — `FmtSubscriber` initialized once in `main.rs` only
- Feature flags in `lib.rs` control which modules compile — check `Cargo.toml` for current flags

### Testing Requirements

```bash
cargo test                          # default features
cargo test --features sonar         # sonar feature
cargo test --all-features           # full local check
cargo clippy --all-targets --locked -- -D warnings
cargo fmt
```

### Common Patterns

- Error propagation: `?` everywhere — map foreign errors to `crate::error::Error` variants in library code
- Async: `tokio` runtime started in `main.rs`; library is sync except GameSense server
- Feature-gated modules: `#[cfg(feature = "audio")]` wraps `audio` module in `lib.rs`

## Dependencies

### Internal
- All submodules depend on `error.rs` for the `Result` alias
- `main.rs` imports from `devices`, `rgb`, `gamesense`, `profiles`, `config`, `performance`

### External
- `clap` (CLI parsing), `tokio` (async), `tracing`/`tracing-subscriber` (logging), `anyhow` (binary error context), `thiserror` (library errors)

<!-- MANUAL: -->
