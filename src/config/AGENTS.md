<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# src/config/

## Purpose

Configuration file handling for `~/.config/ssgg/config.toml`. Provides the `Config` struct with application defaults (default profile, daemon settings, log level, etc.) that persist across CLI invocations.

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | `Config` struct with serde defaults, load/save via TOML, path resolution via `directories` crate |

## For AI Agents

### Working In This Directory

- Config format is **TOML** (not JSON — profiles use JSON, config uses TOML)
- Config path: `~/.config/ssgg/config.toml`
- Always use `#[serde(default)]` on new config fields for backward compatibility
- `directories` crate resolves the config directory cross-platform

### Common Patterns

```rust
// Load config (creates default if missing)
let config = Config::load()?;

// Save config
config.save()?;

// Access config dir
let dir = Config::config_dir()?; // ~/.config/ssgg/
```

## Dependencies

### Internal
- `src/error.rs` — `Result` type

### External
- `toml 1.1.0` — TOML parsing/serialization
- `serde` — derive macros
- `directories 6.0.0` — user config directory

<!-- MANUAL: -->
