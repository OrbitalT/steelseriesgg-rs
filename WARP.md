# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Core commands

All tooling is standard Cargo/Rust.

### Build

- Debug build:
  ```bash path=null start=null
  cargo build
  ```
- Release build (default features only):
  ```bash path=null start=null
  cargo build --release
  ```
- Release build with audio mixer support (`audio` feature):
  ```bash path=null start=null
  cargo build --release --features audio
  ```
- Release build with both audio mixer and Sonar HTTP integration (`audio` + `sonar` features):
  ```bash path=null start=null
  cargo build --release --features sonar
  ```
- CI-equivalent builds use lockfile resolution:
  ```bash path=null start=null
  cargo build --release --locked
  ```

### Linting and formatting

GitHub Actions runs these jobs; keep them green locally:

- Format check (what CI runs):
  ```bash path=null start=null
  cargo fmt --all -- --check
  ```
- Apply formatting:
  ```bash path=null start=null
  cargo fmt
  ```
- Clippy (what CI runs):
  ```bash path=null start=null
  cargo clippy --all-targets --locked -- -D warnings
  ```

### Tests

- Run the full test suite (unit + integration):
  ```bash path=null start=null
  cargo test
  ```
- Run tests that require the `sonar` feature (e.g. `audio::sonar` tests):
  ```bash path=null start=null
  cargo test --features sonar
  ```
- Run a single test (pattern match on test name):
  ```bash path=null start=null
  cargo test <pattern>
  # example: cargo test port_discovery
  ```

### System dependencies for local builds

The project talks directly to SteelSeries HID devices and (optionally) PulseAudio/PipeWire:
- You need development packages for `libudev` and `hidapi` on your distro (see `README.md` for per-distro commands).
- CI installs `libudev-dev` and `libhidapi-dev` before running clippy/tests.

## Runtime configuration and data

- Config directory is derived via `directories::ProjectDirs` with app name `ssgg` and lives under the standard config root, e.g. `~/.config/ssgg/` on Linux.
- Main config file: `config.toml` in that directory (see the "Configuration" section in `README.md` for example contents).
- Profiles are stored as JSON in `~/.config/ssgg/profiles/*.json` and managed via the `ProfileManager` type and the `ssgg profile` CLI subcommands.
- When the GameSense server starts, it writes a `coreProps.json` discovery file under the platform-specific SteelSeries Engine path (on Linux: `/tmp/steelseries-engine/coreProps.json`) so compatible games can discover the HTTP endpoint.

## High-level architecture

### Crates and entry points

- This is a single Cargo package that exposes both:
  - A library crate `steelseries_gg` (`src/lib.rs`), which defines the core domain types and modules.
  - A binary crate `ssgg` (`src/main.rs`), which is the CLI/daemon entry point using `clap` and `tokio`.
- `src/lib.rs` re-exports a small prelude (`steelseries_gg::prelude`) containing common types (`Device`, `DeviceManager`, `DeviceType`, `Color`, `Effect`, and the central `Error`/`Result`).

### Device layer (USB HID)

- The `devices` module is the hardware abstraction layer around SteelSeries USB HID devices.
  - `devices::discovery::DeviceManager` wraps `hidapi::HidApi` and is responsible for enumerating devices, filtering by the constant vendor ID (`STEELSERIES_VENDOR_ID`), and classifying them by product ID.
  - `devices::product_ids` is the single source of truth for supported product IDs and maps them to `DeviceType` (keyboard, headset) and human-readable names.
  - `DeviceInfo` is an immutable description of a discovered device (VID/PID, interface, serial, etc.).
  - `Device` is a trait that all concrete device implementations implement and provides basic lifecycle + raw HID send/receive operations.
- Keyboard support (`devices::keyboards`):
  - `GenericKeyboard` encapsulates a `hidapi::HidDevice` with convenience methods for zone-based RGB, brightness, and apply semantics.
  - `Keyboard` is a higher-level trait for keyboard-specific behaviors (set whole-board color, zone colors, brightness, etc.).
  - `apex::Apex3Tkl` shows how to layer model-specific behavior (reactive mode, color shift effect, per-zone mapping) on top of `GenericKeyboard`.
- Headset support (`devices::headsets`):
  - `GenericHeadset` is the HID wrapper for Arctis-series headsets.
  - `Headset` provides high-level operations such as battery level, sidetone, mic volume/mute, EQ preset, ChatMix, and auto-off timeout, implemented via model-specific HID reports.
- `DeviceManager::open_device` includes logic to open the correct HID interface per device type (keyboards vs headsets) using a cache keyed by `(vendor_id, product_id, interface_number)` for O(1) lookups.

### RGB and effects

- The `rgb` module owns the color and effect system used across devices and GameSense:
  - `Color` is a small value type with helpers for hex conversion, HSV conversion, blending, scaling, and a set of named constants (e.g. `Color::RED`).
  - `Effect` is an enum capturing the main lighting modes: static, breathing, spectrum, wave, reactive, gradient, custom per-zone, and off.
  - `WaveDirection` and the `Effect::Wave` variant support directional, multi-color wave animations across zones.
  - `EffectEngine` computes the per-zone colors over time given an `Effect`, using an internal timer and a small cache to limit recomputation to ~60 FPS.
  - `RgbController` wraps an `EffectEngine` and a global brightness scalar; it is responsible for producing brightness-adjusted colors that device modules can convert into HID reports.

### Configuration and profiles

- App-wide configuration lives in `config::Config`:
  - Includes `GameSenseConfig` (enable/disable GameSense, bind address, port), feature-gated `AudioConfig` (when the `audio` feature is on), a `default_profile` name, and a `debug` flag.
  - `Config::load`/`save` handle TOML serialization/deserialization to `config.toml` under the per-user config directory, creating directories on demand.
- Profiles (`profiles` module) persist user-facing device settings separately from the base config:
  - `Profile` aggregates optional `KeyboardProfile` and `HeadsetProfile` payloads and a human-readable name/description.
  - `ProfileManager` is responsible for loading/saving JSON profiles from `~/.config/ssgg/profiles`, listing available profiles, and CRUD operations.
  - The CLI `profile` subcommands (`ssgg profile list/load/save/delete`) are thin wrappers over `ProfileManager`; applying the profile to live devices is currently left as TODOs in `main.rs`.

### Audio mixer and Sonar integration (feature-gated)

- The `audio` module is only compiled when the corresponding features are enabled:
  - `Channel`, `ChannelState`, `MixerState`, and `AudioMixer` model a multi-channel mixer (`Master`, `Game`, `Chat`, `Media`, `Aux`, `Mic`) with per-channel volume and mute plus a ChatMix balance.
  - The current implementation is intentionally a domain model only; `AudioMixer::apply_channel` is a stub that will eventually integrate with PulseAudio/PipeWire via `libpulse-binding`.
  - `AudioRouter` supports per-application routing by mapping executable names to channels and optional per-app volume overrides.
- Sonar integration (`audio::sonar`) is compiled when the `sonar` feature is enabled:
  - `SonarClient` discovers the dynamic Sonar HTTP API port (via `http://127.0.0.1:6327/subApps` plus fallbacks) and then exposes strongly-typed GET/PUT calls for all major endpoints (mode, devices, configs, classic/streamer volume settings, chat mix, stream redirections, etc.).
  - This is used by the `ssgg sonar ...` CLI subcommands to drive an existing SteelSeries Sonar installation; tests in this module expect Sonar to be running and will otherwise fail.

### GameSense HTTP server

- The `gamesense` module implements a SteelSeries GameSense-compatible HTTP API:
  - `GameSenseServer` (in `gamesense::server`) is an `axum`-based server with CORS enabled that exposes the standard endpoints (`/game_metadata`, `/bind_game_event`, `/game_event`, `/game_heartbeat`, `/remove_game`, `/remove_game_event`) plus a small info endpoint.
  - At startup, `run()` writes a `coreProps.json` file in the standard SteelSeries Engine location so games can automatically find the server.
  - The server maintains in-memory state (`ServerState`) for registered games, bindings, last event values, and an optional RGB callback.
  - Handlers (`gamesense::handlers`) define helper constructors and presets for common patterns: static colors, health bar gradients, ammo/cooldown indicators, plus predefined zone names.
  - When events are received, the server computes RGB colors from the binding definitions and forwards them to the configured RGB callback; higher layers can connect this to `RgbController` + device-specific code.

### CLI / daemon behavior

- `src/main.rs` is the `ssgg` CLI/daemon entry point:
  - Uses `clap` to define subcommands: `devices`, `rgb`, `profile`, `audio` (feature `audio`), `sonar` (feature `sonar`), `server`, and `daemon`.
  - Global options include a `--debug` flag that configures the `tracing_subscriber` log level.
- Subcommand flows:
  - `devices`: instantiates a `DeviceManager` and prints a sorted summary of connected SteelSeries devices.
  - `rgb`: opens the first detected keyboard device and sends HID reports for color, brightness, simple effects, or turning LEDs off directly; more advanced animations are intended to run under `daemon`.
  - `profile`: delegates to `ProfileManager` for listing, saving, loading, and deleting profile JSON files.
  - `audio` and `sonar` (when compiled in): wrap `AudioMixer` and `SonarClient` operations in user-friendly CLI commands for status, volume, mute, chat mix, and streamer-mode controls.
  - `server`: spawns a standalone GameSense HTTP server on the requested port.
  - `daemon`: loads `Config`, optionally starts an embedded GameSense server, prints a device summary, tries to load the configured default profile, and then blocks until it receives a termination signal (SIGTERM/SIGINT on Unix or Ctrl+C on other platforms), shutting down gracefully.

These details should be sufficient for future Warp agents to locate the right modules, understand major responsibilities, and run the appropriate commands during development.