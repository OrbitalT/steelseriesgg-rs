# Rough Idea

Ensure the steelseries protocol implementation is fully working and complete.

## Context

This project aims to complete the SteelSeries protocol implementation for the steelseriesgg-rs project, focusing on the Apex Pro TKL 2023 keyboard.

### Current State
- Basic RGB zone control implemented (9 zones)
- Commands confirmed: 0x21 (RGB), 0x22 (brightness), 0x09 (save), 0x23 (effects), 0x24 (OSD)
- Bulk testing system implemented for command discovery
- Test results available but not fully analyzed

### Major Gaps
- No per-key RGB control
- No actuation point control (core Apex Pro feature)
- No Rapid Trigger protocol
- Missing read/query commands
- Bulk test results (0x20, 0x25-0x2F) not analyzed
- Commands beyond 0x2F not tested

### Available Documentation
- `docs/development/PROTOCOL_RESEARCH.md` - Comprehensive research findings
- `docs/development/APEX_PRO_PROTOCOL.md` - Current protocol documentation
- Existing implementations: apex-tux, msi-perkeyrgb
- Official GameSense SDK (high-level API only)
