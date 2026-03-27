<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# src/rgb/

## Purpose

RGB control engine — defines `Color`, `Effect`, `EffectEngine`, and `RgbController`. Handles color computation, effect animation state, and zone-based RGB control for all supported keyboards.

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | `Color` (RGB/named), `Effect` (static/breathing/spectrum/wave/off), `EffectEngine` (per-device effect computation with caching), `RgbController` (zone-based control), `PerKeyRgbController` (Apex Pro TKL 2023, feature-gated) |
| `tests.rs` | Unit tests for color parsing, effect computation, and RGB controller logic |

## For AI Agents

### Working In This Directory

- `EffectEngine` caches computed frame values — invalidate cache when effect parameters change
- `PerKeyRgbController` is only compiled with `experimental-apex-2023` feature — always gate with `#[cfg(feature = "experimental-apex-2023")]`
- Named colors are parsed case-insensitively (e.g., `"red"`, `"RED"`, `"#FF0000"`)
- CLI RGB commands are **one-shot** — continuous animation requires `ssgg daemon`

### Testing Requirements

```bash
cargo test                          # includes rgb tests
cargo test rgb                      # run rgb-specific tests only
```

### Common Patterns

```rust
// Parse color from CLI input
let color = Color::from_str("red")?;      // named
let color = Color::from_str("#FF0000")?;  // hex

// Zone-based RGB
let controller = RgbController::new(device);
controller.set_zone_color(zone, color)?;

// Effect
let effect = Effect::Breathing { color, speed: 50 };
engine.apply_effect(device, effect)?;
```

## Dependencies

### Internal
- `src/devices/hid_reports.rs` — RGB commands sent via `HidReportBuilder`
- `src/devices/zone_mapping.rs` — maps zone names to report byte positions
- `src/error.rs` — `Result` type

### External
- No external crates specific to this module

<!-- MANUAL: -->
