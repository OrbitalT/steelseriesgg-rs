# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
Releases and this changelog are generated automatically from
[Conventional Commits](https://www.conventionalcommits.org/) by release-please.

## [0.1.0](https://github.com/Ven0m0/steelseriesgg-rs/releases/tag/v0.1.0) (2026-05-31)

Initial release of the open-source SteelSeries GG replacement for Linux.

### Added

- Type-safe HID report builder for talking to SteelSeries devices instead of
  hand-written byte arrays.
- RGB lighting control: colors, effects, per-key effects, and zone-to-HID
  mapping.
- Apex keyboard protocol implementation and headset protocol support.
- Device discovery with hot-plug detection and device fingerprinting.
- GameSense-compatible HTTP server on port 27301 with a localhost-only CORS
  policy.
- Profile save/load as TOML and `~/.config/ssgg/config.toml` configuration.
- Runtime diagnostics, RGB validation, and performance management.
- Optional `audio` feature: PulseAudio/PipeWire mixer.
- Optional `sonar` feature: SteelSeries Sonar HTTP integration.
- Experimental `experimental-apex-2023` feature for Apex Pro TKL 2023 direct
  per-key RGB (reverse-engineered, unverified on hardware).
- udev rules and a systemd user unit for installation.
