# PRD: Precise SteelSeries Apex Pro RGB Control

## Introduction

Improve RGB lighting control for SteelSeries Apex Pro keyboards to achieve reliable, precise per-key control. The current implementation suffers from random key targeting, inconsistent zone responses, and unreliable command execution. This effort will focus on protocol accuracy, proper HID communication, and zone mapping precision.

## Goals

- Achieve reliable per-key RGB control for Apex Pro keyboards
- Eliminate random key color assignments
- Ensure consistent zone response across the entire keyboard
- Implement robust HID protocol handling with proper error recovery
- Create a foundation expandable to other SteelSeries models
- Provide diagnostic tools for protocol analysis and debugging

## User Stories

### US-001: Research and document Apex Pro HID protocol
**Description:** As a developer, I need to understand the exact HID protocol for Apex Pro keyboards so I can implement precise control.

**Acceptance Criteria:**
- [ ] Document current protocol implementation in `docs/development/APEX_PRO_PROTOCOL.md`
- [ ] Analyze existing product ID constants and interface numbers for Apex Pro variants
- [ ] Research official SteelSeries documentation or reverse engineer from working implementations
- [ ] Document zone mapping and addressing scheme
- [ ] Identify differences between per-key vs zone-based commands
- [ ] Typecheck passes

### US-002: Create HID communication diagnostic tool
**Description:** As a developer, I need diagnostic tools to analyze HID communication so I can debug protocol issues.

**Acceptance Criteria:**
- [ ] Add `--debug-hid` flag to CLI that logs all raw HID reports
- [ ] Create function to validate HID report structure and checksums
- [ ] Add timing analysis for command execution (detect timeouts/delays)
- [ ] Log device responses and identify error patterns
- [ ] Save diagnostic output to timestamped files
- [ ] Typecheck passes

### US-003: Implement reliable HID report structure for Apex Pro
**Description:** As a developer, I need proper HID report formatting so commands reach the intended keys consistently.

**Acceptance Criteria:**
- [ ] Define Apex Pro specific report structure (extends generic 65-byte format)
- [ ] Implement proper header/command bytes for per-key addressing
- [ ] Add checksum/validation bytes if required by protocol
- [ ] Create helper functions for building per-key vs zone commands
- [ ] Add retry logic for failed command execution
- [ ] Typecheck passes

### US-004: Create accurate key-to-address mapping for Apex Pro
**Description:** As a developer, I need precise key addressing so RGB commands target the correct physical keys.

**Acceptance Criteria:**
- [ ] Create comprehensive key mapping table for Apex Pro layout
- [ ] Map each physical key to its HID address/index
- [ ] Account for different keyboard layouts (ANSI, ISO if applicable)
- [ ] Validate mapping against known working implementations
- [ ] Add validation function to verify key addresses are within valid range
- [ ] Typecheck passes

### US-005: Implement per-key RGB command builder
**Description:** As a developer, I need reliable command generation so each key receives the intended color.

**Acceptance Criteria:**
- [ ] Create `build_per_key_rgb_command(key_address, color)` function
- [ ] Implement `build_multi_key_rgb_command(key_color_map)` for efficient batch updates
- [ ] Add command validation before sending to device
- [ ] Implement proper timing/delays between commands if needed
- [ ] Add error handling for invalid key addresses or colors
- [ ] Typecheck passes

### US-006: Add per-key RGB control to keyboard trait
**Description:** As a user, I want to set individual key colors so I can create precise lighting effects.

**Acceptance Criteria:**
- [ ] Add `set_key_color(key_id, color)` method to Keyboard trait
- [ ] Add `set_multiple_keys(key_color_map)` for batch operations
- [ ] Implement for GenericKeyboard with Apex Pro specific logic
- [ ] Add key ID validation and error handling
- [ ] Method returns Result with descriptive errors for failures
- [ ] Typecheck passes

### US-007: Create CLI commands for per-key testing
**Description:** As a user, I want CLI commands to test individual keys so I can verify the implementation works correctly.

**Acceptance Criteria:**
- [ ] Add `ssgg rgb set-key --key <key_id> --color <color>` command
- [ ] Add `ssgg rgb test-keys` command that cycles through all keys with different colors
- [ ] Add `ssgg rgb map-keys` command that shows physical layout with addresses
- [ ] Commands provide clear feedback on success/failure
- [ ] Include help text with valid key IDs and color formats
- [ ] Typecheck passes
- [ ] Verify changes work with actual Apex Pro keyboard

### US-008: Implement reliable zone-based RGB fallback
**Description:** As a user, I want zone-based RGB control to work reliably when per-key control isn't available.

**Acceptance Criteria:**
- [ ] Improve existing zone RGB implementation for better reliability
- [ ] Add proper timing and retry logic for zone commands
- [ ] Validate zone addresses match Apex Pro hardware layout
- [ ] Ensure zone commands don't interfere with per-key commands
- [ ] Add automatic fallback from per-key to zone mode if needed
- [ ] Typecheck passes
- [ ] Verify changes work with actual Apex Pro keyboard

### US-009: Add RGB effect engine per-key support
**Description:** As a user, I want effects like breathing and wave to work on individual keys so I can create detailed lighting patterns.

**Acceptance Criteria:**
- [ ] Extend EffectEngine to support per-key color computation
- [ ] Add per-key variants of existing effects (per-key breathing, wave, etc.)
- [ ] Implement efficient per-key color caching to avoid excessive HID commands
- [ ] Add smooth transitions between per-key effect states
- [ ] Optimize for minimal USB communication overhead
- [ ] Typecheck passes
- [ ] Verify effects work smoothly on actual keyboard

### US-010: Create validation and testing framework
**Description:** As a developer, I need automated testing to ensure RGB control remains reliable across changes.

**Acceptance Criteria:**
- [ ] Add unit tests for key mapping and address validation
- [ ] Create integration tests for HID command building
- [ ] Add regression tests for common failure scenarios
- [ ] Create mock device for testing without physical hardware
- [ ] Add performance benchmarks for per-key vs zone operations
- [ ] All tests pass
- [ ] Typecheck passes

## Non-Goals

- Support for other SteelSeries keyboard models in this iteration (focus on Apex Pro first)
- Advanced macro programming or key remapping functionality
- Audio-reactive RGB features (focus on basic reliable control first)
- Cross-platform compatibility beyond Linux (current target platform)
- GUI configuration tools (CLI-first approach)

## Technical Considerations

- Reuse existing HID communication infrastructure in `devices/` module
- Leverage current RGB effect engine architecture for consistency
- Build on existing error handling patterns using `thiserror`
- Consider USB communication timing to avoid overwhelming the device
- Plan for future expansion to other keyboard models
- Document all protocol discoveries for future reference

The implementation will start with your specific Apex Pro model to ensure reliable per-key control, with architecture that allows expansion to other models later.