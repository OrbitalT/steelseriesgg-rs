# Project Index: steelseriesgg-rs

**Last Updated**: 2026-01-20  
**Version**: 0.1.0  
**Language**: Rust 2021 Edition (1.70+)  
**License**: MIT

---

## 📋 Quick Facts

| Property | Value |
|----------|-------|
| **Repository** | https://github.com/Ven0m0/steelseriesgg-rs |
| **Binary** | `ssgg` |
| **Library** | `steelseries_gg` |
| **Project Type** | Open-source SteelSeries GG replacement for Linux |
| **Core Features** | RGB control, GameSense server, performance monitoring, profile management |
| **Key Dependencies** | axum, tokio, hidapi, serde, clap |

---

## 📁 Project Structure

```
steelseriesgg-rs/
├── src/
│   ├── lib.rs                 # Library entry point & module declarations
│   ├── main.rs                # CLI application (clap-based)
│   │
│   ├── devices/               # Hardware discovery & HID communication
│   │   ├── mod.rs             # Device trait, DeviceInfo, product mappings
│   │   ├── discovery.rs       # DeviceManager with hidapi enumeration
│   │   ├── hid_reports.rs     # HID report builders & command protocol
│   │   ├── diagnostics.rs     # Device health checks & diagnostics
│   │   ├── key_mapping.rs     # Per-key RGB addressing
│   │   ├── zone_mapping.rs    # RGB zone definitions
│   │   ├── keyboards/         # Keyboard implementations
│   │   │   ├── mod.rs         # Keyboard trait & common logic
│   │   │   ├── apex.rs        # Apex-series devices
│   │   │   └── apex_pro_tkl_2023.rs
│   │   └── headsets/          # Headset implementations
│   │       └── mod.rs
│   │
│   ├── rgb/                   # Color & lighting effects
│   │   ├── mod.rs             # Color, Effect, EffectEngine, RgbController
│   │   └── tests.rs           # RGB unit tests
│   │
│   ├── gamesense/             # GameSense HTTP server
│   │   ├── mod.rs             # GameMetadata, GameEvent structures
│   │   ├── server.rs          # Axum HTTP server
│   │   └── handlers.rs        # Request handlers
│   │
│   ├── performance.rs         # Performance monitoring & stats tracking
│   ├── validation.rs          # Resource validation & leak detection
│   ├── device_state.rs        # Device state tracking for daemon
│   ├── error.rs               # Error types (thiserror)
│   │
│   ├── audio/                 # Audio mixer (feature: audio)
│   │   ├── mod.rs             # AudioMixer (PulseAudio)
│   │   └── sonar.rs           # SonarClient HTTP API
│   │
│   ├── profiles/              # Configuration persistence
│   │   ├── mod.rs             # Profile struct & management
│   │   └── tests.rs           # Profile serialization tests
│   │
│   ├── config/                # TOML configuration
│   │   └── mod.rs             # Config struct (~/.config/ssgg/)
│   │
│   ├── pollrate.rs            # USB poll rate control (sysfs)
│   │
│   └── bin/                   # Utility binaries
│       ├── discover_actuation.rs
│       └── sonar_control.rs
│
├── tests/                     # Integration tests
│   ├── device_readback.rs
│   ├── device_response.sh
│   ├── test_rgb.sh
│   └── verify_rgb_physical.sh
│
├── docs/                      # Documentation
│   ├── development/           # Developer docs
│   │   ├── OPTIMIZATION_REPORT.md
│   │   ├── DEPENDENCY_AUDIT_REPORT.md
│   │   ├── PROTOCOL_RESEARCH.md
│   │   └── RGB_CONTROL_ANALYSIS.md
│   └── archive/               # Historical reports
│
├── assets/                    # System integration
│   ├── 99-steelseries.rules   # udev rules
│   └── ssgg.service           # systemd service
│
├── .planning/                 # GSD project tracking
│   ├── PROJECT.md
│   ├── ROADMAP.md
│   ├── REQUIREMENTS.md
│   └── phases/
│
├── Cargo.toml                 # Manifest with features & dependencies
├── Cargo.lock
├── CLAUDE.md                  # Comprehensive developer guide
├── README.md                  # User documentation
├── CONTRIBUTING.md
├── LICENSE
└── PERFORMANCE_OPTIMIZATIONS.md
```

---

## 🚀 Entry Points

### Binary: `ssgg` (src/main.rs)
- **Type**: CLI application
- **Features**: Device management, RGB control, daemon mode, GameSense server
- **Framework**: clap 4.5 (derive macros)
- **Commands**:
  - `devices` - List connected SteelSeries devices
  - `rgb` - Control RGB lighting (color, effects, per-key)
  - `daemon` - Run daemon mode (continuous monitoring)
  - `profile` - Manage device profiles
  - `pollrate` - Set USB poll rate

### Library: `steelseries_gg` (src/lib.rs)
- **Type**: Rust library for SteelSeries hardware integration
- **Main Exports**: `Device` trait, `DeviceManager`, `RgbController`, `GamesenseServer`
- **Modules**: 13 public modules (see Core Modules below)
- **Use Case**: Embed SteelSeries hardware support in other applications

### Utilities:
- **discover_actuation** - Device capability discovery tool
- **sonar_control** - SonarClient control utility (requires `sonar` feature)

---

## 📦 Core Modules

### `devices` - Hardware Communication
**Path**: `src/devices/`  
**Key Exports**: `Device` (trait), `DeviceManager`, `DeviceInfo`, `DeviceType`, `HidReportBuilder`

**Submodules**:
- **discovery.rs**: DeviceManager with hidapi enumeration, caching, and hot-swap support
- **hid_reports.rs**: Type-safe HID report builders for keyboards (641 bytes) and headsets (65 bytes)
- **diagnostics.rs**: Device health checks (connection, battery, firmware)
- **key_mapping.rs**: Per-key RGB addressing with keyboard layout support
- **zone_mapping.rs**: Multi-zone RGB definitions with fallback logic
- **keyboards/mod.rs**: Keyboard trait and common logic (Apex Pro TKL 2023, Apex, etc.)
- **keyboards/apex.rs**: Apex-series keyboard implementations
- **keyboards/apex_pro_tkl_2023.rs**: Apex Pro TKL 2023 specific (Product ID: 0x1628)
- **headsets/mod.rs**: Headset implementations (Arctis, Sonar)

**Key Types**:
- `Device` - Common interface for all hardware
- `DeviceManager` - Discovery with O(1) lookup caching
- `HidReportBuilder` - Type-safe report construction
- `DeviceInfo` - Vendor ID, Product ID, path, serial

**Critical Constants**:
- `STEELSERIES_VENDOR_ID` = `0x1038`
- `KEYBOARD_REPORT_SIZE` = 641 bytes
- `HEADSET_REPORT_SIZE` = 65 bytes
- `MAX_RGB_ZONES` = 9

---

### `rgb` - Color & Lighting Effects
**Path**: `src/rgb/mod.rs`  
**Key Exports**: `Color`, `Effect`, `EffectEngine`, `RgbController`, `PerKeyRgbController`

**Supported Effects**: Static, Breathing, Spectrum, Wave, Reactive, Gradient, Custom, Off

**Key Types**:
- `Color` - RGB triplet with blend/scale/HSV operations
- `Effect` - Enum of 8+ effect types with parameters
- `EffectEngine` - Computes animated effect colors (16ms caching for 60 FPS)
- `RgbController` - High-level API for brightness + effect control
- `PerKeyRgbController` - Per-key addressable RGB

**Performance**: Animated effects require daemon mode for continuous updates

**Cached Caching**: `CACHE_THRESHOLD_MS` = 16ms (~60 FPS)

---

### `gamesense` - Game Integration HTTP Server
**Path**: `src/gamesense/`  
**Key Exports**: `GameMetadata`, `GameEvent`, `GamesenseServer`

**Submodules**:
- **server.rs**: Axum-based HTTP server with CORS support
- **handlers.rs**: Request handlers for game events

**Key Types**:
- `GameMetadata` - Game registration (name, version)
- `GameEvent` - Event definition with handler bindings
- `EventBinding` - Maps device zones to color effects
- `ColorHandler` - Static color or gradient color support

**Endpoints**:
- `POST /game_metadata` - Register game
- `POST /bind_game_event` - Bind event to device
- `POST /game_event` - Send game state

**Default Port**: `GAMESENSE_DEFAULT_PORT` = 27301

---

### `performance` - Real-time Performance Monitoring
**Path**: `src/performance.rs`  
**Key Exports**: `PerformanceManager`, `PerformanceStats`, `PerformanceMonitor`

**Key Types**:
- `PerformanceManager` - Orchestrates tracking across all operations
- `PerformanceStats` - Current metrics snapshot
- `PerformanceMonitor` - Tracks CPU/memory per operation
- `EffectComputationCache` - Caches effect calculations
- `AdaptiveRefreshController` - Dynamic refresh rate optimization

**Tracked Metrics**:
- CPU usage per operation
- Memory allocations
- HID write latency
- Effect computation time
- Event loop timing

**Auto-runs**: In daemon mode every 30 seconds

---

### `validation` - Resource Leak Detection
**Path**: `src/validation.rs`  
**Key Exports**: `RgbValidator`, `ValidationResult`

**Checks**:
- Memory leak detection
- CPU spike detection
- Latency regression
- Resource cleanup verification

---

### `profiles` - Configuration Persistence
**Path**: `src/profiles/`  
**Key Exports**: `Profile`

**Storage**: `~/.config/ssgg/profiles/<name>.toml`

**Structure**:
```toml
[keyboard]
rgb_effect = "Static"
rgb_color = "#FF0000"
brightness = 80

[headset]
rgb_effect = "Breathing"
rgb_color = "#00FFFF"
```

---

### `audio` - Audio Mixing (Optional Feature: `audio`)
**Path**: `src/audio/`  
**Key Exports**: `AudioMixer`, `SonarClient`

**Submodules**:
- **mod.rs**: AudioMixer (PulseAudio multi-channel)
- **sonar.rs**: SonarClient for Sonar HTTP API

**Channels**: Game, Chat, Media, Aux, Mic

**Note**: Requires `--features audio` or `--all-features`

---

### `config` - Configuration Management
**Path**: `src/config/mod.rs`  
**Key Exports**: `Config`

**Location**: `~/.config/ssgg/config.toml`

---

### `pollrate` - USB Poll Rate Control
**Path**: `src/pollrate.rs`  
**Supported Rates**: 125, 250, 500, 1000, 2000, 4000 Hz

**Implementation**: Writes to `/sys/bus/usb/devices/<device>/bInterval`

---

### `error` - Error Handling
**Path**: `src/error.rs`  
**Framework**: thiserror 2.0

---

### `device_state` - Daemon State Management
**Path**: `src/device_state.rs`  
**Purpose**: Track device state for daemon mode

---

## 🔗 Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| **axum** | 0.8 | HTTP server for GameSense API |
| **tokio** | 1.49 | Async runtime (multi-threaded) |
| **hidapi** | 2.6.4 | USB HID device communication |
| **clap** | 4.5 | CLI argument parsing |
| **serde** | 1.0 | Serialization/deserialization |
| **serde_json** | 1.0 | JSON serialization |
| **toml** | 0.9 | TOML config parsing |
| **tracing** | 0.1 | Logging framework |
| **tracing-subscriber** | 0.3 | Logging implementation |
| **thiserror** | 2.0 | Error handling macros |
| **tower-http** | 0.6 | CORS middleware |
| **chrono** | 0.4 | Date/time with serde support |
| **directories** | 6.0 | XDG config paths |
| **sysinfo** | 0.33 | System information |
| **libpulse-binding** | 2.30.1 | PulseAudio (optional: `audio`) |
| **reqwest** | 0.13 | HTTP client (optional: `sonar`) |

**Feature Flags**:
- `default` - None (minimal)
- `audio` - PulseAudio integration (requires libpulse-binding)
- `sonar` - SonarClient HTTP API (requires reqwest, implies audio)

---

## 🧪 Test Coverage

**~66 unit tests** across modules:

| Module | Tests | Files |
|--------|-------|-------|
| RGB effects | 15+ | `src/rgb/tests.rs` |
| Profiles | 10+ | `src/profiles/tests.rs` |
| Performance | 8+ | `src/performance.rs::tests` |
| GameSense | 6+ | `src/gamesense/handlers.rs::tests` |
| Pollrate | 4+ | `src/pollrate.rs::tests` |
| HID Reports | 12+ | `src/devices/hid_reports.rs::tests` |
| Device Discovery | 8+ | `src/devices/discovery.rs::tests` |
| Validation | 5+ | `src/validation.rs::tests` |

**Run Tests**:
```bash
cargo test                      # All tests
cargo test --all-features       # With features
cargo test rgb::tests           # Specific module
cargo test -- --nocapture       # Show output
```

---

## 🔧 Build Configuration

### Release Profile (Optimized)
```toml
[profile.release]
strip = true           # Remove debug symbols
lto = "fat"            # Full link-time optimization
codegen-units = 1      # Single unit (better optimization)
panic = "abort"        # No unwinding
opt-level = 3          # Maximum optimization
debug = 0              # No debug info
overflow-checks = false
```

**Result**: ~2-3 MB optimized binary

---

## 📚 Documentation Files

| File | Purpose |
|------|---------|
| **CLAUDE.md** | Comprehensive developer guide (entry point for AI) |
| **README.md** | User documentation & getting started |
| **CONTRIBUTING.md** | Contribution guidelines |
| **PERFORMANCE_OPTIMIZATIONS.md** | Performance findings & improvements |
| **docs/development/OPTIMIZATION_REPORT.md** | Detailed optimization metrics |
| **docs/development/DEPENDENCY_AUDIT_REPORT.md** | Security audit |
| **docs/development/PROTOCOL_RESEARCH.md** | USB protocol notes |
| **docs/development/RGB_CONTROL_ANALYSIS.md** | RGB system analysis |

---

## 🎯 Key Architecture Patterns

### 1. Device Abstraction Layer
- Unified `Device` trait for all hardware types
- Product ID → Device type mapping
- Interface number selection (keyboards: #1, headsets: #3)

### 2. HID Report Builder Pattern
- Type-safe report construction
- Automatic padding & sizing
- Prevents communication failures

### 3. Performance Caching
- Effect computation with 16ms threshold (60 FPS)
- RGB report caching for hotspot optimization
- Pool-based memory allocation

### 4. Resource Monitoring
- Real-time CPU/memory tracking
- Automatic leak detection
- Performance regression alerts

### 5. GameSense HTTP API
- Axum async server with CORS
- Event-driven reactive lighting
- Full SteelSeries GameSense compatibility

---

## 🚀 Quick Start

### Setup
```bash
# Clone and enter directory
git clone https://github.com/Ven0m0/steelseriesgg-rs
cd steelseriesgg-rs

# Install udev rules (one-time)
sudo cp assets/99-steelseries.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo usermod -aG input $USER
# Log out and back in

# Build release binary
cargo build --release
```

### Build Commands
```bash
cargo build                              # Debug build
cargo build --release                    # Optimized release
cargo build --all-features               # Include audio + sonar
```

### Run Commands
```bash
# List devices
cargo run -- devices

# Test RGB (one-shot)
cargo run -- rgb --color red
cargo run -- rgb --effect breathing

# Daemon mode (continuous)
cargo run --release -- daemon

# With logging
RUST_LOG=debug cargo run -- devices
RUST_LOG=trace cargo run -- daemon
```

### Testing
```bash
cargo test                              # All tests
cargo test --all-features               # With features
cargo test -- --nocapture               # Show output
```

---

## ⚠️ Common Gotchas

1. **Animated effects**: Require daemon mode; CLI is one-shot
2. **HID report sizes**: Use HidReportBuilder (auto-sized)
3. **Product IDs**: Some devices shared (e.g., 0x12AD = multiple Arctis)
4. **Apex Pro TKL 2023**: Uses 0x1628 (not 0x1618 from docs)
5. **Interface numbers**: Keyboards #1, headsets #3
6. **Feature flags**: `sonar` requires `audio`
7. **Permissions**: Run with user in `input` group (udev rules)
8. **RGB caching**: First `EffectEngine::compute()` always computes

---

## 📊 Performance Metrics

**Recent Optimizations**:
- 20% CPU reduction: Optimized HID communication
- Adaptive timing: Dynamic effect computation intervals
- Resource validation: Automatic leak detection
- Zero-copy: Per-key RGB buffer reuse

**Binary Size**: ~2-3 MB (release, stripped)

---

## 🔗 Related Resources

- **USB Protocol**: See `docs/development/PROTOCOL_RESEARCH.md`
- **Device Support**: See `README.md` for device compatibility matrix
- **GameSense API**: See `src/gamesense/handlers.rs` for endpoint details
- **RGB Effects**: See `src/rgb/tests.rs` for effect examples
- **Project Roadmap**: See `.planning/ROADMAP.md`

---

## 📝 Code Style Guidelines

| Aspect | Standard |
|--------|----------|
| **Indentation** | 4 spaces |
| **Line length** | 100 characters max |
| **Format** | `cargo fmt` (required) |
| **Lints** | Fix all `cargo clippy` warnings |
| **Naming** | snake_case (functions/vars), PascalCase (types), SCREAMING_SNAKE_CASE (constants) |

---

## 🏗️ Development Workflows

### Adding a CLI Command
1. Define in `Cargo.toml` `[[bin]]` or in `main.rs` CLI enum
2. Implement handler function
3. Wire up in command match

### Testing Device Communication
```bash
RUST_LOG=debug cargo run -- devices   # Enable debug logging
RUST_LOG=trace cargo run -- rgb --effect breathing
```

### Working with Features
```bash
cargo test --features audio            # Test with audio
cargo test --all-features              # Test everything
```

---

## 📞 Support Resources

- **GitHub Issues**: https://github.com/Ven0m0/steelseriesgg-rs/issues
- **Developer Guide**: CLAUDE.md (comprehensive AI assistant guide)
- **Debugging**: Enable `RUST_LOG=debug` or `RUST_LOG=trace`
- **Bug Reports**: Include device info, Rust version, full logs

---

**Generated**: 2026-01-20  
**Project Version**: 0.1.0  
**Maintainer**: steelseriesgg-rs contributors
