# PRD: Reverse Engineer Apex Pro TKL 2023 Protocol

## Introduction

This project aims to reverse engineer the complete HID protocol for the SteelSeries Apex Pro TKL 2023 keyboard to enable full feature support in steelseriesgg-rs on Linux. The keyboard's advanced features (OmniPoint actuation adjustment, Rapid Trigger, expanded RGB control) are currently unavailable on Linux due to lack of protocol documentation. By capturing and analyzing HID reports through multiple methods (WINE/Lutris running official software, usbmon captures, existing open-source implementations, and systematic testing), we will implement these features natively in Rust.

**See PROTOCOL_RESEARCH.md for detailed findings from existing projects and protocol documentation.**

## Goals

- Reverse engineer the HID command protocol for Apex Pro TKL 2023 (product ID: 0x1628)
- Document all discovered HID commands with byte-level specifications
- Implement actuation point adjustment (per-key and global)
- Implement Rapid Trigger functionality
- Extend RGB lighting support to full feature parity with official software
- Create comprehensive protocol documentation for community use
- Ensure all features integrate cleanly with existing steelseriesgg-rs architecture
- Maintain existing functionality without regressions

## User Stories

### US-001: Set up USB HID capture infrastructure (WINE + usbmon)
**Description:** As a developer, I need to capture raw HID reports from the Apex Pro TKL 2023 so I can analyze the protocol.

**Acceptance Criteria:**
- [ ] Install SteelSeries GG via Lutris (WINE) following community guides
- [ ] Verify device detection in SteelSeries GG running under WINE
- [ ] Configure usbmon/Wireshark for USB packet capture on Linux
- [ ] Identify correct USB device path and interface numbers for Apex Pro TKL 2023
- [ ] Successfully capture HID traffic during RGB changes in SteelSeries GG
- [ ] Create scripts/tools for filtering and parsing captured HID data
- [ ] Typecheck passes

### US-002: Analyze existing RGB command structure
**Description:** As a developer, I need to understand the current working RGB implementation so I can identify patterns for discovering new commands.

**Acceptance Criteria:**
- [ ] Document existing RGB HID report format from `generic.rs` and `keyboards/mod.rs`
- [ ] Identify report ID, command byte, and data payload structure
- [ ] Test and document which RGB commands already work (static, effects)
- [ ] Identify gaps in RGB functionality compared to official software
- [ ] Create baseline command reference document
- [ ] Typecheck passes

### US-003: Discover actuation point command structure
**Description:** As a developer, I need to identify the HID commands for adjusting actuation points so users can customize key sensitivity.

**Acceptance Criteria:**
- [ ] Capture USB traffic while changing actuation settings in SteelSeries GG (WINE)
- [ ] Identify command byte(s) for global actuation setting from captures
- [ ] Identify command byte(s) for per-key actuation setting from captures
- [ ] Document byte positions for actuation value (0.4-3.6mm range)
- [ ] Test discovered commands via steelseriesgg-rs on actual hardware
- [ ] Verify commands work by testing key press behavior at different actuation points
- [ ] Add findings to PROTOCOL_RESEARCH.md
- [ ] Typecheck passes

### US-004: Implement global actuation point adjustment API
**Description:** As a user, I want to set a global actuation point for all keys so I can optimize keyboard response for my typing/gaming style.

**Acceptance Criteria:**
- [ ] Add `set_actuation_point_global(distance_mm: f32)` method to `Keyboard` trait
- [ ] Validate actuation distance in range 0.4mm to 3.6mm (hardware limits)
- [ ] Implement HID report construction using discovered protocol
- [ ] Add CLI command `ssgg actuation --global <distance>`
- [ ] Test on actual hardware at multiple actuation distances
- [ ] Typecheck passes
- [ ] Verify changes work on device

### US-005: Implement per-key actuation point adjustment API
**Description:** As a user, I want to set custom actuation points for individual keys so I can optimize WASD for gaming while keeping typing keys at normal sensitivity.

**Acceptance Criteria:**
- [ ] Add `set_actuation_point_key(key: KeyCode, distance_mm: f32)` method to `Keyboard` trait
- [ ] Define KeyCode enum or mapping for all keys on Apex Pro TKL
- [ ] Implement HID report construction for per-key settings
- [ ] Add CLI command `ssgg actuation --key <keycode> <distance>`
- [ ] Support batch key updates for common groups (e.g., WASD)
- [ ] Test on actual hardware with mixed actuation settings
- [ ] Typecheck passes
- [ ] Verify changes work on device

### US-006: Discover Rapid Trigger command structure
**Description:** As a developer, I need to identify the HID commands for enabling/disabling Rapid Trigger so users can access this competitive gaming feature.

**Acceptance Criteria:**
- [ ] Capture USB traffic while toggling Rapid Trigger in SteelSeries GG (WINE)
- [ ] Identify command byte(s) for global Rapid Trigger enable/disable from captures
- [ ] Identify command byte(s) for per-key Rapid Trigger enable/disable (if supported)
- [ ] Document any sensitivity or threshold parameters from captured packets
- [ ] Test discovered commands via steelseriesgg-rs on actual hardware
- [ ] Verify Rapid Trigger behavior by testing rapid key presses
- [ ] Add findings to PROTOCOL_RESEARCH.md
- [ ] Typecheck passes

### US-007: Implement Rapid Trigger toggle API
**Description:** As a user, I want to enable Rapid Trigger mode so I can eliminate key debounce delay for faster inputs in competitive games.

**Acceptance Criteria:**
- [ ] Add `set_rapid_trigger(enabled: bool)` method to `Keyboard` trait
- [ ] Add optional per-key Rapid Trigger control if protocol supports it
- [ ] Implement HID report construction using discovered protocol
- [ ] Add CLI command `ssgg rapid-trigger --enable` / `--disable`
- [ ] Test on actual hardware by measuring key repeat speed
- [ ] Typecheck passes
- [ ] Verify changes work on device

### US-008: Extend RGB command support
**Description:** As a user, I want access to all RGB effects and customization options available in the official software so I have feature parity on Linux.

**Acceptance Criteria:**
- [ ] Document any missing RGB effects from official software
- [ ] Discover and implement commands for missing effects
- [ ] Add support for per-key RGB customization if available
- [ ] Test all RGB effects on actual hardware
- [ ] Update CLI to expose new RGB capabilities
- [ ] Typecheck passes
- [ ] Verify changes work in browser/device

### US-009: Add actuation profile management
**Description:** As a user, I want to save and load actuation profiles so I can quickly switch between typing and gaming configurations.

**Acceptance Criteria:**
- [ ] Extend `Profile` struct to include actuation settings (global and per-key)
- [ ] Extend `Profile` struct to include Rapid Trigger state
- [ ] Update profile serialization/deserialization for new fields
- [ ] Add CLI commands for saving/loading actuation profiles
- [ ] Test profile persistence across application restarts
- [ ] Typecheck passes

### US-010: Create comprehensive protocol documentation
**Description:** As a community member, I want complete protocol documentation so I can build integrations or port features to other platforms.

**Acceptance Criteria:**
- [ ] Create `docs/PROTOCOL.md` with full HID command reference
- [ ] Document all discovered command bytes and data formats
- [ ] Include example HID reports for each command
- [ ] Document initialization sequence and device state management
- [ ] Add protocol version notes and hardware-specific variations
- [ ] Include troubleshooting section for common issues
- [ ] Typecheck passes (if any code examples)

### US-011: Add integration tests for new features
**Description:** As a developer, I need automated tests to prevent regressions when adding new protocol features.

**Acceptance Criteria:**
- [ ] Add unit tests for actuation distance validation and conversion
- [ ] Add unit tests for HID report construction (actuation, Rapid Trigger)
- [ ] Add mock device tests for command sequencing
- [ ] Verify profile serialization includes new fields
- [ ] All tests pass with `cargo test`
- [ ] Typecheck passes

## Non-Goals

- **Hardware emulation or virtual device support** - Focus is on physical Apex Pro TKL 2023 hardware only
- **Native Windows development** - Will use WINE/Lutris for running official software, but all steelseriesgg-rs development remains Linux-focused
- **Macro programming** - Will not implement macro recording/playback (out of scope)
- **Display/OLED control** - Will not implement OLED screen features (Apex Pro TKL 2023 has no display)
- **Other keyboard models** - Protocol discoveries are specific to Apex Pro TKL 2023; porting to other models is future work
- **Firmware updates** - Will not implement firmware flashing or update capabilities
- **Polling rate adjustment** - Already implemented in `pollrate.rs`, no additional work needed
- **Audio/Sonar integration** - Keyboard-specific features only, no audio mixer work

## Technical Considerations

### Existing Codebase Integration
- Extend `Keyboard` trait in `src/devices/keyboards/mod.rs` with new methods
- Reuse existing HID communication infrastructure (`write_padded_report`, device manager)
- Follow existing error handling patterns using `Result<()>` and `SteelSeriesError`
- Maintain consistency with existing CLI structure in `src/main.rs`

### Protocol Discovery Methodology
1. **WINE/Lutris setup**: Install SteelSeries GG via Lutris for controlled USB traffic generation
2. **Baseline capture**: Use `usbmon`/Wireshark to capture HID traffic while using official software
3. **Pattern analysis**: Compare captured commands with existing open-source implementations (apex-tux, msi-perkeyrgb)
4. **Command testing**: Validate discovered commands via steelseriesgg-rs on actual hardware
5. **Systematic exploration**: Fill gaps with targeted command byte testing where captures are insufficient
6. **Documentation**: Record all findings incrementally in PROTOCOL_RESEARCH.md

### Hardware Constraints
- Actuation range: 0.4mm (minimum) to 3.6mm (maximum) per SteelSeries specs
- Poll rate: Already handled by existing `pollrate.rs` module
- Interface number: Use interface 1 for control commands (per existing code)
- HID report size: 65 bytes (report ID + 64 data bytes) per existing implementation

### Safety Considerations
- **Avoid bricking**: Never test firmware-related commands without confirmation
- **Rate limiting**: Space out command testing to avoid overwhelming device
- **Backup profiles**: Document default settings before testing
- **Reversible changes**: Ensure all tested commands can be reverted

### Tools and Resources
- **WINE/Lutris**: Run SteelSeries GG on Linux for controlled protocol capture
- **usbmon**: Kernel USB monitoring for HID capture
- **Wireshark**: For parsing usbmon captures (with USB dissector)
- **hidapi**: Already used in codebase for HID communication
- **Community projects**: apex-tux (Rust), apex7tkl_linux (Python), msi-perkeyrgb (detailed protocol docs)
- **Protocol research**: See PROTOCOL_RESEARCH.md for comprehensive findings and references

## Success Metrics

- [ ] All core features implemented and tested on hardware (actuation, Rapid Trigger, RGB)
- [ ] Comprehensive protocol documentation published
- [ ] Zero regressions in existing RGB/device detection functionality
- [ ] CLI commands provide intuitive access to new features
- [ ] Profile system successfully persists new settings
- [ ] Protocol documentation includes enough detail for community reimplementation

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Protocol is encrypted or obfuscated | Unlikely given existing open-source implementations; if present, focus on WINE capture of actual traffic |
| WINE/Lutris device detection issues | Follow community guides (steelseries-linux-lutris), use USB passthrough if needed, fall back to systematic testing |
| Limited protocol documentation | Leverage existing projects (apex-tux, msi-perkeyrgb) for patterns; WINE capture provides ground truth |
| Device damage from incorrect commands | Test conservatively, avoid firmware-related commands (0xF0+ range), document all safe command ranges |
| Incomplete protocol discovery | Prioritize RGB → actuation → Rapid Trigger; partial success is acceptable |
| Time-consuming trial and error | WINE capture significantly reduces blind testing; start with most impactful features |

---

**Last Updated**: 2026-01-13
**Status**: Planning Phase
**Target Completion**: Incremental (protocol discovery is ongoing)
