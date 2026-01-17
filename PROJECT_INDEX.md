# Project Index: steelseriesgg-rs

Generated: 2026-01-16T21:47:00Z
**Token Reduction**: 69,819 → ~3,000 tokens (94% reduction)

## 📁 Project Structure

```
steelseriesgg-rs/
├── src/                      # Core Rust source code (20 files)
│   ├── main.rs               # CLI entry point
│   ├── lib.rs                # Library entry point
│   ├── error.rs              # Error types (thiserror)
│   ├── devices/              # Hardware device management
│   │   ├── discovery.rs      # USB device enumeration (hidapi)
│   │   ├── keyboards/        # Keyboard implementations
│   │   └── headsets/         # Headset implementations
│   ├── rgb/                  # RGB lighting engine
│   ├── gamesense/            # HTTP API server (axum)
│   ├── audio/                # Audio mixer & Sonar client
│   ├── profiles/             # Configuration persistence
│   ├── config/               # TOML configuration
│   └── pollrate.rs           # USB poll rate control
├── docs/                     # Documentation (19 files)
│   ├── development/          # Development docs (9 files)
│   └── archive/              # Historical docs (10 files)
├── assets/                   # System integration files
├── .github/workflows/        # CI/CD (4 files)
└── scripts/                  # Shell utilities (3 files)
```

## 🚀 Entry Points

- **CLI**: `src/main.rs` - Command-line interface with clap-based argument parsing
- **Library**: `src/lib.rs` - Public API exports and prelude module
- **Binary**: `ssgg` - Main executable for device control and daemon mode

## 📦 Core Modules

### Module: devices
- **Path**: `src/devices/`
- **Exports**: `Device` trait, `DeviceManager`, `DeviceInfo`, `DeviceType`
- **Purpose**: Hardware abstraction layer for SteelSeries USB devices via hidapi

### Module: rgb
- **Path**: `src/rgb/`
- **Exports**: `Color`, `Effect`, `EffectEngine`, `RgbController`, `WaveDirection`
- **Purpose**: RGB color management and animated lighting effects with caching

### Module: gamesense
- **Path**: `src/gamesense/`
- **Exports**: `GameSenseServer`, `GameMetadata`, `GameEvent`, `Handler`
- **Purpose**: HTTP API server (port 27301) for game integration and reactive lighting

### Module: audio (optional)
- **Path**: `src/audio/`
- **Exports**: `AudioMixer`, `SonarClient`, `Channel`
- **Purpose**: Multi-channel audio mixing and SteelSeries Sonar API client

### Module: profiles
- **Path**: `src/profiles/`
- **Exports**: `Profile`, `ProfileManager`, `KeyboardProfile`, `HeadsetProfile`
- **Purpose**: Save/load device configurations to ~/.config/ssgg/profiles/

### Module: config
- **Path**: `src/config/`
- **Exports**: `Config`, `GameSenseConfig`, `AudioConfig`
- **Purpose**: TOML configuration management (~/.config/ssgg/config.toml)

### Module: error
- **Path**: `src/error.rs`
- **Exports**: `Error`, `Result`
- **Purpose**: Structured error types with context preservation using thiserror

### Module: pollrate
- **Path**: `src/pollrate.rs`
- **Exports**: `PollRate` enum, poll rate control functions
- **Purpose**: USB device poll rate control via sysfs (125Hz-4000Hz)

### Module: device_state
- **Path**: `src/device_state.rs`
- **Exports**: `DeviceStateStore`, `DeviceId`, `KeyboardState`, `HeadsetState`
- **Purpose**: Device state tracking and persistence for daemon mode

## 🔧 Configuration

- **Cargo.toml**: Project metadata, dependencies, feature flags, release optimization
- **rustfmt.toml**: Code formatting rules (4 spaces, 100 char lines)
- **.cargo/config.toml**: Cargo build configuration
- **assets/99-steelseries.rules**: udev rules for device permissions
- **assets/ssgg.service**: systemd user service configuration

## 📚 Documentation

- **README.md**: User-facing documentation and installation guide
- **CLAUDE.md**: Comprehensive developer guide (20KB)
- **CONTRIBUTING.md**: Contribution guidelines and development setup
- **docs/development/PROTOCOL_RESEARCH.md**: HID protocol research findings
- **docs/development/DEPENDENCY_AUDIT_REPORT.md**: Security audit results
- **docs/development/OPTIMIZATION_REPORT.md**: Performance optimization analysis
- **docs/development/PRD.md**: Product requirements document

## 🧪 Test Coverage

- **Unit tests**: 29 tests across 8 modules
- **Test modules**: rgb, profiles, pollrate, gamesense, audio
- **Integration tests**: Device communication tests (requires hardware)
- **Scripts**: 3 shell scripts for RGB testing and device verification

**Test Distribution**:
- `src/rgb/tests.rs`: 11 tests (color, effects, caching)
- `src/profiles/tests.rs`: 6 tests (serialization, defaults)
- `src/pollrate.rs`: 5 tests (conversion, validation)
- `src/gamesense/server.rs`: 3 tests (color computation)
- `src/audio/sonar.rs`: 1 async test (port discovery)

## 🔗 Key Dependencies

- **hidapi 2.6.4**: USB HID device communication (pinned for stability)
- **tokio 1.49**: Async runtime for multi-threaded execution
- **axum 0.8**: HTTP server framework for GameSense API
- **clap 4.5**: Command-line interface framework with derive macros
- **serde 1.0**: Serialization/deserialization for configs and profiles
- **thiserror 2.0**: Error type derivation and structured error handling
- **tracing 0.1**: Structured logging framework
- **libpulse-binding 2.30.1**: PulseAudio integration (optional, feature: audio)
- **reqwest 0.13**: HTTP client for Sonar API (optional, feature: sonar)

## 🎯 Feature Flags

```toml
[features]
default = []                    # Core RGB + GameSense only
audio = ["dep:libpulse-binding"]  # PulseAudio multi-channel mixer
sonar = ["dep:reqwest"]          # SteelSeries Sonar HTTP client
```

## 📝 Quick Start

```bash
# 1. Install dependencies
sudo apt install libudev-dev libhidapi-dev

# 2. Build and install
cargo install --path . --features audio,sonar

# 3. Setup udev rules
sudo cp assets/99-steelseries.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules

# 4. Test device detection
ssgg devices

# 5. Run daemon mode
ssgg daemon
```

## 🏗️ Architecture Patterns

- **Device Abstraction**: Uniform `Device` trait for all hardware
- **Async-First**: Tokio runtime with concurrent device + server operations
- **Feature-Gated**: Optional audio/sonar functionality via Cargo features
- **Caching**: RGB effect engine with 16ms cache for 60 FPS performance
- **HID Protocol**: Standardized 65-byte reports (1 byte ID + 64 bytes data)
- **Error Handling**: Structured errors with context preservation
- **State Management**: Persistent device state for daemon mode

## 📊 Metrics

- **Total Files**: 52 (20 Rust source, 19 docs, 13 other)
- **Lines of Code**: ~8,691 lines
- **Binary Size**: ~2-3 MB (optimized release build)
- **Dependencies**: 14 direct, ~50 transitive
- **Test Coverage**: 29 unit tests, multiple integration tests
- **Documentation**: 20KB developer guide + comprehensive inline docs

## 🔄 Build Optimization

```toml
[profile.release]
strip = true              # Remove debug symbols
lto = "fat"               # Full link-time optimization
codegen-units = 1         # Single codegen unit
panic = "abort"           # No unwinding code
opt-level = 3             # Maximum optimization
```

## 📡 API Surface

**CLI Commands**:
- `ssgg devices` - List connected devices
- `ssgg rgb` - Control RGB lighting
- `ssgg daemon` - Start background service
- `ssgg profile` - Manage device profiles
- `ssgg audio` - Audio mixer control (feature: audio)

**HTTP API** (GameSense):
- `POST /game_metadata` - Register game
- `POST /bind_game_event` - Bind event to device
- `POST /game_event` - Send game state updates
- `GET /game_event` - Query event values

**Rust Library API**:
```rust
use steelseries_gg::prelude::*;

// Device management
let manager = DeviceManager::new()?;
let device = manager.find_device(DeviceType::Keyboard)?;

// RGB control
let mut controller = RgbController::new(Effect::Static {
    color: Color::RED
});
controller.set_brightness(0.8);
```

---

**Index Status**: ✅ Complete
**Usage**: Read this index instead of scanning 69,819 tokens across 52 files
**Update Command**: `/sc:index-repo mode=update`