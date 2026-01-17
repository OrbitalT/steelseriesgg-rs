# SteelSeries Apex Pro HID Protocol Documentation

## Overview

This document details the HID communication protocol for SteelSeries Apex Pro keyboards, focusing on RGB lighting control and device communication patterns.

**Last Updated:** 2026-01-16
**Target Devices:** Apex Pro series keyboards
**Protocol Version:** Based on reverse engineering and existing implementation analysis

---

## Product ID Mapping

### Currently Supported Apex Pro Variants

| Model | Product ID | Zone Count | Current Status |
|-------|------------|------------|----------------|
| Apex Pro | `0x1610` | 1 (single zone) | Basic support |
| Apex Pro TKL | `0x1614` | 1 (single zone) | Basic support |
| Apex Pro TKL (2023) | `0x1628` | 9 zones | Enhanced support |

**Note:** The 2023 variant (`0x1628`) appears to be the primary hardware in use based on codebase comments.

### USB Interface Configuration

- **Vendor ID:** `0x1038` (SteelSeries)
- **Control Interface:** Interface 1 (keyboards use interface 1, headsets use interface 3)
- **Report Size:** 65 bytes (1 byte report ID + 64 bytes data)
- **Communication Type:** HID Output Reports

---

## Current HID Protocol Implementation

### Report Structure

All HID reports follow this format:
```
[Report ID: 1 byte] [Command: 1 byte] [Data: 63 bytes]
Total: 65 bytes (padded with zeros)
```

### Known Commands

#### RGB Zone Control (0x21)
**Purpose:** Set RGB colors for keyboard zones

**Format:**
```
[0x00] [0x21] [0xFF] [R1] [G1] [B1] [R2] [G2] [B2] ... [padding]
```

**Details:**
- Report ID: `0x00`
- Command: `0x21` (RGB control)
- Subcommand: `0xFF` (set zone colors)
- Color data: 3 bytes per zone (R, G, B)
- Supports up to 9 zones for Apex Pro TKL (2023)

**Current Implementation:** `GenericKeyboard::set_zone_colors()`

#### Brightness Control (0x22)
**Purpose:** Set overall keyboard brightness

**Format:**
```
[0x00] [0x22] [brightness] [padding...]
```

**Details:**
- Report ID: `0x00`
- Command: `0x22` (brightness)
- Brightness: 0-100 (percentage)

**Current Implementation:** `GenericKeyboard::set_brightness()`

#### Apply/Save Settings (0x09)
**Purpose:** Commit RGB settings to device

**Format:**
```
[0x00] [0x09] [padding...]
```

**Details:**
- Report ID: `0x00`
- Command: `0x09` (save/apply)
- Used in initialization and apply operations

**Current Implementation:** `GenericKeyboard::apply()` and `GenericKeyboard::initialize()`

---

## Zone Mapping (Current Implementation)

### Apex Pro TKL (2023) - 9 Zone Layout

Current zone enumeration from `ApexZone`:

| Zone Index | Zone Name | Physical Area | Enum Value |
|------------|-----------|---------------|------------|
| 0 | Left | Left side keys | `ApexZone::Left` |
| 1 | LeftCenter | Left-center area | `ApexZone::LeftCenter` |
| 2 | Center | Center area | `ApexZone::Center` |
| 3 | RightCenter | Right-center area | `ApexZone::RightCenter` |
| 4 | Right | Right side keys | `ApexZone::Right` |
| 5 | FunctionRow | F1-F12 keys | `ApexZone::FunctionRow` |
| 6 | NumberRow | 1-9, 0 keys | `ApexZone::NumberRow` |
| 7 | Wasd | WASD cluster | `ApexZone::Wasd` |
| 8 | ArrowKeys | Arrow key cluster | `ApexZone::ArrowKeys` |

---

## Identified Gaps for Per-Key Control

### Current Limitations

1. **Zone-Only Control:** Current implementation only supports zone-based RGB, not per-key
2. **Limited Command Set:** Only basic RGB, brightness, and save commands documented
3. **No Per-Key Addressing:** Missing individual key addressing scheme
4. **No Read Commands:** Cannot query current device state
5. **Protocol Variations:** Differences between Apex Pro variants not fully documented

### Missing Protocol Elements

#### Per-Key RGB Commands
- **Unknown:** Command bytes for individual key addressing
- **Unknown:** Key mapping table (physical key → HID address)
- **Unknown:** Per-key command format and structure
- **Unknown:** Batch per-key update mechanisms

#### Device Query Commands
- **Unknown:** Read current RGB state
- **Unknown:** Query device capabilities
- **Unknown:** Read actuation point settings (Apex Pro feature)
- **Unknown:** Device mode queries

#### Advanced Features
- **Unknown:** Reactive lighting protocol
- **Unknown:** Audio-reactive commands
- **Unknown:** Profile switching commands
- **Unknown:** Macro programming interface (if supported)

---

## Research Findings

### Working Commands (Confirmed)

✅ **RGB Zone Control (0x21, 0xFF)** - Working for zone-based lighting
✅ **Brightness Control (0x22)** - Working for global brightness
✅ **Apply Settings (0x09)** - Working for committing changes

### Known Issues

❌ **Random Key Targeting** - RGB commands sometimes affect unintended keys
❌ **Inconsistent Zone Response** - Some zones respond intermittently
❌ **Command Reliability** - Commands sometimes fail without error indication

### Hypothesis for Issues

1. **Missing Command Headers:** Per-key commands may require different header bytes
2. **Timing Issues:** Device may need delays between commands
3. **Checksum Requirements:** Commands may require validation bytes
4. **Mode Switching:** Device may need to be put in specific mode for per-key control
5. **Interface Selection:** May need different USB interface for per-key operations

---

## Implementation Status

### Current State (Zone-Based)
```rust
// Working: Zone-based RGB control
keyboard.set_zone_colors(&[Color::RED, Color::GREEN, Color::BLUE]); // ✅

// Working: Global operations
keyboard.set_brightness(80); // ✅
keyboard.apply(); // ✅
```

### Target State (Per-Key)
```rust
// Goal: Per-key RGB control
keyboard.set_key_color("Q", Color::RED); // ❌ Not implemented
keyboard.set_multiple_keys(key_map); // ❌ Not implemented

// Goal: Individual key addressing
let key_addr = keyboard.get_key_address("W"); // ❌ Not implemented
keyboard.send_per_key_command(key_addr, Color::BLUE); // ❌ Not implemented
```

---

## Next Steps for Protocol Discovery

### Priority Research Areas

1. **Per-Key Command Discovery**
   - Analyze official SteelSeries Engine communication
   - Reverse engineer per-key RGB packets
   - Document command format differences

2. **Key Addressing Scheme**
   - Map physical keys to HID addresses
   - Document layout variations (ANSI/ISO)
   - Create comprehensive key mapping tables

3. **Command Reliability**
   - Investigate timing requirements
   - Test checksum/validation needs
   - Implement robust error handling

4. **Device Mode Management**
   - Discover mode switching commands
   - Document initialization sequences
   - Test compatibility across Apex Pro variants

### Research Methods

- **Packet Capture:** Monitor SteelSeries Engine ↔ device communication
- **Command Fuzzing:** Systematically test command variations
- **Reverse Engineering:** Analyze official driver behavior
- **Hardware Testing:** Validate commands on actual devices

---

## References

### Current Implementation Files
- `src/devices/mod.rs` - Product ID constants and zone mapping
- `src/devices/keyboards/mod.rs` - Generic keyboard HID implementation
- `src/devices/keyboards/apex.rs` - Apex-specific features and zone definitions
- `src/devices/discovery.rs` - Device enumeration and interface selection

### Protocol Constants
```rust
// Product IDs
const APEX_PRO: u16 = 0x1610;
const APEX_PRO_TKL: u16 = 0x1614;
const APEX_PRO_TKL_2023: u16 = 0x1628;

// Commands
const CMD_RGB_ZONE: u8 = 0x21;
const CMD_BRIGHTNESS: u8 = 0x22;
const CMD_APPLY: u8 = 0x09;

// Interface
const KEYBOARD_CONTROL_INTERFACE: i32 = 1;
const HID_REPORT_SIZE: usize = 65;
```

### External Resources
- SteelSeries Engine (proprietary) - Reference implementation
- USB HID 1.11 Specification - Protocol standards
- Community reverse engineering efforts - Protocol discoveries

---

*This document will be updated as protocol research progresses and new commands are discovered.*