# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Per-key RGB support for Apex Pro TKL 2023 via `Apex2023Direct (0x40)` opcode (experimental, feature-gated behind `experimental-apex-2023`)
- GameSense-compatible HTTP server on port 27301 with strict localhost-only CORS policy
- Windows polling rate query via `IOCTL_HID_GET_POLL_FREQUENCY_MSEC` with clear error messaging for USB HID devices that don't support the IOCTL
- Secure file write utilities (`fs_utils::secure_write`, `fs_utils::secure_write_async`) that refuse to follow symlinks
- Device diagnostics with runtime HID operation metrics
- Hot-plug device discovery via `devices::discovery`
- Support for Arctis Nova Pro Omni PID `0x2290`
- Support for Apex Pro TKL 2023 WIRELESS_2 PID `0x1630`
- Arch Linux PKGBUILD for AUR packaging
- Audio hang protection: 5-second timeout in `PulseHandler::new()`
- `ValidationReport` via `RgbValidator` for config validation
- `PerformanceManager` for device-level performance metrics
- `DeviceStateStore::update_states_with` for in-place state updates avoiding unnecessary clones

### Fixed
- GameSense CORS origin validation: strict `localhost`/`127.0.0.1` check, rejecting all other origins
- Insecure file write in benchmark and performance report output paths
- `send_feature_report` on non-Unix platforms now routes through hidapi instead of a no-op
- GameSense server insecure write fallback in stats export
- Apex Pro TKL 2023: include `WIRELESS_2 (0x1630)` in per-zone RGB buffer path

### Performance
- Optimized GameSense gradient color calculation using integer arithmetic instead of float blending per-request
- `EffectEngine::new` no longer clones the initial `Effect` unnecessarily
- CPU sampling I/O migrated to `tokio::fs` to avoid blocking the async runtime
- `MemorySample::new` moved to async I/O to avoid blocking in async context
- Final device state save avoids cloning `Effect` (which can hold `Vec<Color>`) when state is unchanged
- String conversion in device tests avoids intermediate allocation

### Changed
- Pinned `hidapi` to `=2.6.6` for ABI stability
- Consolidated four redundant protocol research docs into `docs/development/protocol-keyboard.md`
- Removed large binary capture files (~70 MB) from repository history; captures now gitignored

### Dependencies
- `hidapi` → `2.6.6`
- `reqwest` → `0.13.4`
- `serde_json` → `1.0.150`
- `sysinfo` → `0.39.0`

## [0.1.0] - Initial Development

Initial project scaffold: RGB lighting control, GameSense HTTP server stub, HID report builder, profile save/load, per-key effect types, and device discovery framework.

[Unreleased]: https://github.com/Ven0m0/steelseriesgg-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Ven0m0/steelseriesgg-rs/releases/tag/v0.1.0
