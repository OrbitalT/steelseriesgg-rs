# Protocol Research Findings - Apex Pro TKL 2023

**Last Updated**: 2026-01-15
**Status**: Active Testing Phase - Bulk Command Discovery

---

## Executive Summary

This document compiles reverse engineering resources and protocol discoveries for implementing full SteelSeries Apex Pro TKL 2023 support in steelseriesgg-rs. Key findings include:

- ✅ **WINE/Lutris compatibility** - SteelSeries GG can run on Linux via Lutris
- ✅ **Existing protocol implementations** - Multiple open-source projects with working Rust/Python code
- ✅ **RGB protocol documentation** - Detailed packet structures from MSI keyboard research
- ✅ **Systematic command testing** - Automated bulk testing system implemented (0x20-0x2F range)
- ✅ **Confirmed RGB commands** - 0x21 (colors), 0x22 (brightness), 0x23 (effects)
- ✅ **OSD navigation discovered** - 0x24 brings up actuation menu
- 🔄 **Actuation control** - In progress: identifying value-setting commands
- ❌ **Rapid Trigger protocol** - Not yet tested (planned for 0x30-0x3F range)

---

## 1. Confirmed HID Protocol Commands (Apex Pro TKL 2023)

### Testing Methodology

**Device**: Apex Pro TKL 2023 (VID:0x1038, PID:0x1628)
**Approach**: Systematic bulk HID command testing with automated harness
**Test Tool**: `bulk_test` binary in `src/bin/bulk_test.rs`

### Bulk Testing Results (2026-01-15)

**Test Execution**:
- Command range: 0x20-0x2F (16 commands)
- Value patterns: 0x00, 0x04, 0x08, 0x14, 0x24, 0x30
- Total tests: 96 (16 commands × 6 values)
- Save command: 0x09 sent after each test
- Duration: ~3.4 minutes (2-second delays)
- Results: `test-results-1768466614.json`

### Confirmed Commands

| Command | Function | Status | Value Range | Notes |
|---------|----------|--------|-------------|-------|
| 0x09 | Save/apply settings | ✅ Confirmed | N/A (parameterless) | Must be sent after configuration commands |
| 0x21 | RGB zone colors | ✅ Confirmed | 3 bytes per zone (RGB) | Sets individual zone colors |
| 0x22 | RGB brightness | ✅ Confirmed | 0x00-0x64 (0-100) | Global brightness control |
| 0x23 | RGB effects | ✅ Confirmed | Effect ID + params | Changes lighting effects |
| 0x24 | OSD navigation | ✅ Confirmed | Menu position | Brings up actuation menu in keyboard OSD |

### Commands Under Investigation (0x20, 0x25-0x2F)

**Status**: Tested, awaiting observation analysis

These commands were systematically tested with value patterns but their functions have not been identified through observable keyboard behavior yet. Possible functions:
- Direct actuation point setting
- Per-key actuation configuration
- Rapid Trigger controls
- Additional OSD/menu commands
- Advanced RGB controls

**Next Steps**:
1. Manual observation review of bulk test execution
2. Annotation of `test-results-1768466614.json` with observed effects
3. Focused retesting of promising commands
4. USB capture comparison with SteelSeries GG commands

### HID Communication Pattern

**Report Structure**:
```
[Report ID: 0x00] [Command: 0xXX] [Data: variable] [Padding: to 64 bytes]
Total: 65 bytes
```

**Example Commands**:

**Set RGB Color** (Command 0x21):
```rust
[0x00, 0x21, 0xFF, R1, G1, B1, R2, G2, B2, ..., 0x00, 0x00, ...]
       ^^^^  ^^^^  ^^^^^^^^^^^^^^^^^^^^^^^
       CMD   Zone  RGB data (9 zones × 3 bytes)
```

**Set Brightness** (Command 0x22):
```rust
[0x00, 0x22, brightness, 0x00, 0x00, ...]
       ^^^^  ^^^^^^^^^^
       CMD   0-100 (decimal)
```

**Save Settings** (Command 0x09):
```rust
[0x00, 0x09, 0x00, 0x00, ...]
       ^^^^
       Parameterless save/apply command
```

**OSD Navigation** (Command 0x24):
```rust
[0x00, 0x24, value, 0x00, 0x00, ...]
       ^^^^  ^^^^^
       CMD   Position/menu parameter
```

### Actuation Point Hypothesis

**Encoding**: Likely 0.1mm increments
- 0x04 = 0.4mm (minimum)
- 0x08 = 0.8mm (default)
- 0x14 = 2.0mm
- 0x24 = 3.6mm (maximum)

**Command Pattern**: Not yet identified
- **Option 1**: Direct write command (e.g., `[0x2X, value]`)
- **Option 2**: Sequence-based (navigate with 0x24, then set with another command)
- **Option 3**: Multi-byte command (e.g., `[0x2X, zone/key, value]`)

**Discovery Strategy**:
1. ✅ Systematic testing completed (0x20-0x2F range)
2. 🔄 Manual observation analysis in progress
3. ⏳ USB capture with SteelSeries GG (if needed)
4. ⏳ Command sequence testing
5. ⏳ Focused value pattern testing on identified commands

### Testing Tools

**Bulk Test Harness** (`src/bin/bulk_test.rs`):
```bash
# Quick test (2 commands, ~6 seconds)
cargo run --release --bin bulk_test -- \
  --start 0x20 --end 0x21 --values 0x04 --delay 2 --with-save

# Full range test (96 tests, ~3.2 minutes)
cargo run --release --bin bulk_test -- \
  --start 0x20 --end 0x2F \
  --values 0x00,0x04,0x08,0x14,0x24,0x30 \
  --delay 2 --with-save

# Test both with/without save
cargo run --release --bin bulk_test -- \
  --start 0x20 --end 0x2F --values 0x04 --delay 2 \
  --with-save --test-both-modes

# Test parameterless commands
cargo run --release --bin bulk_test -- \
  --start 0x20 --end 0x2F --values 0x04 --delay 2 \
  --with-save --test-parameterless
```

**Features**:
- Hex value parsing for commands and values
- Configurable delay between tests
- With/without save command testing
- Parameterless command support
- JSON result logging with timestamp
- Manual observation annotation via `"notes"` field

**Documentation**:
- Test guide: `docs/sessions/20260115-bulk-testing-guide.md`
- Analysis: `docs/sessions/20260115-bulk-test-analysis.md`
- Progress log: `progress-bulk-testing.txt`

---

## 2. WINE Compatibility for Protocol Capture

### SteelSeries GG via Lutris

**Status**: Functional with caveats

**Installation Method**:
- Use Lutris installer: [SteelSeries GG - Lutris](https://lutris.net/games/steelseries-gg/)
- Requires additional setup from: https://github.com/asheriif/steelseries-linux-lutris

**Known Issues**:
- Wine 7.6 (April 2022) fixed installer crashes
- Device detection requires special configuration (see Lutris instructions)
- Taskbar applet must be properly closed after install for device recognition

**Benefit for Protocol Capture**:
- Allows running official SteelSeries GG software on Linux
- Enables USB traffic capture using Wireshark/usbmon while software communicates with device
- Can test and validate discovered commands against official software behavior

**References**:
- [SteelSeries GG - Lutris](https://lutris.net/games/steelseries-gg/)
- [WineHQ Forums - SteelSeries Engine 3](https://forum.winehq.org/viewtopic.php?t=30879)

---

## 2. Official Resources

### GameSense SDK (High-Level API)

**Repository**: [SteelSeries/gamesense-sdk](https://github.com/SteelSeries/gamesense-sdk)

**What It Provides**:
- HTTP-based API for game integration
- RGB per-key zone control (high-level)
- OLED screen content
- Event-driven lighting effects

**What It Doesn't Provide**:
- Low-level HID protocol details
- Direct actuation point control
- Rapid Trigger settings
- Raw USB command structures

**How to Use**:
```bash
# Find GameSense server port
cat %PROGRAMDATA%/SteelSeries/SteelSeries Engine 3/coreProps.json
# Send HTTP POST events to control RGB
curl -X POST http://127.0.0.1:27301/game_event ...
```

**Implementation Notes**:
- steelseriesgg-rs already implements GameSense server in `src/gamesense/`
- This is a higher-level abstraction; doesn't reveal underlying HID protocol

**References**:
- [GameSense Developer Portal](https://steelseries.com/developer)
- [GameSense SDK Documentation](https://github.com/SteelSeries/gamesense-sdk/tree/master/doc/api)

---

## 3. Existing Linux Implementations

### 3.1 apex-tux (Rust - OLED Support)

**Repository**: [not-jan/apex-tux](https://github.com/not-jan/apex-tux)

**Supported Devices**:
- Apex Pro (0x1610)
- Apex Pro TKL (0x1614) ⭐ *Same family as 2023 model*
- Apex 7 (0x1612)
- Apex 7 TKL (0x1618)
- Apex 5 (0x161C)

**Protocol Discoveries**:

#### USB Communication Details
```rust
// From apex-hardware/src/usb.rs
const STEELSERIES_VENDOR_ID: u16 = 0x1038;

// Product IDs
ApexProTKL: 0x1614
Apex7: 0x1612
ApexPro: 0x1610
Apex7TKL: 0x1618
Apex5: 0x161C
```

#### OLED Protocol Pattern
- **Interface**: Interface 1 (multi-interface device)
- **Report Type**: Feature reports (`send_feature_report()`)
- **Data Format**: Raw framebuffer bytes
- **Communication**: Synchronous HID writes

**Implementation Language**: Rust
**Library**: hidapi-rs

**Relevance**: Demonstrates working HID communication patterns for Apex keyboards. Uses same vendor ID and similar product IDs. OLED protocol may share structural patterns with RGB/actuation commands.

**References**:
- [apex-tux GitHub](https://github.com/not-jan/apex-tux)
- [apex-hardware crate](https://github.com/not-jan/apex-tux/tree/master/apex-hardware)

---

### 3.2 apex7tkl_linux (Python - RGB + OLED)

**Repository**: [FrankGrimm/apex7tkl_linux](https://github.com/FrankGrimm/apex7tkl_linux)

**Features Implemented**:
- Individual and region-based RGB lighting control
- Profile/configuration switching
- OLED display manipulation
- Multiple key group definitions (FKEYS, ALPHA, NUMERIC)

**Protocol Approach**:
"Raw and unpolished code" - reverse engineered through trial and error. No formal protocol documentation included, but working Python implementation reveals command structures.

**Key Modules**:
- `device.py` - HID communication layer
- `payload.py` - Command packet construction
- `colors.py` - RGB command generation
- `oled.py` - OLED protocol
- `keys.py` - Key mapping tables

**Implementation Language**: Python
**Library**: hidapi Python bindings

**Relevance**: Working RGB control for Apex 7 TKL. Apex 7 series shares architecture with Apex Pro series, so RGB commands likely have overlap.

**References**:
- [apex7tkl_linux GitHub](https://github.com/FrankGrimm/apex7tkl_linux)

---

### 3.3 msi-perkeyrgb (Python - Comprehensive Protocol Docs)

**Repository**: [Askannz/msi-perkeyrgb](https://github.com/Askannz/msi-perkeyrgb)

**Why It Matters**: MSI gaming laptops use **SteelSeries keyboards**. This project reverse-engineered the complete RGB protocol by capturing traffic between SteelSeries Engine (Windows) and the keyboard using Wireshark.

**Protocol Documentation**:
[msi-kb-effectdoc](https://github.com/Askannz/msi-perkeyrgb/blob/master/documentation/0b_packet_information/msi-kb-effectdoc)

#### RGB Protocol Structure

**Packet Types**:

1. **0x0b Packet** - Effect Definition
   - Size: 533 bytes (0x00-0x20C)
   - Defines animation effects
   - Can be assigned to multiple keys

2. **0x0e Packet** - Key Assignment
   - Maps effects and colors to individual keys
   - Structure: `[Base RGB] [Reactive RGB] [Reactive Duration] [Effect ID] [Mode] 0x00 [Keycode]`

#### 0x0b Effect Packet Layout

| Byte Range | Purpose | Notes |
|------------|---------|-------|
| 0x00-0x01 | Packet header | `[0x0b 0x00]` |
| 0x02-0x81 | Transition blocks | 16 max, 8 bytes each |
| 0x82-0x83 | Filler | `[0x00 0x00]` |
| 0x84-0x89 | Starting color | RGB, 2 bytes each (nibble-swapped) |
| 0x8a-0x8b | Separator | `[0xFF 0x00]` |
| 0x8c-0x95 | Wave mode params | Origin, direction, wavelength |
| 0x96 | Transition count | Max 16 |
| 0x97 | Separator | `[0x00]` |
| 0x98-0x99 | Effect duration | Milliseconds, little-endian |
| 0x9a | Wave direction | 0=outward, 1=inward |
| 0x9b-0x20C | Padding | All zeros |

#### Color Encoding (Nibble Swap)
```python
# Unusual byte ordering
encoded_color = (color & 0x0F) << 4 + (color & 0xF0) >> 4
```

#### 0x0e Key Assignment Structure
```
[Base RGB (3 bytes)]
[Reactive RGB (3 bytes)]
[Reactive Duration (2 bytes)]
[Effect ID (1 byte)]
[Mode (1 byte)]
[0x00]
[Keycode (1 byte)]
```

**Mode Values**:
- `0` - Effect without refresh
- `1` - Inline static color
- `2` - Effect with auto-refresh
- `8` - Reactive (responds to key press)

**Reactive Duration**: 100-1000ms in vendor software

**Implementation Language**: Python
**Library**: hidapi

**Relevance**: Most detailed RGB protocol documentation found. While specific byte values may differ for Apex Pro TKL 2023, the packet structure and general approach are likely similar.

**References**:
- [msi-perkeyrgb GitHub](https://github.com/Askannz/msi-perkeyrgb)
- [Protocol Documentation](https://github.com/Askannz/msi-perkeyrgb/blob/master/documentation/0b_packet_information/msi-kb-effectdoc)

---

## 4. Product ID Mapping

### Known Apex Keyboard Product IDs

| Model | Product ID | Notes |
|-------|-----------|-------|
| Apex Pro | 0x1610 | Full-size |
| Apex Pro TKL | 0x1614 | Original TKL |
| Apex 7 | 0x1612 | |
| Apex 7 TKL | 0x1618 | |
| Apex 5 | 0x161C | |
| **Apex Pro TKL 2023** | **0x1628** | **Target device** |

**Vendor ID**: 0x1038 (all SteelSeries devices)

**Interface Numbers**:
- Interface 0: Keyboard HID (standard keypress reporting)
- Interface 1: Control interface (RGB, actuation, OLED)
- Interface 2-3: Additional interfaces (device-specific)

**Note**: Apex Pro TKL 2023 (0x1628) is NOT documented in existing projects. Protocol must be discovered independently, but can build on patterns from 0x1614 (original Apex Pro TKL).

---

## 5. Actuation & Rapid Trigger Protocol

### Current Status: Unknown

**Official Documentation**: None available
**Community Research**: None found publicly

### What We Know (Feature Behavior)

**Actuation Point Adjustment**:
- Range: 0.4mm to 3.6mm (per SteelSeries specs)
- Can be set globally or per-key
- Uses OmniPoint Hall Effect sensors
- Measurement precision: ~0.1mm increments

**Rapid Trigger**:
- Firmware update (2023) added this feature
- Eliminates fixed reset point
- Key releases as soon as movement detected upward
- Response time: 0.54ms (claimed)
- 11x faster than traditional mechanical switches

### Discovery Strategy

#### Method 1: WINE + Wireshark Capture
1. Install SteelSeries GG via Lutris
2. Configure device detection (per community guides)
3. Start Wireshark/usbmon capture on Linux
4. Change actuation settings in SteelSeries GG
5. Analyze USB traffic for command patterns

**Tools**:
```bash
# USB capture
sudo modprobe usbmon
sudo wireshark
# Filter: usb.device_address == X && usb.src == "host"

# Or use usbmon directly
sudo cat /sys/kernel/debug/usb/usbmon/1u > capture.log
```

#### Method 2: Systematic HID Command Testing
1. Start with known RGB command structures (0x0b, 0x0e patterns)
2. Test variations with different command bytes
3. Monitor keyboard behavior (actuation feel, rapid trigger responsiveness)
4. Document successful commands

**Safety**: Avoid firmware-related command ranges (likely 0xF0+). Stick to configuration commands.

#### Method 3: Firmware Analysis (Advanced)
- Extract firmware binary (if accessible via USB)
- Use tools like Ghidra for reverse engineering
- Look for command parsing routines

**Risk**: High complexity, low success rate without insider knowledge

---

## 6. Recommended Implementation Plan

### Phase 1: Setup & Validation
1. ✅ Install Lutris + SteelSeries GG (via WINE)
2. ✅ Verify device detection in official software
3. ✅ Set up usbmon/Wireshark for USB capture
4. Test existing RGB commands from steelseriesgg-rs

### Phase 2: RGB Protocol Reverse Engineering
1. Capture USB traffic during RGB changes (SteelSeries GG → device)
2. Compare with known protocol structures (msi-perkeyrgb, apex-tux)
3. Test captured commands via steelseriesgg-rs
4. Document working commands in PROTOCOL.md
5. Extend RGB implementation in `src/rgb/` and `src/devices/keyboards/`

### Phase 3: Actuation Protocol Discovery
1. Capture USB traffic during actuation adjustments
2. Identify command byte patterns
3. Test commands systematically on hardware
4. Validate actuation changes (physical testing)
5. Implement API in `Keyboard` trait

### Phase 4: Rapid Trigger Protocol Discovery
1. Capture USB traffic during Rapid Trigger toggle
2. Identify enable/disable commands
3. Test on hardware (rapid keypress tests)
4. Document protocol
5. Implement API in `Keyboard` trait

### Phase 5: Integration & Testing
1. Add profile persistence for new features
2. Create CLI commands
3. Write integration tests
4. Update documentation

---

## 7. Key Insights for Implementation

### USB Communication Patterns (From Existing Projects)

**Common Patterns**:
- **Interface 1** used for control commands (RGB, OLED)
- **Feature reports** (`send_feature_report()`) for configuration
- **Output reports** (`write()`) for real-time effects
- **Report ID** usually 0x00 for SteelSeries devices
- **Padding** to fixed packet sizes (65 bytes typical)

**HID Report Structure** (General):
```
[Report ID: 1 byte]
[Command Byte: 1 byte]
[Data Payload: N bytes]
[Padding: to 65 bytes total]
```

### Byte Order Considerations
- MSI/SteelSeries RGB uses **nibble swapping** for colors
- Duration values are **little-endian**
- Multi-byte values need endianness testing

### Command Discovery Heuristics
Based on MSI protocol:
- `0x0b` = RGB effect definition
- `0x0d` = Preview/save (?)
- `0x0e` = Key assignment

**Hypothesis**: Apex Pro TKL 2023 may use:
- `0x0A-0x0F` range for RGB/lighting commands
- `0x20-0x2F` range for actuation commands (unknown)
- `0x30-0x3F` range for Rapid Trigger (unknown)

*These are speculative - requires testing!*

---

## 8. Existing Code in steelseriesgg-rs

### Current RGB Implementation (src/devices/keyboards/generic.rs)

```rust
pub fn set_color(&mut self, zone: u8, color: Color) -> Result<()> {
    let mut report = vec![0u8; 65];
    report[0] = 0x00; // Report ID
    report[1] = 0x21; // Command byte (RGB)
    // ... color data
    write_padded_report(&self.device, &report, 65, true)?;
    Ok(())
}
```

**Current Command Byte**: `0x21` for RGB commands

**Next Steps**:
1. Verify if `0x21` works for Apex Pro TKL 2023 (product ID 0x1628)
2. Test if different command bytes exist for:
   - Static color (`0x21`)
   - Effects (`0x22`?)
   - Per-key RGB (`0x23`?)
   - Actuation (`0x30`??)
   - Rapid Trigger (`0x40`??)

---

## 9. Testing Checklist

### Hardware Testing Requirements
- [ ] Apex Pro TKL 2023 connected via USB
- [ ] Baseline RGB command works (`ssgg rgb --color red`)
- [ ] usbmon/Wireshark capture configured
- [ ] SteelSeries GG installed in WINE/Lutris
- [ ] Device detected by official software

### Capture Workflow
1. Start USB capture
2. Make ONE change in SteelSeries GG (e.g., set actuation to 1.0mm)
3. Stop capture
4. Filter packets by device address
5. Identify HID output/feature reports from host → device
6. Extract hex data
7. Test command in steelseriesgg-rs

### Validation Tests
- **RGB**: Visual confirmation (color changes)
- **Actuation**: Physical testing (key press feel at different distances)
- **Rapid Trigger**: Rapid keypress test (measure repeat rate)

---

## 10. Safety Considerations

### What to Avoid
- ❌ Commands in `0xF0-0xFF` range (likely firmware/bootloader)
- ❌ Sending random data without validation
- ❌ Overwriting device configuration without backup
- ❌ Testing without ability to restore defaults

### Safety Practices
- ✅ Test on non-critical device if possible
- ✅ Document default settings before changes
- ✅ Start with read-only operations (queries)
- ✅ Verify commands are reversible
- ✅ Keep official software available for recovery

### Recovery Plan
1. If device becomes unresponsive:
   - Unplug USB
   - Wait 10 seconds
   - Reconnect
   - Use SteelSeries GG to restore defaults

2. If settings are corrupted:
   - Use official software to reset to factory defaults
   - Test with official software first before retrying custom commands

---

## 11. Next Actions

### Immediate Next Steps
1. **Update PRD.md** with WINE/Lutris approach
2. **Set up capture environment**:
   - Install Lutris + SteelSeries GG
   - Configure usbmon/Wireshark
   - Test device detection
3. **Begin RGB capture** (baseline before actuation work)
4. **Clone reference repositories** for code study:
   - `git clone https://github.com/not-jan/apex-tux`
   - `git clone https://github.com/Askannz/msi-perkeyrgb`

### Development Workflow
1. Capture → Analyze → Test → Document → Implement
2. Focus on one feature at a time (RGB → Actuation → Rapid Trigger)
3. Commit protocol findings to `docs/PROTOCOL.md` incrementally
4. Update steelseriesgg-rs implementation as commands are validated

---

## 12. References & Resources

### Official Resources
- [GameSense Developer Portal](https://steelseries.com/developer)
- [GameSense SDK GitHub](https://github.com/SteelSeries/gamesense-sdk)
- [Apex Pro TKL 2023 Manual](https://support.steelseries.com/hc/en-us/articles/37474028182541-Apex-Pro-TKL-2023-Manual-and-Product-Information-Guide)
- [How to Change Actuation Point](https://support.steelseries.com/hc/en-us/articles/9645931478029)
- [How to Activate Rapid Trigger](https://support.steelseries.com/hc/en-us/articles/18742108024717)

### Community Projects
- [apex-tux (Rust)](https://github.com/not-jan/apex-tux)
- [apex7tkl_linux (Python)](https://github.com/FrankGrimm/apex7tkl_linux)
- [msi-perkeyrgb (Python + Protocol Docs)](https://github.com/Askannz/msi-perkeyrgb)
- [apexctl (Rust)](https://github.com/AstroSnail/apexctl)

### WINE/Lutris
- [SteelSeries GG - Lutris](https://lutris.net/games/steelseries-gg/)
- [SteelSeries Linux Lutris Setup](https://github.com/asheriif/steelseries-linux-lutris)
- [WineHQ - SteelSeries Engine Issues](https://forum.winehq.org/viewtopic.php?t=30879)

### Protocol Analysis Resources
- [MSI Keyboard Effect Protocol](https://github.com/Askannz/msi-perkeyrgb/blob/master/documentation/0b_packet_information/msi-kb-effectdoc)
- [USB HID Protocol Specification](https://www.usb.org/sites/default/files/documents/hid1_11.pdf)
- [Linux HID Introduction](https://docs.kernel.org/hid/hidintro.html)

---

**Document Version**: 1.0
**Contributors**: Research compiled from web sources and existing open-source projects
**License**: MIT (same as steelseriesgg-rs project)
