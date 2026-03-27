<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# assets/

## Purpose

System integration files for deploying `ssgg` as a Linux system service. Contains udev rules for non-root HID access and a systemd service unit.

## Key Files

| File | Description |
|------|-------------|
| `99-steelseries.rules` | udev rules granting non-root access to SteelSeries HID devices — required for `ssgg` to communicate with hardware without `sudo` |
| `ssgg.service` | systemd service unit for running `ssgg daemon` as a user or system service |

## For AI Agents

### Working In This Directory

- When adding support for a new device PID, **update `99-steelseries.rules`** to include the new USB product ID
- The udev rules use `SUBSYSTEM=="hidraw"` and `ATTRS{idVendor}=="1038"` (SteelSeries vendor ID) patterns
- Install location: `/etc/udev/rules.d/99-steelseries.rules` — reload with `sudo udevadm control --reload-rules && sudo udevadm trigger`
- The `PKGBUILD` in the root installs these files automatically for Arch Linux

### Common Patterns

- udev rule format: `SUBSYSTEM=="hidraw", ATTRS{idVendor}=="1038", ATTRS{idProduct}=="XXXX", TAG+="uaccess"`
- Reload udev after rule changes: `sudo udevadm control --reload-rules`

<!-- MANUAL: -->
