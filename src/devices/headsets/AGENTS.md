<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# src/devices/headsets/

## Purpose

Headset device implementations for the Arctis series. Implements the `Headset` trait for audio control (volume, EQ, sidetone) and any RGB lighting on supported headsets via HID communication.

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | `Headset` trait definition and implementations for all Arctis models — audio channel control, sidetone, battery status |

## For AI Agents

### Working In This Directory

- Headset HID reports are **64 bytes with no report ID** (unlike keyboard's 65 bytes with report ID) — do not confuse the two
- Use `HEADSET_REPORT_SIZE` constant from `src/devices/hid_reports.rs`
- All HID writes use `HidReportBuilder` — never raw byte arrays
- Audio control (volume, EQ) is separate from HID headset control — see `src/audio/` for PulseAudio integration

### Supported Headsets

Arctis 1, Arctis 1 Wireless, Arctis 5, Arctis 7 (2019), Arctis 9, Arctis Pro, Arctis Pro Wireless, Arctis Nova Pro, Arctis Nova Pro Wireless, Arctis Nova 5, Arctis Nova 3, Arctis Nova 1.

### Testing Requirements

```bash
cargo build
cargo clippy --all-targets --locked -- -D warnings
```

Physical headset tests require a connected Arctis device.

### Common Patterns

```rust
// Headset report — 64 bytes, no report ID
// HEADSET_REPORT_SIZE = 64
let report = HidReportBuilder::new_headset()
    .command(CommandCode::SetSidetone)
    .byte(level)
    .build()?;
device.write(&report)?;
```

## Dependencies

### Internal
- `src/devices/hid_reports.rs` — `HidReportBuilder`, `HEADSET_REPORT_SIZE`
- `src/devices/mod.rs` — `Device` trait, `HidOptimizer`
- `src/error.rs` — `Result` type

### External
- `hidapi =2.6.5`

<!-- MANUAL: -->
