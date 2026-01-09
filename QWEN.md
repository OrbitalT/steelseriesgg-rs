# steelseriesgg-rs - SteelSeries GG for Linux

## Project Overview

steelseriesgg-rs is a complete open-source replacement for SteelSeries GG on Linux. It provides comprehensive control over SteelSeries keyboards and headsets with RGB lighting, audio mixing, GameSense API compatibility, and profile management. The project is written in Rust and supports a wide range of SteelSeries devices including Apex keyboards and Arctis headsets.

### Key Features
- **RGB Lighting Control**: Static colors, breathing, spectrum cycling, wave effects, reactive effects, and gradients
- **GameSense Server**: HTTP API compatible with SteelSeries GameSense for game integrations
- **Audio Mixer**: Per-channel volume control (PulseAudio/PipeWire integration in progress)
- **Sonar API Integration**: Direct control of SteelSeries Sonar audio device
- **Profile Management**: Save and load device configurations
- **Daemon Mode**: Run as a background service

### Supported Devices
#### Keyboards
- Apex Pro / Apex Pro TKL / Apex Pro TKL 2023
- Apex 3 / Apex 3 TKL
- Apex 5
- Apex 7 / Apex 7 TKL

#### Headsets
- Arctis 1 / Arctis 1 Wireless
- Arctis 5 / Arctis 7 / Arctis 7 (2019 Edition)
- Arctis 9 / Arctis Pro / Arctis Pro Wireless
- Arctis Nova Pro / Arctis Nova Pro Wireless
- Arctis Nova 5 / Arctis Nova 3 / Arctis Nova 1

## Project Structure

```
steelseriesgg-rs/
├── Cargo.toml          # Rust project manifest with dependencies and features
├── README.md           # Main project documentation
├── PKGBUILD            # Arch Linux package build script
├── assets/
│   ├── 99-steelseries.rules  # udev rules for device permissions
│   └── ssgg.service          # systemd service file
├── src/
│   ├── main.rs         # CLI entry point
│   ├── lib.rs          # Library exports
│   ├── audio/          # Audio mixer functionality (optional feature)
│   ├── config/         # Configuration management
│   ├── devices/        # Device discovery and management
│   ├── gamesense/      # GameSense API server
│   ├── profiles/       # Profile management
│   ├── rgb/            # RGB lighting control and effects
│   └── error.rs        # Error handling
└── target/             # Build artifacts
```

## Building and Running

### Prerequisites
- Linux with udev support
- Rust toolchain (1.70+)
- `libudev-dev` or equivalent
- `libhidapi-dev` for HID device communication

### Installation Commands
```bash
# Debian/Ubuntu:
sudo apt install libudev-dev libhidapi-dev

# Fedora:
sudo dnf install systemd-devel hidapi-devel

# Arch Linux:
sudo pacman -S hidapi
```

### Building from Source
```bash
git clone https://github.com/Ven0m0/steelseriesgg-rs.git
cd steelseriesgg-rs
cargo build --release
```

The binary will be located at `target/release/ssgg`.

### Device Permissions
To access SteelSeries devices without root, install the udev rules:
```bash
sudo cp assets/99-steelseries.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Then add your user to the `input` group:
```bash
sudo usermod -aG input $USER
```

Log out and log back in for the group change to take effect.

## Usage

### CLI Commands
The project provides a comprehensive CLI tool called `ssgg` with the following main commands:

- `ssgg devices` - List connected SteelSeries devices
- `ssgg rgb` - Control RGB lighting
- `ssgg profile` - Manage profiles
- `ssgg audio` - Control audio mixer (requires audio feature)
- `ssgg sonar` - Control SteelSeries Sonar (requires sonar feature)
- `ssgg server` - Start the GameSense server
- `ssgg daemon` - Run as a background daemon

### RGB Lighting Examples
```bash
# Set a static color
ssgg rgb --color red
ssgg rgb --color "#ff5500"

# Set brightness
ssgg rgb --brightness 80

# Apply effects
ssgg rgb --effect breathing --color cyan
ssgg rgb --effect spectrum
ssgg rgb --effect wave --direction left-to-right
```

### Profile Management
```bash
# Save current configuration
ssgg profile save my-profile

# Load a profile
ssgg profile load my-profile

# List profiles
ssgg profile list
```

### Audio Mixer (with audio feature enabled)
```bash
# View audio status
ssgg audio status

# Set volume
ssgg audio volume --channel master --level 75

# Mute/unmute
ssgg audio mute --channel game
ssgg audio unmute --channel game

# Adjust chat mix
ssgg audio chat-mix --balance 25
```

## Configuration

Configuration is stored in `~/.config/ssgg/config.toml`:

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

## Dependencies

| Crate | Purpose |
|-------|---------|
| hidapi | HID device communication |
| tokio | Async runtime |
| axum | HTTP server for GameSense |
| reqwest | HTTP client for Sonar API |
| serde | Serialization |
| clap | CLI argument parsing |
| libpulse-binding | PulseAudio integration (optional) |

## Feature Flags

- `audio` - Enable audio mixer with PulseAudio support (optional)
- `sonar` - Enable SteelSeries Sonar API integration (optional, requires `audio`)

By default, no optional features are enabled.

Build with audio support:
```bash
cargo build --release --features audio
```

Build with all features:
```bash
cargo build --release --all-features
```

## Development Conventions

- The project follows Rust idioms and conventions
- Error handling is done through the `anyhow` crate for application errors and `thiserror` for library errors
- Async operations use the `tokio` runtime
- Configuration is managed through the `directories` and `toml` crates
- Logging is implemented with `tracing` and `tracing-subscriber`
- The code is organized into logical modules (audio, config, devices, gamesense, profiles, rgb)
- Device discovery and management is handled through the hidapi crate
- RGB effects are computed using an effect engine with caching for performance

## Daemon Mode

The daemon can be run as a systemd user service:
```bash
systemctl --user enable --now ssgg.service
```

This will run the daemon in the background with device control and GameSense server capabilities.

## License

This project is licensed under the MIT License.