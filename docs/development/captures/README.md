# captures/

Frida-based HID capture sessions from SteelSeriesEngine.exe process.

## Files

| Pattern | Description |
|---------|-------------|
| `frida-selftest*.txt` | Frida self-test / hook validation runs |
| `frida-hid-cap*.txt` | Raw HID report captures (all interfaces) |
| `frida-nt-<pid>.txt` | NT syscall captures for process `<pid>` |
| `frida-full-<pid>*.txt` | Full capture including HID + NT for process `<pid>` |

## Key Finding

Per-key RGB for Apex Pro TKL 2023 uses `IOCTL_HID_SET_FEATURE` (code `0x000B0191`) with a 645-byte report.
Command byte `0x40`, 84 key entries of `[hid_code R G B]`, sent to MI_01 (vendor-defined interface).

See `../protocol-keyboard.md` for the full protocol reference.

## Still Needed

USB capture (Wireshark + USBPcap) of:
- Actuation read-back command
- Rapid Trigger enable/disable
- Per-key actuation thresholds
