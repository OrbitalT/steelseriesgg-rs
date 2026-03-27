<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# mock_udev/

## Purpose

Mock udev library for CI environments where `libudev` is not available. Provides a stub `libudev.a` static library so `hidapi` (with `linux-native-basic-udev` feature) can link successfully in CI without real udev support.

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `lib/` | Pre-compiled mock static library (`libudev.a`) |
| `include/` | Mock header files for udev symbols |
| `pkgconfig/` | pkg-config `.pc` file pointing to mock library |

## For AI Agents

### Working In This Directory

- Do NOT modify `lib/libudev.a` — it is a pre-built binary stub used only in CI
- The mock is activated by `setup_mock_udev.sh` in the root, which sets `PKG_CONFIG_PATH` to point here
- When CI fails with linker errors related to `udev_*` symbols, check that `setup_mock_udev.sh` ran correctly
- This is a CI-only workaround — real hardware tests require a Linux system with actual `libudev`

### Common Patterns

```bash
# Activate mock udev for CI build
source ./setup_mock_udev.sh
cargo build
```

<!-- MANUAL: -->
