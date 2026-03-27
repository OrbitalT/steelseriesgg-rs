<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# src/bin/

## Purpose

Standalone utility binaries for protocol research, benchmarking, and hardware verification. These are separate Cargo binaries (not subcommands of `ssgg`) used primarily for development and debugging.

## Key Files

| File | Description |
|------|-------------|
| `discover_actuation.rs` | Reads current actuation point settings from connected Apex Pro devices |
| `sonar_control.rs` | CLI for direct SteelSeries Sonar HTTP API control (requires `sonar` feature) |
| `verify_key_mapping.rs` | Verifies that logical key names map to correct HID codes (development tool) |
| `benchmark_fragment.rs` | Benchmarks HID report fragment operations for performance tuning |
| `benchmark_rgb_alloc.rs` | Benchmarks RGB color allocation paths to identify memory hotspots |

## For AI Agents

### Working In This Directory

- Error handling: `anyhow::Context` + `?` — never `.unwrap()` or `.expect()`
- These binaries are for **development use only** — not installed by PKGBUILD or systemd service
- `sonar_control.rs` requires `--features sonar` to compile
- Benchmark binaries are not run in CI — they require physical hardware or profiling setup

### Testing Requirements

```bash
cargo build --bin discover_actuation
cargo build --bin sonar_control --features sonar
cargo build --bin verify_key_mapping
```

### Common Patterns

```rust
// Standard bin entry point pattern
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let device = DeviceManager::find_first()
        .context("no SteelSeries device found")?;
    // ...
    Ok(())
}
```

## Dependencies

### Internal
- `src/devices/` — device discovery and HID communication
- `src/audio/` — Sonar client (sonar_control.rs)

### External
- `anyhow` — error context for binary code
- `reqwest` — HTTP (sonar_control.rs, `sonar` feature)

<!-- MANUAL: -->
