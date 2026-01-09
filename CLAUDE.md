# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

steelseriesgg-rs is a complete open-source replacement for SteelSeries GG on Linux. It provides RGB lighting control, GameSense API server, audio mixing capabilities (via Sonar integration), and profile management for SteelSeries keyboards and headsets.

Binary name: `ssgg`
Library name: `steelseries_gg`

## Common Build Commands

```bash
# Build for development
cargo build

# Build optimized release binary
cargo build --release

# Build with audio feature (PulseAudio support)
cargo build --release --features audio

# Build with Sonar integration (requires audio feature)
cargo build --release --features sonar

# Build with all features
cargo build --release --all-features

# Run without building
cargo run -- <args>

# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Run specific test by name
cargo test <test_name>
```

The release binary will be at `target/release/ssgg`.

## Code Style

- Rust files: 4 spaces indentation, 100 character line limit (enforced by .editorconfig)
- Use `cargo fmt` before committing
- Run `cargo clippy` to catch common issues

## Architecture Overview

### Module Structure

```
src/
├── lib.rs              # Library entry point with module declarations
├── main.rs             # CLI application with clap argument parsing
├── devices/            # Device discovery and hardware communication
│   ├── mod.rs          # Device trait, DeviceInfo, product ID mappings
│   ├── discovery.rs    # DeviceManager using hidapi for device enumeration
│   ├── keyboards/      # Keyboard-specific implementations
│   └── headsets/       # Headset-specific implementations
├── rgb/                # RGB color and lighting effects
├── gamesense/          # GameSense HTTP server (axum-based)
│   ├── mod.rs          # Data structures (GameMetadata, GameEvent, etc.)
│   ├── server.rs       # HTTP server implementation
│   └── handlers.rs     # Request handlers
├── audio/              # Audio mixer and Sonar API client (optional features)
│   ├── mod.rs          # AudioMixer with PulseAudio integration
│   └── sonar.rs        # SonarClient for SteelSeries Sonar HTTP API
├── profiles/           # Save/load device configurations
├── config/             # TOML configuration management
└── error.rs            # Error types using thiserror
```

### Key Architectural Patterns

**Device Abstraction Layer**
- `Device` trait in `devices/mod.rs` defines common interface for all hardware
- `DeviceManager` handles discovery via hidapi and maintains a cache of connected devices
- Product IDs mapped to device types and names in `devices::product_ids`
- HID communication uses 64-byte reports (padded to 65 bytes including report ID)

**Device Discovery**
- Uses hidapi to enumerate USB HID devices
- Filters by `STEELSERIES_VENDOR_ID` (0x1038)
- Caches devices by (vendor_id, product_id, interface_number) for O(1) lookup
- Call `DeviceManager::refresh()` to rescan devices

**RGB Control Protocol**
- Keyboards use HID reports starting with command bytes (e.g., 0x21 for color, 0x22 for brightness)
- 9-zone RGB layout: each zone gets 3 bytes (R, G, B)
- Reports padded to 65 bytes total (index 0 = report ID 0, index 1 = command)
- Animated effects require continuous updates in daemon mode

**GameSense Server**
- HTTP server on port 27301 (default) compatible with SteelSeries GameSense API
- Uses axum for async HTTP handling with CORS support
- Games register via `/game_metadata`, bind events via `/bind_game_event`, send data via `/game_event`
- Event handlers translate game state to device effects (reactive lighting)

**Feature Flags**
- Default: no optional features (core RGB and GameSense only)
- `audio`: Enables PulseAudio integration for audio mixing
- `sonar`: Enables SteelSeries Sonar API client (requires `audio`)
- Check features with `#[cfg(feature = "audio")]`

**Async Runtime**
- Uses tokio with multi-threaded runtime
- Daemon mode runs GameSense server in background task
- Graceful shutdown on SIGTERM and SIGINT

### HID Communication Details

All device communication goes through hidapi:
1. Open device via `DeviceManager::open_device(info)` using path from `DeviceInfo`
2. Write HID reports: `device.write(&[u8])` (must be 65 bytes: [0, cmd, ...data, padding])
3. Read HID reports: `device.receive_raw(&mut buf)`

### Configuration

Config file location: `~/.config/ssgg/config.toml`
Uses `directories` crate for cross-platform paths
Loaded via `Config::load()` in `config/mod.rs`

## Common Development Workflows

**Adding Support for a New Device**
1. Identify USB product ID from `lsusb` or similar
2. Add product ID constant to `devices::product_ids`
3. Update `device_type_from_product_id()` match statement
4. Update `device_name_from_product_id()` match statement
5. If device needs specialized implementation, create module in `devices/keyboards/` or `devices/headsets/`

**Adding a New CLI Command**
1. Add variant to `Commands` enum in `main.rs`
2. Add command handler function (e.g., `cmd_foo()`)
3. Add match arm in `main()` to call handler
4. If command has subcommands, create new enum (e.g., `FooAction`)

**Testing Device Communication**
- Use `ssgg devices` to list connected devices
- Enable debug logging with `--debug` or `-d` flag
- Use `RUST_LOG=debug` environment variable for more verbose output
- Check HID report bytes match expected protocol in device documentation

**Working with Feature-Gated Code**
- Use `#[cfg(feature = "audio")]` for compile-time feature gates
- Use `#[cfg(any(feature = "audio", feature = "sonar"))]` for multiple features
- Ensure code compiles with no features, each individual feature, and all features

## Dependencies

Core dependencies:
- `hidapi`: HID device communication (pinned to 2.6.4 to avoid libudev-dev issues)
- `tokio`: Async runtime
- `axum`: HTTP server for GameSense API
- `clap`: CLI argument parsing (derive API)
- `serde`/`serde_json`: Serialization
- `tracing`: Logging infrastructure

Optional dependencies:
- `libpulse-binding`: PulseAudio integration (audio feature)
- `steelseries-sonar`: Sonar API types (sonar feature)

## Testing

Currently, the project has minimal automated tests. When adding tests:
- Unit tests: Add `#[cfg(test)]` modules in the same file
- Integration tests: Create files in `tests/` directory
- Device tests require actual hardware or mocking
- Use `cargo test` to run all tests
- Use `cargo test <name>` to run specific tests

## Daemon Mode and Systemd

The daemon runs in foreground with:
- Device monitoring and control
- GameSense server on configured port
- Profile loading on startup
- Graceful shutdown on SIGTERM/SIGINT

Systemd service file: `assets/ssgg.service`
- User service installed to `~/.config/systemd/user/`
- Requires `loginctl enable-linger` for boot-time start without login

## Important Constants and Limits

- SteelSeries Vendor ID: `0x1038`
- HID report size: 65 bytes (report ID + 64 data bytes)
- RGB zones per keyboard: 9 zones
- GameSense default port: 27301
- Volume range: 0.0 to 1.0 (internal) or 0-100 (CLI)

## Known Gotchas

1. **Product ID 0x12AD ambiguity**: Shared by Arctis 1 and Arctis 7 (2017)
2. **Apex Pro TKL 2023 PID**: Hardware uses 0x1628, not documented 0x1618
3. **HID report padding**: Always pad to 65 bytes, with byte 0 = report ID (usually 0x00)
4. **Feature dependencies**: `sonar` feature requires `audio` feature (enforced in Cargo.toml)
5. **Interface numbers**: Some devices expose multiple HID interfaces; use interface_number to distinguish
6. **Animated effects**: Require daemon mode or continuous updates; one-shot commands won't animate

## Release Profile

Optimized for size and performance:
- `strip = true`: Remove debug symbols
- `lto = true`: Link-time optimization
- `codegen-units = 1`: Single codegen unit for better optimization
- `panic = "abort"`: No unwinding
- `opt-level = 3`: Maximum optimization
