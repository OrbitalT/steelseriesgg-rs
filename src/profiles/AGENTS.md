<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# src/profiles/

## Purpose

Profile management system — load, save, and list named device configurations. Profiles store keyboard RGB settings, headset audio settings, and other device state as JSON files in `~/.config/ssgg/profiles/`.

## Key Files

| File | Description |
|------|-------------|
| `mod.rs` | `Profile` struct (optional `KeyboardProfile` + `HeadsetProfile`), `ProfileManager` (load/save/list/apply), JSON serialization |
| `mod.rs.orig` | Backup of original module — can be deleted if no longer needed |
| `tests.rs` | Unit tests for profile serialization, load/save round-trips |

## For AI Agents

### Working In This Directory

- Profile files are JSON — `serde`/`serde_json` handle serialization
- Profile storage path: `~/.config/ssgg/profiles/` (resolved via `directories` crate)
- `mod.rs.orig` is a stale backup — safe to delete if not needed
- When adding new device settings to `KeyboardProfile` or `HeadsetProfile`, ensure backward-compatible serde defaults (`#[serde(default)]`) so old profiles still load

### Testing Requirements

```bash
cargo test profile                  # profile-specific tests
cargo test                          # full suite
```

### Common Patterns

```rust
// Load profile
let manager = ProfileManager::new()?;
let profile = manager.load("my-profile")?;

// Save profile
manager.save("my-profile", &profile)?;

// List profiles
let names = manager.list()?;
```

## Dependencies

### Internal
- `src/config/` — base config directory resolution
- `src/rgb/` — `Color`, `Effect` types stored in `KeyboardProfile`
- `src/error.rs` — `Result` type

### External
- `serde`/`serde_json` — profile serialization
- `directories` — user config path resolution

<!-- MANUAL: -->
