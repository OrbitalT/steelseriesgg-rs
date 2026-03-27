<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# docs/development/

## Purpose

Protocol research and reverse-engineering documentation for SteelSeries HID communication. Contains findings from USB traffic analysis, command byte mappings, and RGB control protocol notes.

## Key Files

| File | Description |
|------|-------------|
| `APEX_PRO_PROTOCOL.md` | Apex Pro HID protocol — command codes, report format, actuation control (`0x2D`) findings |
| `KEY_MAPPING_RESEARCH.md` | Research notes on keyboard key-to-HID-code mapping for per-key RGB |
| `PROTOCOL_RESEARCH.md` | General SteelSeries HID protocol research — vendor commands, report structure |
| `RGB_CONTROL_ANALYSIS.md` | RGB control analysis — zone byte positions, color encoding, effect commands |

## For AI Agents

### Working In This Directory

- These are **reference documents** — cross-reference with `src/devices/hid_reports.rs` before trusting them
- `hid_reports.rs` is the **source of truth** for current protocol implementation; docs may lag
- Update these docs when new protocol behavior is discovered or verified via hardware testing
- Do NOT implement protocol changes based solely on these docs — verify against actual device behavior first

### Common Patterns

- Protocol docs use hex byte notation: `0x2D`, `0xFF00`, etc.
- Report structures are described with byte offsets (0-indexed)
- "WIP" or "TBD" sections indicate unverified/incomplete reverse-engineering

<!-- MANUAL: -->
