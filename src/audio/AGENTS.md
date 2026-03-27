<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# src/audio/

## Purpose

Audio mixer integration — feature-gated module providing PulseAudio volume control and SteelSeries Sonar HTTP API client. Only compiled when `audio` or `sonar` features are enabled.

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | `AudioMixer` trait, `Channel` enum (Game/Chat/Media/Aux/Mic), module re-exports |
| `pulse.rs` | PulseAudio integration via `libpulse-binding` — per-channel volume control for SteelSeries headsets |
| `sonar.rs` | `SonarClient` — HTTP client for SteelSeries Sonar Windows API (`reqwest`, `sonar` feature only) |

## For AI Agents

### Working In This Directory

- This entire module is **feature-gated** — wrap all code in `#[cfg(feature = "audio")]` or `#[cfg(feature = "sonar")]` as appropriate
- `pulse.rs` requires `libpulse-dev` system library — CI uses default features (no `audio`) to avoid this dependency
- `sonar.rs` requires `--features sonar` — uses `reqwest` for HTTP calls to Sonar API
- Do NOT add `libpulse-binding` or `reqwest` calls outside their respective feature gates

### Testing Requirements

```bash
# Test with audio feature (requires libpulse-dev)
cargo test --features audio

# Test with sonar feature
cargo test --features sonar
```

### Common Patterns

```rust
// Feature-gated usage in lib.rs
#[cfg(any(feature = "audio", feature = "sonar"))]
pub mod audio;

// Channel enum
let volume = mixer.get_volume(Channel::Game)?;
mixer.set_volume(Channel::Chat, 75)?;
```

## Dependencies

### Internal
- `src/error.rs` — `Result` type

### External
- `libpulse-binding 2.30.1` — PulseAudio (optional, `audio` feature)
- `reqwest 0.13.2` — HTTP client for Sonar API (optional, `sonar` feature)
- `tokio` — async runtime for `reqwest`

<!-- MANUAL: -->
