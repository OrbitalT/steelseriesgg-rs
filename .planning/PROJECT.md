# steelseriesgg-rs

## What This Is

A complete open-source replacement for SteelSeries GG on Linux, providing RGB lighting control for keyboards and headsets, GameSense API server for game integration, and audio mixing capabilities. This project enables Linux users to have full control over their SteelSeries devices without requiring proprietary software.

## Core Value

Linux users should have the same level of device control and game integration as Windows users, with a lightweight, reliable daemon that just works.

## Requirements

### Validated

- ✓ RGB lighting control for SteelSeries keyboards and headsets — existing
- ✓ GameSense HTTP API server compatible with games — existing
- ✓ Device discovery and management via HID — existing
- ✓ Profile system for saving/loading device configurations — existing
- ✓ Audio mixing via PulseAudio integration — existing
- ✓ SteelSeries Sonar API client integration — existing
- ✓ USB poll rate control via sysfs — existing
- ✓ CLI interface for device control — existing
- ✓ Daemon mode for continuous operation — existing

### Active

- [ ] Enhanced device diagnostics and debugging tools
- [ ] Advanced key mapping and zone mapping functionality
- [ ] Performance optimizations for RGB effects and HID communication
- [ ] Expanded device protocol support for newer models
- [ ] Web UI for easier configuration management
- [ ] Plugin system for custom effects and integrations

### Out of Scope

- Windows/macOS support — Linux-focused project
- Hardware modifications or firmware changes — software-only solution
- Real-time audio processing beyond mixing — not a DAW replacement

## Context

This is a mature Rust project with comprehensive architecture already in place. The codebase has been optimized for performance and follows Rust best practices. Recent work has focused on protocol reverse-engineering for the Apex Pro series and performance optimizations. The project serves as a critical tool for the Linux gaming community who need SteelSeries device support.

Key technical context:
- Uses hidapi for HID communication with 65-byte report protocol
- Axum-based HTTP server for GameSense compatibility
- Tokio async runtime for concurrent operations
- Feature-gated audio capabilities (PulseAudio + Sonar)
- Comprehensive test suite with 25+ unit tests

## Constraints

- **Platform**: Linux-only — Windows has official SteelSeries GG
- **Language**: Rust 1.70+ — established codebase and ecosystem
- **Dependencies**: Minimal external dependencies — security and reliability
- **Compatibility**: Must maintain GameSense API compatibility — existing games depend on it
- **Performance**: Sub-16ms RGB updates for smooth effects — gaming requirements
- **License**: MIT — open source with permissive licensing

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust language choice | Memory safety, performance, Linux ecosystem fit | ✓ Good |
| HID-based communication | Direct hardware control without kernel modules | ✓ Good |
| GameSense API compatibility | Existing game integration without modification | ✓ Good |
| Feature-gated audio | Optional PulseAudio dependency for broader compatibility | ✓ Good |
| 65-byte HID reports | SteelSeries hardware protocol standard | ✓ Good |
| Axum for HTTP server | Async performance with Tokio ecosystem | ✓ Good |

---
*Last updated: 2026-01-16 after project initialization*