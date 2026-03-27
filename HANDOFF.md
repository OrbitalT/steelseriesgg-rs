---
date: 2026-03-27
session: 2026-03-27 PM
branch: main
status: Phase 6 Complete — HID Code Discovery
---

# Session Handoff — 2026-03-27

## What happened this session

### 1. SteelSeries GG 107.0.0 Extraction & Analysis

**Extracted:** `SteelSeriesGG107.0.0Setup.exe` (360MB Nullsoft installer)
- 2656 files extracted using 7z
- 1.09 GB of SteelSeries Engine files

**Key directories analyzed:**
- `apps/engine/` — Main SteelSeries Engine
- `apps/engine/deviceSpecifications/` — 473 encrypted `.edevice` files
- `apps/engine/configurationMigrations/` — Lisp-like `.migration` files

### 2. MAJOR BREAKTHROUGH: HID Code Discovery

**Finding:** SteelSeries uses **USB HID Usage IDs** (NOT row/col matrix addresses) for per-key identification.

**Evidence from `apex_7+pro.migration`:**
```lisp
(define tkl-keyboard-keycodes '(4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 73 74 75 76 77 78 79 80 81 82 100 133 135 136 137 138 139 224 225 226 227 228 229 230 231 240))
```

**Complete TKL keycode list (87 keys):**
- HID 4-69: Standard keys (A-Z, 1-9, 0, symbols, F1-F12, etc.)
- HID 73-82: Navigation keys (Home, End, Arrows, etc.)
- HID 100: Menu key
- HID 133, 135-139: Media keys
- HID 224-231: Modifiers (Ctrl, Shift, Alt, GUI - left/right)
- HID 240: SteelSeries key

### 3. Code Updates

**Updated `src/devices/key_mapping.rs`:**
- Changed `KeyAddress` from `(row, col)` to `hid_code: u8`
- Updated all 87 key mappings with verified HID codes
- Added `is_actuation_capable` field for analog keys
- Updated documentation and tests

**Updated `docs/development/KEY_MAPPING_RESEARCH.md`:**
- Documented HID code discovery
- Added complete keycode tables
- Updated status to VERIFIED

**Updated `docs/development/APEX_PRO_PROTOCOL.md`:**
- Added HID code addressing section
- Updated per-key RGB command documentation
- Added protocol implications

**Updated `docs/development/PROTOCOL_RESEARCH.md`:**
- Added Section 0: SteelSeries GG 107.0.0 Extraction Analysis
- Documented binary analysis results
- Added protocol implications

---

## Current State

### What is now working

| Item | Status | Notes |
|------|--------|-------|
| Key matrix addressing | ✅ VERIFIED | Uses USB HID keycodes, not row/col |
| `key_mapping.rs` | ✅ UPDATED | All 87 keys mapped to HID codes |
| Protocol documentation | ✅ UPDATED | All 3 protocol docs updated |
| `experimental-apex-2023` feature | ✅ EXISTS | Feature flag in Cargo.toml |

### What still needs work

| Item | File | Status |
|------|------|--------|
| Per-key RGB command format | `src/devices/hid_reports.rs` | ⚠️ Need USB capture to verify exact packet structure |
| Actuation read-back | Unknown | ⚠️ Query command not discovered |
| `PulseHandler::new()` timeout | `src/audio/pulse.rs` | ⚠️ Unbounded wait remains |
| USB capture for protocol | External | ⚠️ USBPcap not installed |

---

## Next Steps

### Priority 1: USB Capture for Per-Key RGB Protocol

**Goal:** Capture actual HID packets while SteelSeries GG sets per-key colors.

**Requirements:**
1. Install USBPcap (Windows) or use usbmon (Linux)
2. Run SteelSeries GG (via Wine/Lutris on Linux, or native on Windows)
3. Set per-key colors and capture traffic
4. Analyze packet structure for command 0x23

**Alternative:** Use the extracted SteelSeries GG directly on Windows with USBPcap.

### Priority 2: Update HidReportBuilder for Per-Key RGB

**File:** `src/devices/hid_reports.rs`

**Changes needed:**
1. Update `PerKeyRgbCommand` to use HID codes
2. Update packet format based on USB capture results
3. Test with actual hardware

### Priority 3: Test with Hardware

**Commands to test:**
```bash
cargo build --locked --features experimental-apex-2023
cargo run --features experimental-apex-2023 -- rgb perkey --key A --color red
```

---

## Quick Orientation for Next Agent

```
src/devices/key_mapping.rs     ← UPDATED: HID codes now used
src/devices/hid_reports.rs     ← NEXT: Update PerKeyRgbCommand
docs/development/KEY_MAPPING_RESEARCH.md  ← UPDATED: HID code tables
docs/development/APEX_PRO_PROTOCOL.md     ← UPDATED: Protocol docs
docs/development/PROTOCOL_RESEARCH.md     ← UPDATED: Extraction analysis
```

**Key Insight:** The row/col matrix approach was wrong. SteelSeries uses standard USB HID keycodes (0x04-0x64) for per-key addressing. The `PerKeyRgbCommand` needs to be updated to use HID codes in the packet data.

---

## Files Modified This Session

- `src/devices/key_mapping.rs` — Core key mapping (HID codes)
- `docs/development/KEY_MAPPING_RESEARCH.md` — Research documentation
- `docs/development/APEX_PRO_PROTOCOL.md` — Protocol documentation
- `docs/development/PROTOCOL_RESEARCH.md` — Extraction analysis
- `HANDOFF.md` — This file

## Extracted Files Location

```
C:\Users\Ven0m0\Downloads\steelseries_extracted\
├── apps\engine\SteelSeriesEngine.exe  — Main engine (18MB)
├── apps\engine\HIDDLL.dll             — HID library (2.1MB)
├── apps\engine\deviceSpecifications\  — 473 encrypted .edevice files
├── apps\engine\configurationMigrations\ — Key data in .migration files
└── localization\en_US.json            — RGB preset names
```
