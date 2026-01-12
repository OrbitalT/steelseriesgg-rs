# CLAUDE.md - Developer Guide

This file provides comprehensive guidance for AI assistants (Claude Code) and developers working with this codebase.

## Table of Contents

1. [Project Overview](#project-overview)
2. [Quick Reference](#quick-reference)
3. [Architecture](#architecture)
4. [Module Documentation](#module-documentation)
5. [Development Workflows](#development-workflows)
6. [Testing Strategy](#testing-strategy)
7. [Common Gotchas](#common-gotchas)

---

## Project Overview

**steelseriesgg-rs** is a complete open-source replacement for SteelSeries GG on Linux, providing:
- RGB lighting control for SteelSeries keyboards and headsets
- GameSense API server for game integration
- Audio mixing capabilities (via Sonar integration)
- Profile management for device configurations

**Binary name**: `ssgg`
**Library name**: `steelseries_gg`
**Language**: Rust 1.70+
**License**: MIT

### Feature Flags

- **Default**: Core RGB control + GameSense server (no optional features)
- **`audio`**: PulseAudio integration for audio mixing
- **`sonar`**: SteelSeries Sonar API client (requires `audio`)

---

## Quick Reference

### Essential Commands

```bash
# Development
cargo build                              # Debug build
cargo build --release                    # Optimized release
cargo build --release --all-features     # All features enabled

# Code Quality
cargo fmt                                # Format code (required before commit)
cargo clippy                             # Lint (catch issues)
cargo clippy --all-features              # Lint with all features
cargo test                               # Run all tests

# Testing Device Communication
RUST_LOG=debug cargo run -- devices     # List devices with debug logging
cargo run -- rgb --color red             # Test RGB control

# Running the Daemon
cargo run --release -- daemon            # Foreground daemon mode
systemctl --user status ssgg.service     # Check systemd service status
```

### Code Style Standards

- **Indentation**: 4 spaces (enforced by `.editorconfig`)
- **Line Length**: 100 characters maximum
- **Format**: Always run `cargo fmt` before committing
- **Lint**: Fix all `cargo clippy` warnings
- **Naming**:
  - Snake_case for functions, variables, modules
  - PascalCase for types, traits, enums
  - SCREAMING_SNAKE_CASE for constants

### Important Constants

| Constant | Value | Purpose |
|----------|-------|---------|
| `STEELSERIES_VENDOR_ID` | `0x1038` | USB vendor ID for all SteelSeries devices |
| `HID_REPORT_SIZE` | 65 bytes | Report ID (1 byte) + data (64 bytes) |
| `RGB_ZONE_COUNT` | 9 | Number of RGB zones on keyboards |
| `GAMESENSE_DEFAULT_PORT` | 27301 | Default GameSense HTTP server port |
| `CACHE_THRESHOLD_MS` | 16 | RGB effect cache threshold (~60 FPS) |

---

## Architecture

### Module Structure

```
src/
├── lib.rs              # Library entry point, module declarations, prelude
├── main.rs             # CLI application (clap-based argument parsing)
├── error.rs            # Error types (using thiserror)
│
├── devices/            # Device discovery and hardware communication
│   ├── mod.rs          # Device trait, DeviceInfo, product ID mappings
│   ├── discovery.rs    # DeviceManager with hidapi-based enumeration
│   ├── keyboards/      # Keyboard-specific implementations
│   │   ├── mod.rs      # Keyboard trait and common logic
│   │   └── generic.rs  # GenericKeyboard for most keyboards
│   └── headsets/       # Headset-specific implementations
│       └── mod.rs      # Headset trait and implementations
│
├── rgb/                # RGB color and lighting effects
│   ├── mod.rs          # Color struct, Effect enum, EffectEngine
│   └── tests.rs        # RGB effect tests
│
├── gamesense/          # GameSense HTTP server (axum-based)
│   ├── mod.rs          # Data structures (GameMetadata, GameEvent, etc.)
│   ├── server.rs       # Axum HTTP server implementation
│   └── handlers.rs     # Request handlers for GameSense API endpoints
│
├── audio/              # Audio mixer and Sonar client (optional features)
│   ├── mod.rs          # AudioMixer with PulseAudio integration
│   └── sonar.rs        # SonarClient for SteelSeries Sonar HTTP API
│
├── profiles/           # Save/load device configurations
│   ├── mod.rs          # Profile struct and management
│   └── tests.rs        # Profile serialization tests
│
├── config/             # TOML configuration management
│   └── mod.rs          # Config struct, loading from ~/.config/ssgg/
│
├── pollrate.rs         # USB poll rate control (sysfs-based)
└── device_state.rs     # Device state tracking for daemon mode
```

### Key Architectural Patterns

#### 1. Device Abstraction Layer

**Purpose**: Provide a uniform interface for all SteelSeries hardware

```rust
pub trait Device: Send + Sync {
    fn info(&self) -> &DeviceInfo;
    fn initialize(&mut self) -> Result<()>;
    fn send_raw(&mut self, data: &[u8]) -> Result<()>;
    // ... more methods
}
```

- `Device` trait in `devices/mod.rs` defines the common interface
- `DeviceManager` handles discovery via hidapi
- Product IDs mapped to device types in `device_type_from_product_id()`
- HID communication uses 64-byte reports (padded to 65 bytes with report ID)

#### 2. Device Discovery & Caching

**Purpose**: Efficient device enumeration with O(1) lookup

```rust
pub struct DeviceManager {
    api: HidApi,
    devices: HashMap<String, DeviceInfo>,          // path -> info
    device_cache: HashMap<(u16, u16, i32), String>, // (vid, pid, iface) -> path
}
```

**How it works**:
1. `refresh()` scans USB devices via hidapi
2. Filters by `STEELSERIES_VENDOR_ID` (0x1038)
3. Caches devices by `(vendor_id, product_id, interface_number)` for O(1) lookup
4. `open_device()` uses cache to avoid O(n) iteration

**When to call**: `DeviceManager::refresh()` to rescan devices (e.g., after hotplug)

#### 3. RGB Effect Engine

**Purpose**: Compute animated RGB effects efficiently with caching

```rust
pub struct EffectEngine {
    effect: Effect,
    cached_colors: Vec<Color>,
    last_compute_time: Duration,
    cache_threshold: Duration,  // 16ms = ~60 FPS
}
```

**Caching strategy**:
- Returns cached colors if time delta < 16ms
- **CRITICAL**: Always computes on first call (`last_compute_time == Duration::ZERO`)
- Reuses `Vec<Color>` to avoid allocations

**Bug fix (2026-01-12)**: Fixed first-call caching bug where initial compute would return black colors

#### 4. HID Communication Protocol

**Purpose**: Low-level device communication

**Standard HID Report Format**:
```
[Report ID: 1 byte] [Command: 1 byte] [Data: 63 bytes]
Total: 65 bytes
```

**Example RGB Command**:
```rust
let mut report = vec![0u8; 65];
report[0] = 0x00;        // Report ID (usually 0)
report[1] = 0x21;        // Command (0x21 = set color)
report[2..29].copy_from_slice(&color_data); // 9 zones × 3 bytes RGB
// Remaining bytes are padding (0x00)
device.write(&report)?;
```

**Important**: Always pad to exactly 65 bytes!

#### 5. GameSense Server

**Purpose**: HTTP API compatible with SteelSeries GameSense

**Architecture**:
- Axum-based async HTTP server on port 27301 (default)
- CORS enabled for cross-origin requests
- Event-driven: games register → bind events → send game state → device reacts

**Key Endpoints**:
- `POST /game_metadata`: Register game
- `POST /bind_game_event`: Bind event to device handler
- `POST /game_event`: Send game state updates

**Handler Flow**:
```
Game → /game_event → Event Handler → RgbController → Device HID Report
```

#### 6. Async Runtime

**Purpose**: Handle concurrent operations (server + device control)

- Uses `tokio` with multi-threaded runtime
- Daemon mode runs GameSense server in background task
- Graceful shutdown on SIGTERM/SIGINT via `tokio::signal`

---

## Module Documentation

### `devices` - Device Discovery & Hardware Communication

**Responsibility**: Enumerate USB devices, manage connections, abstract hardware differences

**Key Types**:
- `DeviceManager`: Discovers and manages device connections
- `Device` trait: Common interface for all devices
- `DeviceInfo`: Metadata about a device (VID, PID, path, etc.)
- `DeviceType`: Enum for Keyboard, Headset, Unknown

**Product ID Mapping**:
Product IDs are mapped in `device_type_from_product_id()` and `device_name_from_product_id()`.

**Adding a new device**:
1. Find product ID: `lsusb | grep SteelSeries`
2. Add constant to `product_ids` module
3. Update `device_type_from_product_id()` match
4. Update `device_name_from_product_id()` match
5. If device needs special logic, create module in `keyboards/` or `headsets/`

**Common Interface Numbers**:
- Keyboards: Interface 1 for control
- Headsets: Interface 3 for control
- Use `open_device()` to automatically select correct interface

### `rgb` - Color & Lighting Effects

**Responsibility**: RGB color representation, effect computation, brightness control

**Key Types**:
- `Color`: RGB color (u8, u8, u8) with blend, scale, HSV conversion
- `Effect`: Enum for Static, Breathing, Spectrum, Wave, Reactive, Gradient, Custom, Off
- `EffectEngine`: Computes colors over time with caching
- `RgbController`: High-level controller with brightness support

**Effect Examples**:
```rust
// Static red
Effect::Static { color: Color::RED }

// Breathing cyan at 1 cycle/sec
Effect::Breathing { color: Color::CYAN, speed: 1.0 }

// Rainbow spectrum
Effect::Spectrum { speed: 0.5 }

// Wave with custom colors
Effect::Wave {
    colors: vec![Color::RED, Color::BLUE],
    speed: 2.0,
    direction: WaveDirection::LeftToRight,
}
```

**Performance Note**: Animated effects require daemon mode for continuous updates

### `gamesense` - GameSense HTTP Server

**Responsibility**: HTTP API for game integration, reactive lighting

**Key Types**:
- `GameMetadata`: Game registration data
- `GameEvent`: Event type (e.g., "health", "ammo")
- `GameEventHandler`: Maps event values to device effects
- `GamesenseServer`: Axum HTTP server

**Event Binding Example**:
```json
{
  "game": "CSGO",
  "event": "health",
  "handlers": [
    {
      "device-type": "keyboard",
      "zone": "all",
      "color": {
        "gradient": {
          "zero": {"red": 255, "green": 0, "blue": 0},
          "hundred": {"red": 0, "green": 255, "blue": 0}
        }
      }
    }
  ]
}
```

**Server Lifecycle**:
1. `GamesenseServer::new()` creates server
2. `run()` starts HTTP listener
3. Games send events via POST requests
4. Handlers translate events to RGB effects

### `audio` - Audio Mixer & Sonar Integration (optional)

**Responsibility**: Multi-channel audio mixing, Sonar API client

**Feature Gates**:
- `audio`: PulseAudio integration
- `sonar`: HTTP client for SteelSeries Sonar

**Key Types**:
- `AudioMixer`: Multi-channel mixer (Game, Chat, Media, Aux, Mic)
- `SonarClient`: HTTP client for Sonar API

**Sonar Port Discovery**:
Sonar uses dynamic ports. Use `sonar discover` to find the active port.

### `profiles` - Configuration Persistence

**Responsibility**: Save/load device configurations

**Storage Location**: `~/.config/ssgg/profiles/<name>.toml`

**Profile Structure**:
```toml
[keyboard]
rgb_effect = "Static"
rgb_color = "#FF0000"
brightness = 80

[headset]
rgb_effect = "Breathing"
rgb_color = "#00FFFF"
```

**Usage**:
```rust
let profile = Profile::load("gaming")?;
profile.apply_to_device(&mut device)?;
profile.save("gaming")?;
```

### `pollrate` - USB Poll Rate Control

**Responsibility**: Adjust USB poll rate via sysfs

**Supported Rates**:
- 125 Hz (8ms)
- 250 Hz (4ms)
- 500 Hz (2ms)
- 1000 Hz (1ms)
- 2000 Hz (0.5ms)
- 4000 Hz (0.25ms) - Maximum supported

**Note**: 8000 Hz is NOT supported by the kernel (capped at 4000 Hz)

**Implementation**: Writes to `/sys/bus/usb/devices/<device>/bInterval`

---

## Development Workflows

### Adding a New Device

1. **Identify Product ID**:
   ```bash
   lsusb | grep SteelSeries
   # Example: Bus 001 Device 005: ID 1038:1628 SteelSeries ApS
   ```

2. **Add Product ID Constant**:
   ```rust
   // src/devices/mod.rs (product_ids module)
   pub const APEX_PRO_TKL_2023: u16 = 0x1628;
   ```

3. **Update Type Mapping**:
   ```rust
   pub fn device_type_from_product_id(product_id: u16) -> DeviceType {
       match product_id {
           product_ids::APEX_PRO_TKL_2023 => DeviceType::Keyboard,
           // ... other devices
       }
   }
   ```

4. **Update Name Mapping**:
   ```rust
   pub fn device_name_from_product_id(product_id: u16) -> &'static str {
       match product_id {
           product_ids::APEX_PRO_TKL_2023 => "Apex Pro TKL (2023)",
           // ... other devices
       }
   }
   ```

5. **Test Device Detection**:
   ```bash
   cargo run -- devices
   ```

6. **If device needs custom logic**:
   Create module in `devices/keyboards/` or `devices/headsets/`

### Adding a New CLI Command

1. **Add Command Variant**:
   ```rust
   // src/main.rs
   #[derive(Parser)]
   enum Commands {
       // ... existing commands
       MyCommand(MyCommandArgs),
   }

   #[derive(Args)]
   struct MyCommandArgs {
       #[arg(long)]
       my_option: String,
   }
   ```

2. **Add Handler Function**:
   ```rust
   fn cmd_my_command(args: MyCommandArgs) -> Result<()> {
       // Implementation
       Ok(())
   }
   ```

3. **Add Match Arm**:
   ```rust
   match cli.command {
       Commands::MyCommand(args) => cmd_my_command(args)?,
       // ... other commands
   }
   ```

### Testing Device Communication

**Enable Debug Logging**:
```bash
RUST_LOG=debug cargo run -- devices
```

**Inspect HID Reports**:
```rust
use tracing::debug;
debug!("Sending HID report: {:02X?}", &report);
```

**Test RGB Commands**:
```bash
# Static red
cargo run -- rgb --color red

# Breathing effect
cargo run -- rgb --effect breathing --color cyan

# List devices
cargo run -- devices
```

**Monitor Daemon Logs**:
```bash
journalctl --user -u ssgg.service -f
```

### Working with Feature-Gated Code

**Compile-Time Feature Gates**:
```rust
#[cfg(feature = "audio")]
pub mod audio;

#[cfg(any(feature = "audio", feature = "sonar"))]
use crate::audio::AudioMixer;
```

**Test All Feature Combinations**:
```bash
# No features (default)
cargo test

# Audio only
cargo test --features audio

# All features
cargo test --all-features
```

**Ensure code compiles with all combinations!**

---

## Testing Strategy

### Current Test Coverage

- **RGB Module**: 11 tests (color, effects, caching)
- **Profiles Module**: 7 tests (serialization, defaults)
- **Pollrate Module**: 4 tests (conversion, validation)
- **GameSense Module**: 3 tests (color computation)

**Total**: 25 unit tests, 2 doc tests

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = create_test_input();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_effect_engine_static

# With output
cargo test -- --nocapture

# Specific module
cargo test rgb::tests
```

### Adding Tests for New Features

1. Create `#[cfg(test)] mod tests` in same file
2. Import module under test: `use super::*;`
3. Write unit tests for public functions
4. Test edge cases (empty input, invalid values, etc.)
5. Run `cargo test` to verify

**Note**: Device tests require actual hardware or mocking (currently limited coverage)

---

## Common Gotchas

### 1. Product ID Ambiguity

**Problem**: Product ID `0x12AD` is shared by Arctis 1 and Arctis 7 (2017)

**Solution**: Can't distinguish without additional heuristics. Currently defaults to "Arctis 1 / Arctis 7"

### 2. Apex Pro TKL 2023 Product ID Mismatch

**Problem**: Hardware uses `0x1628`, but some documentation shows `0x1618`

**Solution**: Always use `0x1628` (verified on actual hardware)

### 3. HID Report Padding

**Problem**: Forgetting to pad HID reports to exactly 65 bytes causes communication failures

**Solution**: Always use `write_padded_report()` helper:
```rust
write_padded_report(&device, &data, 65, true)?;
```

**Why 65 bytes?**: Report ID (1 byte) + data (64 bytes)

### 4. Feature Dependency: `sonar` requires `audio`

**Problem**: Building with `--features sonar` fails if `audio` not enabled

**Solution**: This is enforced in `Cargo.toml`:
```toml
[features]
sonar = ["audio", "steelseries-sonar"]
```

Always use `--features audio,sonar` or `--all-features`

### 5. Interface Numbers

**Problem**: Some devices expose multiple HID interfaces (e.g., keyboard might have 0, 1, 2)

**Solution**: Use correct interface for control:
- Keyboards: Interface 1
- Headsets: Interface 3
- `DeviceManager::open_device()` automatically selects the right one

### 6. Animated Effects Require Daemon Mode

**Problem**: Running `ssgg rgb --effect breathing` shows no animation

**Why**: CLI commands are one-shot. Breathing requires continuous color updates.

**Solution**: Use daemon mode:
```bash
ssgg daemon
# Now effects animate continuously
```

### 7. RGB Effect Caching Bug

**Problem** (Fixed 2026-01-12): `EffectEngine::compute()` returned black colors on first call

**Root Cause**: Caching logic returned cached (empty) colors when `elapsed < cache_threshold` on initial call

**Fix**: Check `last_compute_time != Duration::ZERO` before returning cached colors

**Lesson**: Always compute on first call, then cache subsequent calls

### 8. HID Report Byte Order

**Problem**: RGB colors appear wrong (e.g., red shows as blue)

**Why**: Byte order might be BGR instead of RGB for some devices

**Solution**: Check device documentation. Most SteelSeries devices use RGB order.

### 9. udev Permissions

**Problem**: "Permission denied" when accessing devices

**Solution**: Install udev rules:
```bash
sudo cp assets/99-steelseries.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo usermod -aG input $USER
# Log out and back in
```

### 10. Daemon Systemd Service Won't Start at Boot

**Problem**: Service only starts after login

**Solution**: Enable linger for user:
```bash
sudo loginctl enable-linger $USER
```

---

## Configuration Files

### Config Location

`~/.config/ssgg/config.toml`

### Default Config

```toml
[gamesense]
enabled = true
bind = "127.0.0.1"
port = 27301

[audio]
master_volume = 100
game_volume = 100
chat_volume = 100

[general]
default_profile = "default"
debug = false
```

### Profiles Location

`~/.config/ssgg/profiles/<name>.toml`

---

## Release Profile Optimization

The `Cargo.toml` release profile is optimized for size and performance:

```toml
[profile.release]
strip = true              # Remove debug symbols
lto = true                # Link-time optimization
codegen-units = 1         # Single codegen unit (better optimization)
panic = "abort"           # No unwinding (smaller binary)
opt-level = 3             # Maximum optimization
```

**Result**: ~2-3 MB binary with excellent performance

---

## Documentation Files

### Root Documentation
- `README.md`: User-facing documentation
- `CLAUDE.md`: This file (developer guide)
- `CONTRIBUTING.md`: Contribution guidelines
- `LICENSE`: MIT license

### Development Documentation
- `docs/development/`: Development reports and notes
  - `OPTIMIZATION_REPORT.md`: Performance optimization findings
  - `DEPENDENCY_AUDIT_REPORT.md`: Security audit results
  - `todo.md`: Development notes and future work

### Archived Documentation
- `docs/archive/`: Historical/session documentation
  - Previous verification reports (RGB_*, RALPH_*, etc.)
  - No longer needed for active development

---

## Getting Help

### Debug Logging

Enable verbose logging:
```bash
RUST_LOG=debug cargo run -- devices
RUST_LOG=trace cargo run -- rgb --color red
```

### Common Issues

1. **Device not detected**: Check udev rules, permissions, USB connection
2. **RGB not working**: Verify interface number, check HID reports with RUST_LOG=debug
3. **GameSense not responding**: Check port 27301 not in use, firewall rules
4. **Audio features failing**: Ensure PulseAudio running, check feature flags

### Reporting Bugs

1. Run with `RUST_LOG=debug`
2. Capture full output
3. Include device info from `ssgg devices`
4. Check existing issues on GitHub

---

**Last Updated**: 2026-01-12
**Maintainer**: steelseriesgg-rs contributors
