<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# src/devices/keyboards/

## Purpose

Keyboard device implementations for the Apex series. Each file implements the `Keyboard` trait for a specific device family, handling RGB zone control, actuation point configuration, and device-specific HID quirks.

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | Keyboard module root — `Keyboard` trait definition, common keyboard types, module re-exports |
| `apex.rs` | Implementation for Apex Pro, Apex 3, Apex 5, Apex 7 (and TKL variants) — zone-based RGB, standard HID command path |
| `apex_pro_tkl_2023.rs` | Implementation for Apex Pro TKL 2023 (PID `0x1628`) — actuation point control (`0x2D`), optional direct RGB command path (`experimental-apex-2023` feature) |

## For AI Agents

### Working In This Directory

- **Apex Pro TKL 2023 PID is `0x1628`** — not `0x1618` (common mistake)
- Actuation write uses command `0x2D` (`ActuationControl`) — read-back is not yet implemented
- `apex_pro_tkl_2023.rs` has a feature-gated direct RGB path (`experimental-apex-2023`) — always compile-test both with and without this feature
- All HID writes must use `HidReportBuilder` — never construct raw byte arrays
- Keyboard reports are 65 bytes with report ID in position 0

### Device PID Reference

| Device | PID |
|--------|-----|
| Apex Pro | 0x1610 |
| Apex Pro TKL | 0x1614 |
| Apex Pro TKL (2023) | **0x1628** |
| Apex 3 | 0x161A |
| Apex 3 TKL | 0x1622 |
| Apex 5 | 0x161C |
| Apex 7 | 0x1612 |
| Apex 7 TKL | 0x1616 |

### Testing Requirements

```bash
cargo build
cargo build --features experimental-apex-2023
cargo clippy --all-targets --locked -- -D warnings
```

Physical actuation and RGB tests require a connected Apex Pro TKL 2023 (PID 0x1628).

### Common Patterns

```rust
// Actuation point write (Apex Pro TKL 2023 only)
// Uses CommandCode::ActuationControl (0x2D)
// Read-back not implemented — write-only

// Feature-gated direct RGB path
#[cfg(feature = "experimental-apex-2023")]
fn set_rgb_direct(&self, colors: &[Color]) -> Result<()> { ... }
```

## Dependencies

### Internal
- `src/devices/hid_reports.rs` — `HidReportBuilder`, `CommandCode`
- `src/devices/mod.rs` — `HidOptimizer`, `Device` trait
- `src/rgb/` — `Color`, `Effect` types
- `src/error.rs` — `Result` type

### External
- `hidapi =2.6.5`

<!-- MANUAL: -->
