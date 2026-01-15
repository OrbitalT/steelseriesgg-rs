# PRD: Automated Bulk HID Command Testing System

## Introduction

Create an automated testing system to systematically discover actuation point commands for the Apex Pro TKL 2023 by testing all command candidates (0x20-0x2F) with multiple value patterns, detecting keyboard state changes automatically, and generating comprehensive reports. This eliminates the slow manual testing loop and accelerates protocol reverse engineering.

## Goals

- Test all commands in 0x20-0x2F range with minimal manual intervention
- Automatically detect keyboard state changes (actuation point, menu state, RGB changes)
- Test commands with and without save/apply (0x09) to understand persistence behavior
- Generate structured test reports mapping commands to observed effects
- Test multiple value encodings per command (0x04, 0x08, 0x14, 0x24, 0x00, etc.)
- Complete all testing in one automated run

## User Stories

### US-001: Create automated test harness binary
**Description:** As a developer, I need a test harness that can send multiple commands in sequence and collect results so I can test efficiently without manual intervention.

**Acceptance Criteria:**
- [ ] Create `src/bin/bulk_test.rs` binary
- [ ] Accepts command range (e.g., 0x20-0x2F) as CLI argument
- [ ] Accepts value patterns to test (e.g., 0x00, 0x04, 0x08, 0x14, 0x24)
- [ ] Sends each command with each value pattern systematically
- [ ] Logs all sent commands with timestamps to stdout
- [ ] Includes delay between tests (configurable, default 2 seconds)
- [ ] Cargo build succeeds

### US-002: Add keyboard state polling capability
**Description:** As a developer, I need to read the current actuation setting from the keyboard so I can automatically detect when a command changes it.

**Acceptance Criteria:**
- [ ] Add function to read current actuation point from keyboard (if possible via HID read)
- [ ] If direct read not available, document limitation and add placeholder
- [ ] Test function can detect difference between 0.4mm, 0.8mm, 1.2mm, etc.
- [ ] Add to `src/devices/keyboards/generic.rs` or appropriate module
- [ ] Cargo build succeeds

### US-003: Implement test result detection and logging
**Description:** As a developer, I need to capture observable effects after each command so the test report shows what each command does.

**Acceptance Criteria:**
- [ ] After each command, poll keyboard state (actuation setting)
- [ ] Compare state before/after command to detect changes
- [ ] Log result: "no_change", "actuation_changed", "read_failed", "error"
- [ ] Include previous and new actuation values if changed
- [ ] Log to structured format (JSON or CSV)
- [ ] Cargo build succeeds

### US-004: Test commands with and without save/apply
**Description:** As a developer, I need to test each command both with and without the 0x09 save command so I understand which commands require persistence.

**Acceptance Criteria:**
- [ ] For each command+value combination, run two tests: with 0x09 and without 0x09
- [ ] Log whether 0x09 was sent as part of test metadata
- [ ] Compare results: does 0x09 make a difference?
- [ ] Record timing: state change immediate or only after 0x09?
- [ ] Cargo build succeeds

### US-005: Generate comprehensive test report
**Description:** As a developer, I need a detailed report of all test results so I can identify which commands control actuation.

**Acceptance Criteria:**
- [ ] Generate report file: `test-results-{timestamp}.json`
- [ ] Include: command, value, with_save, state_before, state_after, effect, timestamp
- [ ] Generate summary: commands that changed state, commands with no effect, errors
- [ ] Include test configuration: command range, values tested, delays used
- [ ] Save report to `docs/sessions/` directory
- [ ] Cargo build succeeds

### US-006: Add test for parameterless commands
**Description:** As a developer, I need to test commands without value parameters so I can identify navigation commands like 0x24.

**Acceptance Criteria:**
- [ ] Add flag `--test-parameterless` to test harness
- [ ] Send commands without value byte (just command byte + padding)
- [ ] Log parameterless tests separately in report
- [ ] Test all commands in range as parameterless
- [ ] Cargo build succeeds

### US-007: Execute full test suite and analyze results
**Description:** As a developer, I need to run the complete test suite and analyze results so I can identify the actuation command.

**Acceptance Criteria:**
- [ ] Run test harness: commands 0x20-0x2F, values [0x00, 0x04, 0x08, 0x14, 0x24, 0x30]
- [ ] Test both with and without 0x09
- [ ] Test parameterless versions
- [ ] Total: ~16 commands × 6 values × 2 save modes × 2 param modes = ~384 tests
- [ ] Complete test run generates valid report file
- [ ] Review report and identify commands that changed actuation
- [ ] Update PROTOCOL_RESEARCH.md with findings
- [ ] Cargo build succeeds

## Non-Goals

- No GUI or interactive progress display (terminal output only)
- No retry logic if tests fail (fail fast and log errors)
- No testing of commands outside 0x20-0x2F range (focus on actuation only)
- No RGB effect detection or logging (focus on actuation only)
- No per-key actuation testing (global actuation only for now)
- No Rapid Trigger testing (0x30-0x3F range deferred to future work)

## Technical Considerations

### Keyboard State Polling
- **Challenge:** May not be possible to read actuation setting via HID
- **Fallback:** If reading not supported, use OSD observation + manual verification
- **Alternative:** Use manual checkpoint system where user verifies at intervals

### Test Duration
- 384 tests × 2 second delay = ~13 minutes total runtime
- Can reduce delay if no state changes observed
- May need longer delays for OSD menu commands

### Safety
- All commands in 0x20-0x2F tested previously without adverse effects
- Include emergency stop mechanism (Ctrl+C handling)
- Backup current keyboard settings before starting (if possible)

### Existing Code to Reuse
- `src/bin/test_commands.rs` - Basic command sending infrastructure
- `src/devices/discovery.rs` - Device enumeration and opening
- `DeviceManager` and `Device` trait - HID communication layer
- Existing debug logging via `tracing`

## Success Metrics

- Test suite runs to completion without manual intervention
- At least one command identified that changes actuation setting
- All test results logged with sufficient detail to analyze
- Report clearly identifies actuation command(s)
- Total testing time < 20 minutes
