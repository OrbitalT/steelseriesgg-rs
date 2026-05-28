# Design: Apex Pro TKL 2023 — Full RE + Implementation

**Date:** 2026-05-27
**Status:** Approved
**Scope:** Per-key RGB, actuation read-back + Rapid Trigger, Windows HID cross-platform support
**Out of scope:** OLED display (deferred), Sonar/audio (separate feature gate)

---

## Context

The `steelseriesgg-rs` project implements a Linux replacement for SteelSeries GG. The Apex Pro TKL 2023 (VID `0x1038`, PID `0x1628`) is the developer's primary device — it is **live and connected** on the development machine running Windows 11.

Current implementation gaps:
- `CommandCode::PerKeyRgb (0x23)` is a placeholder; the actual packet format is unverified
- `CommandCode::ActuationControl (0x2D)` write works; read-back and Rapid Trigger commands are unknown
- `send_feature` on non-Unix platforms returns a `DeviceCommunication` error stub — nothing actually works on Windows

All gap resolutions depend on confirmed HID byte data. This spec defines the RE methodology to obtain that data and the implementation plan to apply it.

---

## Phase A: Tool Installation

Install via winget (automated, no user interaction after approval):

| Tool | winget ID | Purpose |
|------|-----------|---------|
| Wireshark (includes tshark) | `WiresharkFoundation.Wireshark` | USB capture CLI |
| USBPcap | `DesowinApps.USBPcap` | Kernel USB capture driver |
| .NET SDK (if missing) | `Microsoft.DotNet.SDK.8` | Required for ilspycmd |
| ilspycmd | `dotnet tool install -g ilspycmd` | Decompile GG .NET assemblies |

Community sources (fetch via git clone or GitHub archive, no install):
- `AstroSnail/apexctl` — C, shows HID interface/command patterns for Apex keyboards
- `Askannz/msi-perkeyrgb` — Python, documented `0x0b`/`0x0e` per-key packet layout captured from SteelSeries hardware (MSI uses SteelSeries keyboards)
- `not-jan/apex-tux` — Rust, OLED + OLED init sequence patterns, same hidapi as ssgg

---

## Phase B: Live USB Capture

**Interface discovery:**
```
tshark -D   →   find interface matching VID_1038/PID_1628
```
The keyboard exposes 5 interfaces (MI_00–MI_04). MI_01 and MI_04 are HID vendor-defined devices; one of these is the control interface for RGB/actuation.

**Capture procedure (scripted):**
1. Start background capture: `tshark -i <usb-iface> -w capture-<label>.pcapng`
2. Trigger GG action via its HTTP GameSense API or GG UI:
   - Static solid color (red, green, blue, white) → per-zone then per-key effect
   - Brightness 0% → 50% → 100%
   - Actuation level change (e.g., 0.8 mm → 2.0 mm)
   - Rapid Trigger toggle on/off
3. Stop capture
4. Extract HID OUT reports (host→device, interrupt transfers):
   ```
   tshark -r capture.pcapng \
     -Y "usb.transfer_type==0x01 && usb.endpoint_address.direction==0" \
     -T fields -e usb.capdata
   ```

**One capture per distinct operation** to keep analysis clean.

---

## Phase C: Static RE

Run in parallel with captures:

### ILSpy decompilation
```
ilspycmd "C:\Program Files\SteelSeries\GG\GG.Interop.Services.dll" -o decompiled/
ilspycmd "C:\Program Files\SteelSeries\GG\GG.Models.dll" -o decompiled/
```
Look for: `HidCommand`, `PerKeyRgb`, `SetColor`, `SendReport`, `RapidTrigger`, `ActuationPoint`, `OmniPoint`.

### Community source analysis
- **msi-perkeyrgb**: Extract `0x0b` (533-byte effect definition) and `0x0e` (key assignment) structures. Compare command bytes against Apex Pro TKL captures.
- **apexctl**: Extract HID write calls and their byte offsets. Note interface number used.
- **apex-tux**: Extract OLED init sequence (feature report structure). Note if per-key commands appear.

### GG config/DB cross-reference
Already extracted in `docs/development/`:
- `engine-db-schema.md` — `devices.settings` BLOB structure with `hall_thresholds`, `rapid_trigger_sensitivity`, `release_mode`
- `prism-schema.md` — 84 zones × (x,y,hid_code) table, zone classification

---

## Phase D: Protocol Documentation

After captures are analyzed, document findings in `docs/development/protocol-keyboard.md`:

| Command | Expected source | Format to document |
|---------|----------------|-------------------|
| Per-key RGB | Capture + msi-perkeyrgb cross-ref | Report ID, command byte, key batch structure, commit sequence |
| Actuation read-back | Capture while GG reads actuation | Query byte, response format |
| Rapid Trigger | Capture while RT toggled | Enable/disable byte, sensitivity parameter |

Each command gets a "CONFIRMED" or "INFERRED + NEEDS HARDWARE TEST" marker.

---

## Phase E: Implementation

### E1 — Per-key RGB (`src/devices/hid_reports.rs`, `src/devices/keyboards/apex_pro_tkl_2023.rs`)

Remove the `0x23` placeholder. Implement the verified packet structure:
- New `PerKeyBatchCommand` struct (or update existing `PerKeyRgbCommand`) with confirmed command byte and per-key field layout
- Gated behind `#[cfg(feature = "experimental-apex-2023")]` until hardware-tested on Linux
- `ApexProTkl2023::set_key_color(key: KeyId, color: Color)` and `set_key_colors(pairs: &[(KeyId, Color)])`
- Uses `KeyId → HID code` mapping already in `key_mapping.rs`

### E2 — Actuation read-back + Rapid Trigger (`src/devices/hid_reports.rs`, `apex_pro_tkl_2023.rs`)

- `ActuationReadCommand` — query current actuation point; parses firmware response
- `RapidTriggerCommand { enabled: bool, sensitivity_mm: f32 }` — enable/disable RT with sensitivity
- `ApexProTkl2023::get_actuation_point() -> Result<f32>` (read-back)
- `ApexProTkl2023::set_rapid_trigger(enabled: bool, sensitivity_mm: f32) -> Result<()>`

### E3 — Windows HID cross-platform (`src/devices/keyboards/mod.rs`)

Replace the `#[cfg(not(unix))]` stub in `send_feature` with hidapi's cross-platform feature report API:
```rust
// Before (non-unix stub — always errors):
#[cfg(not(unix))]
pub fn send_feature(&self, _data: &[u8], _report_len: usize) -> Result<()> {
    Err(Error::DeviceCommunication("Unix only".to_string()))
}

// After (cross-platform via hidapi):
#[cfg(not(unix))]
pub fn send_feature(&self, data: &[u8], _report_len: usize) -> Result<()> {
    self.device.send_feature_report(data)
        .map_err(|e| Error::DeviceCommunication(e.to_string()))
}
```
`hidapi` already calls `HidD_SetFeature` on Windows and `HIDIOCSFEATURE` ioctl on Linux. The Unix path keeps its raw hidraw ioctl for interface routing flexibility; the Windows path uses hidapi's built-in API.

Interface selection on Windows (for control interface MI_01 vs MI_00):
- Current Unix path: `find_hidraw_for_interface(vid, pid, interface_num)`
- Windows path: enumerate hidapi devices with same VID/PID, filter by `usage_page == 0xFF00` (vendor-defined) or `interface_number == 1`

---

## Phase F: QA

```
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo test --locked --features experimental-apex-2023
cargo build --locked
cargo build --locked --features experimental-apex-2023
```

Hardware smoke test (Windows, device connected):
- `ssgg rgb solid "#FF0000"` — red zone color (confirms zone RGB path)
- If per-key implemented: manual per-key test via `ssgg` CLI

---

## Constraints

From `CLAUDE.md` (must be preserved throughout):
- Always use `HidReportBuilder` and typed command structs — no raw byte arrays
- `hidapi = "=2.6.6"` pinned — do not change
- No `.unwrap()` / `.expect()` in production paths
- `experimental-apex-2023` stays behind feature flag until hardware-verified
- Do not state any protocol as confirmed unless a capture confirms it

---

## Success Criteria

- [ ] `cargo build --locked` and `cargo test --locked` pass on Windows and Linux
- [ ] Clippy clean with `-D warnings`
- [ ] Per-key RGB command bytes documented in `docs/development/protocol-keyboard.md` with source evidence
- [ ] Actuation read-back: `get_actuation_point()` returns current value from device
- [ ] Rapid Trigger: `set_rapid_trigger(true, 0.2)` sends verified HID report
- [ ] `send_feature` works on Windows (returns real result, not stub error)
- [ ] All new commands labelled CONFIRMED or EXPERIMENTAL in docs and code comments
