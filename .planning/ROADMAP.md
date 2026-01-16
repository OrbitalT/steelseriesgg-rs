# Roadmap: steelseriesgg-rs

## Overview

This roadmap transforms steelseriesgg-rs from a functional SteelSeries device controller into a comprehensive, user-friendly ecosystem. Starting with performance optimizations and diagnostics, we'll add advanced device features, a web interface, and an extensible plugin system to rival proprietary solutions.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Performance Foundation** - Optimize existing system for reliability and speed
- [ ] **Phase 2: Enhanced Diagnostics** - Build comprehensive troubleshooting capabilities
- [ ] **Phase 3: Advanced Device Support** - Expand device functionality and compatibility
- [ ] **Phase 4: Web Interface Core** - Create web-based configuration interface
- [ ] **Phase 5: Web Interface Polish** - Complete web interface with advanced features
- [ ] **Phase 6: Plugin System Foundation** - Build core plugin architecture
- [ ] **Phase 7: Plugin Ecosystem** - Enable community extensions and integrations

## Phase Details

### Phase 1: Performance Foundation
**Goal**: System operates with professional-grade performance and reliability
**Depends on**: Nothing (first phase)
**Requirements**: PERF-01, PERF-02, PERF-03, PERF-04, PERF-05
**Success Criteria** (what must be TRUE):
  1. RGB effects maintain consistent frame timing even under system load
  2. CPU usage decreases measurably during typical operation
  3. Daemon runs for days without memory leaks or performance degradation
  4. Multiple simultaneous effects render smoothly without conflicts
  5. Device hotplug events are handled gracefully without crashes
**Research**: Unlikely (optimization of existing patterns)
**Plans**: TBD

### Phase 2: Enhanced Diagnostics
**Goal**: Users can easily troubleshoot and debug device issues
**Depends on**: Phase 1
**Requirements**: DIAG-01, DIAG-02, DIAG-03, DIAG-04, DIAG-05
**Success Criteria** (what must be TRUE):
  1. User can instantly see if devices are connected and functioning
  2. Technical users can inspect raw HID communication for debugging
  3. User can run automated tests to verify device responsiveness
  4. Bug reports include complete diagnostic information automatically
  5. Performance metrics show RGB timing accuracy and bottlenecks
**Research**: Unlikely (extending existing diagnostic patterns)
**Plans**: TBD

### Phase 3: Advanced Device Support
**Goal**: Users have full control over all device capabilities
**Depends on**: Phase 2
**Requirements**: DEVICE-01, DEVICE-02, DEVICE-03, DEVICE-04, DEVICE-05, DEVICE-06
**Success Criteria** (what must be TRUE):
  1. User can assign any key to custom functions beyond just RGB
  2. Different applications can have unique key mapping profiles
  3. Individual RGB zones respond to precise user control
  4. New keyboard models work automatically without code changes
  5. New headset models work automatically without code changes
  6. User can record and play macro sequences on supported hardware
**Research**: Likely (newer device protocols, key mapping standards)
**Research topics**: SteelSeries HID protocol updates, key mapping APIs, macro storage formats
**Plans**: TBD

### Phase 4: Web Interface Core
**Goal**: Users can configure devices through a modern web interface
**Depends on**: Phase 3
**Requirements**: WEB-01, WEB-02, WEB-03
**Success Criteria** (what must be TRUE):
  1. User can access full device configuration via web browser
  2. RGB settings work identically to CLI interface through web UI
  3. Profile management (save/load/edit) works through web interface
**Research**: Likely (web framework choice, real-time updates)
**Research topics**: Rust web frameworks (Axum vs alternatives), WebSocket for real-time updates, web UI libraries
**Plans**: TBD

### Phase 5: Web Interface Polish
**Goal**: Web interface provides superior user experience on all devices
**Depends on**: Phase 4
**Requirements**: WEB-04, WEB-05
**Success Criteria** (what must be TRUE):
  1. Diagnostic information displays clearly in web interface
  2. Mobile devices provide full functionality with touch-friendly controls
**Research**: Unlikely (extending established web interface patterns)
**Plans**: TBD

### Phase 6: Plugin System Foundation
**Goal**: Users can extend functionality with custom effects and integrations
**Depends on**: Phase 5
**Requirements**: EXT-01, EXT-02, EXT-03
**Success Criteria** (what must be TRUE):
  1. User can load custom RGB effect plugins from local files
  2. Simple custom effects can be created without programming
  3. Plugin API provides safe, sandboxed access to device controls
**Research**: Likely (plugin architecture, security sandboxing)
**Research topics**: Rust plugin systems (dynamic loading vs WASM), sandboxing approaches, effect DSL design
**Plans**: TBD

### Phase 7: Plugin Ecosystem
**Goal**: Community can share and distribute extensions easily
**Depends on**: Phase 6
**Requirements**: EXT-04, EXT-05
**Success Criteria** (what must be TRUE):
  1. User can easily share custom effects with other users
  2. Third-party applications (Discord, OBS) can control device lighting
**Research**: Likely (distribution mechanisms, third-party integration APIs)
**Research topics**: Plugin distribution formats, Discord Rich Presence API, OBS plugin interface
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6 → 7

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Performance Foundation | 0/TBD | Not started | - |
| 2. Enhanced Diagnostics | 0/TBD | Not started | - |
| 3. Advanced Device Support | 0/TBD | Not started | - |
| 4. Web Interface Core | 0/TBD | Not started | - |
| 5. Web Interface Polish | 0/TBD | Not started | - |
| 6. Plugin System Foundation | 0/TBD | Not started | - |
| 7. Plugin Ecosystem | 0/TBD | Not started | - |