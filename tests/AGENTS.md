<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# tests/

## Purpose

Integration test suite for steelseriesgg-rs. Contains Rust integration tests (compiled by `cargo test`) and shell scripts for manual hardware verification.

## Key Files

| File | Description |
|------|-------------|
| `cors_security.rs` | Integration tests verifying GameSense server CORS is restricted to `127.0.0.1` — must not regress |
| `device_readback.rs` | Tests for reading device state back after writing (requires physical hardware) |
| `security_diagnostics.rs` | Security-related diagnostic tests |
| `device_response.sh` | Shell script for manual device response testing |
| `test_rgb.sh` | Shell script to verify RGB functionality on physical hardware |
| `verify_rgb_physical.sh` | Physical RGB verification — run against real device to confirm color output |

## For AI Agents

### Working In This Directory

- `cors_security.rs` is a **critical security test** — never weaken its assertions; any change to CORS config must keep these tests passing
- Shell scripts require physical SteelSeries hardware and udev rules in place (`assets/99-steelseries.rules`)
- Rust integration tests run via `cargo test` — CI executes these automatically on push

### Testing Requirements

```bash
# Run all integration tests
cargo test

# Run specific test by name
cargo test cors

# Run with sonar feature
cargo test --features sonar
```

### Common Patterns

- Rust tests in this directory are integration tests — they have access to the full `steelseries_gg` library
- Shell scripts are for physical hardware validation only — not automated in CI

## Dependencies

### Internal
- `src/gamesense/` — tested by `cors_security.rs`
- `src/devices/` — tested by `device_readback.rs`

<!-- MANUAL: -->
