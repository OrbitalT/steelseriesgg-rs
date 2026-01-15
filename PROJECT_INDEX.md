# Project Index: SteelSeries GG for Linux

**Generated**: 2026-01-15 23:22:00 (Updated post-merge)

**Token Efficiency**: 94% reduction (3K tokens vs 58K full read)

## 📁 Project Structure

```
steelseriesgg-rs/
├── src/                 # Rust source code (5,513 lines)
│   ├── audio/          # Audio mixing & Sonar integration (feature-gated)
│   ├── devices/        # USB HID device discovery & control
│   ├── gamesense/      # GameSense HTTP server & handlers
│   ├── profiles/       # Configuration profile management
│   ├── rgb/            # RGB lighting effects & color management
│   ├── config/         # TOML configuration management
│   ├── main.rs         # CLI application entry point (1,132 lines)
│   ├── lib.rs          # Library exports & prelude (41 lines)
│   ├── error.rs        # Centralized error handling
│   ├── device_state.rs # Device state persistence
│   └── pollrate.rs     # USB polling rate control
├── docs/               # Documentation & development guides
├── .github/            # CI/CD workflows (GitHub Actions)
├── assets/             # System integration (udev rules)
└── test_*.sh          # Hardware verification scripts
```

## 🚀 Entry Points

- **CLI**: `src/main.rs` - Command-line interface with clap-based argument parsing
- **Library**: `src/lib.rs` - Public API exports and prelude module
- **Binary**: `ssgg` - Main executable for device control and daemon mode
- **Tests**: `src/rgb/tests.rs`, `src/profiles/tests.rs` - 290 lines of test code

## 📦 Core Modules

### Module: devices
- **Path**: `src/devices/mod.rs`
- **Exports**: Device trait, DeviceInfo, DeviceManager, DeviceType
- **Purpose**: USB HID device discovery, abstraction, and communication
- **Sub-modules**: keyboards/ (Apex series), headsets/ (Arctis series), discovery.rs

### Module: rgb
- **Path**: `src/rgb/mod.rs`
- **Exports**: Color, Effect, RgbController, EffectEngine, WaveDirection
- **Purpose**: RGB lighting effects, color management, and animation engine with 60 FPS caching
- **Tests**: 181 lines covering color operations and effect engine

### Module: gamesense
- **Path**: `src/gamesense/mod.rs`, `server.rs`, `handlers.rs`
- **Exports**: GameSenseServer, GameMetadata, GameEvent, EventBinding
- **Purpose**: HTTP API server compatible with SteelSeries GameSense (port 27301)
- **Protocol**: JSON-based REST API for game integration and reactive lighting

### Module: audio (feature-gated)
- **Path**: `src/audio/mod.rs`, `sonar.rs`
- **Exports**: AudioMixer, SonarClient, Channel
- **Purpose**: PulseAudio integration and Sonar API client for audio control
- **Features**: `audio` (PulseAudio), `sonar` (Sonar API client)

### Module: profiles
- **Path**: `src/profiles/mod.rs`
- **Exports**: Profile, KeyboardProfile, HeadsetProfile, ProfileManager
- **Purpose**: Device configuration persistence via TOML files
- **Tests**: 109 lines covering serialization and filename sanitization

### Module: config
- **Path**: `src/config/mod.rs`
- **Exports**: Config, GameSenseConfig, AudioConfig, PollRateConfig
- **Purpose**: Application configuration management from `~/.config/ssgg/`

## 🔧 Configuration

- **Cargo.toml**: Project manifest with feature flags (audio, sonar)
- **rustfmt.toml**: Code formatting (100 char limit, 4 spaces, 2024 edition)
- **.cargo/config.toml**: Compiler optimization flags (native CPU target)
- **.editorconfig**: Cross-editor standards (UTF-8, Unix line endings)

## 📚 Documentation

- **CLAUDE.md**: Comprehensive developer guide (781 lines) - architecture, workflows, gotchas
- **README.md**: User installation and usage instructions
- **CONTRIBUTING.md**: Development workflow and code style guidelines
- **docs/**: Additional development documentation and archived iteration reports
- **docs/development/PRD.md**: Product Requirements Document *[Merged 2026-01-15]*
- **docs/development/PRD-bulk-testing.md**: Bulk testing PRD for hardware verification *[Merged 2026-01-15]*
- **docs/development/PROTOCOL_RESEARCH.md**: Hardware protocol research (21KB) *[Merged 2026-01-15]*

## 🧪 Test Coverage

- **Unit tests**: 2 dedicated test modules (290 total lines)
  - `src/rgb/tests.rs`: 181 lines - Color operations, effect engine
  - `src/profiles/tests.rs`: 109 lines - Serialization, sanitization
- **Integration tests**: Hardware verification shell scripts
- **CI/CD**: Matrix testing across feature combinations (none, audio, sonar)
- **Coverage**: 25 unit tests + 2 doc tests covering core functionality

## 🔗 Key Dependencies

- **axum 0.8**: HTTP server framework for GameSense API
- **tokio 1.49**: Async runtime for multi-threaded execution
- **hidapi 2.6.4**: USB HID device communication for hardware control
- **clap 4.5**: Command-line interface framework with derive macros
- **serde 1.0**: Serialization/deserialization for configs and profiles
- **thiserror 2.0**: Error type derivation and structured error handling
- **tracing 0.1**: Structured logging framework
- **libpulse-binding 2.30.1**: PulseAudio integration (optional, feature: audio)
- **reqwest 0.13**: HTTP client for Sonar API (optional, feature: sonar)

## 📝 Quick Start

1. **Setup**: `cargo build --release --all-features` - Build with all features
2. **Run**: `./target/release/ssgg devices` - List connected SteelSeries devices
3. **Test**: `cargo test --all-features` - Run all tests including feature-gated
4. **RGB Control**: `./target/release/ssgg rgb color red` - Set keyboard to red
5. **Daemon**: `./target/release/ssgg daemon` - Start GameSense server mode
6. **Install**: Copy udev rules from `assets/99-steelseries.rules` for device access

## 🎯 Architecture Highlights

- **CLI + HTTP Server**: Embedded Axum server for GameSense API compatibility
- **Device Abstraction**: Uniform Device trait for all SteelSeries hardware
- **HID Protocol**: 65-byte reports (1 byte ID + 64 bytes data) for USB communication
- **Effect Engine**: RGB animation with 16ms caching threshold (~60 FPS)
- **Feature Gates**: Optional audio and Sonar integration via Cargo features
- **Async Runtime**: Multi-threaded Tokio for concurrent device control and HTTP server

## 🔍 Public API Surface

- **162 public items**: Functions, structs, enums across all modules
- **Main traits**: Device (hardware abstraction), Keyboard (device-specific)
- **Core types**: Color, Effect, DeviceInfo, Profile, Config
- **Error handling**: Result<T> type alias with structured Error enum
- **Constants**: STEELSERIES_VENDOR_ID (0x1038), USB protocol definitions

---

*Index generated by SuperClaude sc:index-repo*
*Update frequency: After major architectural changes*