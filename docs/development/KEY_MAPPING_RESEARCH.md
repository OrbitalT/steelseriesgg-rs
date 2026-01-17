# SteelSeries Apex Pro Key Mapping Research

**Date**: 2026-01-16
**Status**: PLACEHOLDER MAPPINGS - Requires Hardware Research
**Priority**: Critical for per-key RGB implementation

## Overview

This document outlines the current state of key-to-address mapping research for SteelSeries Apex Pro keyboards. The goal is to enable precise per-key RGB control by understanding the HID key matrix addressing scheme.

## ⚠️ CRITICAL DISCLAIMER

**The key mappings currently implemented are PLACEHOLDER data based on standard keyboard layouts and educated guesses. They are NOT accurate for real hardware and should NOT be used in production.**

To use per-key RGB control, accurate HID key addresses must be discovered through hardware research and reverse engineering.

## Current Implementation

### Architecture

The key mapping system is implemented in `src/devices/key_mapping.rs` with:

- **KeyId enum**: Physical key identifiers (e.g., `KeyId::A`, `KeyId::F1`)
- **KeyAddress struct**: Matrix coordinates `(row, col)` for HID addressing
- **KeyMapping struct**: Complete mapping for a specific keyboard model
- **KeyMappingDatabase**: Database of all supported keyboard mappings

### Supported Models (PLACEHOLDER)

| Model | Product ID | Layout | Matrix Size | Status |
|-------|------------|--------|-------------|---------|
| Apex Pro TKL (2023) | 0x1628 | TenKeyLess | 6×17 | PLACEHOLDER |
| Apex Pro | 0x1610 | Full-size | 6×21 | PLACEHOLDER |
| Apex Pro TKL | 0x1614 | TenKeyLess | 6×17 | PLACEHOLDER |

### Current Structure

```rust
// Example placeholder mapping
mapping.add_key(KeyId::A, KeyAddress::new(3, 1));      // Row 3, Col 1
mapping.add_key(KeyId::Enter, KeyAddress::new(3, 13)); // Row 3, Col 13
mapping.add_key(KeyId::Space, KeyAddress::new(5, 6));  // Row 5, Col 6
```

## Research Requirements

### Phase 1: HID Descriptor Analysis

**Objective**: Understand the device's HID report structure

**Methods**:
1. **Linux**: `lsusb -v` to dump HID descriptors
2. **Analysis**: Parse HID report descriptor to find input/output report structure
3. **Tools**: `hidapi` test programs, `usbhid-dump`

**Expected Findings**:
- Report size (already known: 65 bytes for keyboards)
- Input/output report types
- Usage pages and usage IDs
- Possible key addressing hints

### Phase 2: Protocol Analysis

**Objective**: Discover per-key RGB commands by analyzing existing software

**Methods**:
1. **USB Traffic Capture**: Use Wireshark/USBPcap to monitor SteelSeries Engine
2. **HID Report Analysis**: Decode captured HID reports
3. **Pattern Recognition**: Identify per-key command patterns

**Expected Findings**:
- Per-key RGB command format (likely different from current 0x21 zone command)
- Key addressing scheme
- Command validation and acknowledgment

### Phase 3: Hardware Testing

**Objective**: Empirically discover key addresses through controlled testing

**Methods**:
1. **Systematic Testing**: Send test commands to individual matrix positions
2. **Visual Verification**: Observe which physical keys light up
3. **Matrix Mapping**: Build accurate row/column to physical key mapping

**Tools Needed**:
- Apex Pro hardware (preferably multiple variants)
- Test RGB commands with single-key targeting
- Physical observation and documentation

### Phase 4: Reverse Engineering

**Objective**: Understand SteelSeries proprietary protocol extensions

**Methods**:
1. **Firmware Analysis**: If possible, analyze keyboard firmware
2. **Driver Analysis**: Reverse engineer Windows/macOS drivers
3. **Documentation**: Search for leaked protocol specifications

**Risk**: May violate terms of service or IP rights

## Known Protocol Information

### Current Zone-Based RGB (Working)

```
Command: 0x21 0xFF [R G B] [R G B] ... (9 zones)
- Works for zone-based lighting
- Controls groups of keys, not individual keys
- Limited precision for effects
```

### Hypothetical Per-Key RGB (Unknown)

```
Possible formats:
1. 0x21 [key_id] [R G B]           # Single key
2. 0x21 [row] [col] [R G B]        # Matrix addressing
3. 0x2X [key_matrix_data...]       # Bulk key update
4. Different command code entirely  # Not 0x21
```

### Missing Commands

- **Key Query**: Read current key state/color
- **Matrix Info**: Get matrix dimensions
- **Key Capability**: Which keys support RGB
- **Firmware Info**: Protocol version/capabilities

## Implementation Strategy

### Immediate (Development Structure)

✅ **Completed**:
- Key mapping data structure (`KeyId`, `KeyAddress`, `KeyMapping`)
- Database system for multiple keyboard variants
- Placeholder mappings for development
- Comprehensive test coverage

### Short-term (Research Phase)

🔄 **Next Steps**:
1. Create HID research tools in `src/devices/research/`
2. Implement USB traffic capture analysis
3. Build systematic key testing utilities
4. Document findings in this research log

### Long-term (Production Ready)

⏸️ **Future Work**:
1. Replace placeholder mappings with accurate data
2. Implement per-key RGB commands in HID reports system
3. Add runtime key mapping discovery
4. Create validation tools for mapping accuracy

## Research Tools Needed

### HID Analysis Tools

```rust
// Proposed: src/devices/research/hid_analyzer.rs
pub struct HidAnalyzer {
    device: HidDevice,
    capture_log: Vec<HidReport>,
}

impl HidAnalyzer {
    pub fn capture_traffic(&mut self, duration: Duration) -> Result<()>;
    pub fn analyze_patterns(&self) -> Vec<ProtocolPattern>;
    pub fn test_key_matrix(&mut self, start_row: u8, end_row: u8) -> Result<()>;
}
```

### Traffic Capture Integration

```rust
// Proposed: Integration with existing diagnostic system
use crate::devices::diagnostics::HidDiagnostics;

// Enhance diagnostics to capture per-key test patterns
diagnostics.start_key_mapping_research()?;
```

### Systematic Testing Framework

```rust
// Proposed: src/devices/research/key_tester.rs
pub struct KeyMatrixTester {
    keyboard: Box<dyn Keyboard>,
    results: HashMap<KeyAddress, TestResult>,
}

impl KeyMatrixTester {
    pub fn test_matrix_position(&mut self, addr: KeyAddress) -> Result<TestResult>;
    pub fn discover_all_keys(&mut self) -> Result<Vec<(KeyAddress, KeyId)>>;
    pub fn validate_mapping(&self, mapping: &KeyMapping) -> ValidationReport;
}
```

## Current Limitations

### Functional Limitations

- ❌ **No per-key control**: Only zone-based RGB
- ❌ **Inaccurate mappings**: Placeholder data only
- ❌ **No key discovery**: Cannot detect available keys
- ❌ **No validation**: Cannot verify mapping accuracy

### Research Limitations

- ❌ **No hardware access**: Development without physical devices
- ❌ **No protocol docs**: Proprietary SteelSeries protocol
- ❌ **No reference impl**: Cannot analyze official software behavior
- ❌ **Legal constraints**: Reverse engineering restrictions

## Expected Challenges

### Technical Challenges

1. **Matrix Complexity**: Real key matrices may be non-linear or have gaps
2. **Model Variations**: Different Apex Pro models may use different addressing
3. **Protocol Evolution**: Newer firmware may change command formats
4. **Error Handling**: Invalid addresses may cause device lockup/reset

### Legal/Ethical Challenges

1. **IP Rights**: Key mapping data may be considered IP
2. **DMCA**: Reverse engineering may violate software licenses
3. **ToS Violation**: Analyzing official software may breach terms
4. **Hardware Warranty**: Testing may void device warranty

## Research Log

### 2026-01-16: Initial Implementation

**Accomplished**:
- ✅ Complete key mapping data structure
- ✅ Database for multiple keyboard models
- ✅ Placeholder mappings (development only)
- ✅ Comprehensive test coverage (6/6 tests passing)
- ✅ Integration with existing HID report system

**Findings**:
- Standard matrix addressing is typically 6-8 rows × 15-21 columns
- TKL keyboards usually use fewer columns than full-size
- Matrix utilization is typically 40-60% (many positions unused)
- SteelSeries likely uses sparse matrix addressing

**Next Priority**: Create HID analysis tools for traffic capture

### Future Research Entries

```
### YYYY-MM-DD: [Research Phase]
**Objective**: [What was attempted]
**Methods**: [How research was conducted]
**Findings**: [What was discovered]
**Challenges**: [What problems were encountered]
**Next Steps**: [What should be tried next]
```

## Testing Matrix

### Development Testing (Current)

| Test | Status | Notes |
|------|--------|-------|
| Data structures | ✅ PASS | All 6 tests passing |
| Database loading | ✅ PASS | Known products detected |
| Mapping stats | ✅ PASS | Statistics calculation works |
| Key identification | ✅ PASS | KeyId display formats correct |

### Hardware Testing (Future)

| Test | Status | Hardware Required |
|------|--------|-------------------|
| Matrix addressing | ❌ TODO | Apex Pro TKL 2023 |
| Per-key RGB commands | ❌ TODO | Any Apex Pro model |
| Command validation | ❌ TODO | Multiple Apex variants |
| Error handling | ❌ TODO | Test invalid addresses |

## Integration Points

### Current Integration

The key mapping system integrates with:

- ✅ **HID Reports**: `HidReportBuilder` ready for per-key commands
- ✅ **Diagnostics**: Can log key mapping test results
- ✅ **Device Discovery**: Product ID based mapping selection
- ✅ **Keyboard Traits**: Ready for per-key methods

### Future Integration

When accurate mappings are available:

- 🔄 **RGB Controller**: Per-key effect rendering
- 🔄 **GameSense**: Individual key reactive lighting
- 🔄 **Profiles**: Save/load per-key configurations
- 🔄 **CLI Tools**: Per-key testing and debugging

## Success Criteria

### Phase 1 Success (Research Structure)

✅ **COMPLETED**: All requirements met

- [x] Complete key mapping data structure
- [x] Database system for multiple keyboards
- [x] Placeholder mappings for development
- [x] Comprehensive test coverage
- [x] Integration with HID system

### Phase 2 Success (Research Tools)

⏸️ **PENDING**: Hardware access required

- [ ] HID traffic analysis tools
- [ ] Systematic matrix testing
- [ ] Protocol pattern recognition
- [ ] Research documentation

### Phase 3 Success (Accurate Mappings)

⏸️ **PENDING**: Requires Phase 2 completion

- [ ] Accurate key addresses for at least one Apex Pro model
- [ ] Validated per-key RGB command format
- [ ] Error handling for invalid addresses
- [ ] Production-ready key mapping database

### Phase 4 Success (Full Implementation)

⏸️ **PENDING**: Long-term goal

- [ ] Per-key RGB control in user interface
- [ ] Advanced lighting effects (ripple, reactive, etc.)
- [ ] GameSense per-key integration
- [ ] Support for multiple Apex Pro variants

## Conclusion

**Current Status**: US-004 architecture and structure are complete. The key mapping system provides a solid foundation for per-key RGB control once accurate HID addresses are discovered.

**Critical Path**: Hardware research is the blocking factor. All software infrastructure is ready to support per-key control once the protocol is reverse-engineered.

**Risk Assessment**: High dependency on hardware access and reverse engineering success. Alternative: Focus on improved zone-based effects while researching per-key capabilities.

**Next Actions**:
1. Complete US-004 documentation and testing
2. Move to US-005 (per-key RGB command builder) with placeholder data
3. Prioritize hardware acquisition for research
4. Build research tools for future protocol analysis

---

**Document Status**: Complete for current development phase
**Last Updated**: 2026-01-16
**Next Review**: When hardware access is available