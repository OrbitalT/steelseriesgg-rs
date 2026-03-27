<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# src/devices/

## Purpose

Device abstraction layer — the core of all hardware communication. Contains HID report construction, device discovery, hot-plug support, key/zone mapping, and fuzzing utilities. All HID writes go through `HidOptimizer` for deduplication.

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | Device traits (`Device`, `Keyboard`, `Headset`), `HidOptimizer` singleton (OnceLock), known device PIDs |
| `discovery.rs` | `DeviceManager` — hidapi enumeration, `DeviceInfo`, `DeviceFingerprint`, hot-plug event channel |
| `hid_reports.rs` | **Source of truth** for HID report format — `HidReportBuilder`, `CommandCode` enum, `KEYBOARD_REPORT_SIZE` (65), `HEADSET_REPORT_SIZE` (64) |
| `key_mapping.rs` | Maps logical key names to HID key codes for per-key RGB |
| `zone_mapping.rs` | Maps RGB zone names to byte positions in HID reports |
| `diagnostics.rs` | Device-level diagnostics for `bug-report` and `validate` subcommands |
| `fuzz.rs` | Hidden `fuzz` subcommand implementation — raw HID byte sending for protocol research |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `keyboards/` | Apex series keyboard implementations (see `keyboards/AGENTS.md`) |
| `headsets/` | Arctis series headset implementations (see `headsets/AGENTS.md`) |

## For AI Agents

### Working In This Directory

- **Never construct raw byte arrays** — always use `HidReportBuilder` from `hid_reports.rs`
- **Always use `CommandCode` enum** — never hardcode command byte literals
- `HidOptimizer` silently skips identical reports within 50ms — document this when relevant
- `hidapi` is pinned at `=2.6.5` — do NOT change this constraint in `Cargo.toml`
- Use `rustix::process::getuid()` for permission checks — not raw `libc`

### HID Report Sizes

| Device Type | Size | Report ID |
|-------------|------|-----------|
| Keyboard | 65 bytes | Yes (position 0) |
| Headset | 64 bytes | No |

### Testing Requirements

```bash
cargo test                          # unit tests
cargo clippy --all-targets --locked -- -D warnings
```

Physical hardware tests require a connected SteelSeries device and udev rules installed.

### Common Patterns

**HID Communication Pipeline**
```
CLI command
  → HidCommand trait impl
  → HidReportBuilder::build_report()
  → write_padded_report() [HidOptimizer dedup]
  → hidapi::HidDevice::write()
```

**Device Discovery**
```
DeviceManager::new()
  → hidapi enumerate
  → match PID → Keyboard or Headset
  → DeviceInfo + DeviceFingerprint
  → hot-plug event channel (HotPlugEvent)
```

## Dependencies

### Internal
- `src/error.rs` — error types
- `src/rgb/` — RGB color/effect types passed to keyboard commands

### External
- `hidapi =2.6.5` — USB HID communication
- `parking_lot` — `RwLock` in HidOptimizer
- `rustix` — safe syscall for UID check

<!-- MANUAL: -->
