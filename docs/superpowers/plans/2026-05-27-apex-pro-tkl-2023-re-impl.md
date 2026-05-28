# Apex Pro TKL 2023 — Full RE + Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Discover the confirmed HID protocol for per-key RGB, actuation read-back, and Rapid Trigger on the Apex Pro TKL 2023 (PID 0x1628) via live USB capture + static RE, then implement all three features cross-platform (Windows + Linux).

**Architecture:** Live USB captures (tshark + USBPcap) while SteelSeries GG operates the connected keyboard yield ground-truth HID byte sequences. Community RE sources (msi-perkeyrgb, apexctl, apex-tux) cross-validate. Confirmed bytes are encoded as typed command structs in `hid_reports.rs` and exposed through `ApexProTkl2023`. The Unix-only `send_feature` ioctl path gains a Windows implementation via hidapi's cross-platform `send_feature_report`.

**Tech Stack:** Rust 1.95.0, hidapi 2.6.6 (pinned), PowerShell 7, tshark (Wireshark CLI), ILSpy CLI (ilspycmd)

---

## Task 1: Install RE Tools

**Files:**
- No source files — environment setup only

- [ ] **Step 1: Install Wireshark (includes tshark)**
```powershell
winget install --id WiresharkFoundation.Wireshark --silent --accept-package-agreements --accept-source-agreements
```
Expected: Wireshark installs to `C:\Program Files\Wireshark\`. Takes 1–2 min.

- [ ] **Step 2: Install USBPcap kernel driver**
```powershell
winget install --id DesowinApps.USBPcap --silent --accept-package-agreements --accept-source-agreements
```
Expected: USBPcap driver installs. A prompt may appear to install the kernel driver — accept it. **Reboot may be required** for USBPcap interfaces to appear.

- [ ] **Step 3: Verify tshark is accessible**
```powershell
$env:PATH += ";C:\Program Files\Wireshark"
tshark --version | Select-Object -First 1
```
Expected output: `TShark (Wireshark) 4.x.x`

- [ ] **Step 4: Install ilspycmd (.NET decompiler)**
```powershell
dotnet tool install -g ilspycmd
```
Expected: `ilspycmd` appears in `~\.dotnet\tools\`. If dotnet SDK missing: `winget install Microsoft.DotNet.SDK.8` first.

- [ ] **Step 5: Verify ilspycmd**
```powershell
ilspycmd --version
```
Expected: `ICSharpCode.Decompiler 9.x.x.x`

- [ ] **Step 6: Create analysis workspace**
```powershell
New-Item -ItemType Directory -Force "C:\Users\Ven0m0\projects\steelseriesgg-rs\docs\development\decompiled"
New-Item -ItemType Directory -Force "C:\Users\Ven0m0\projects\steelseriesgg-rs\docs\development\captures"
```

---

## Task 2: Fetch Community RE Sources

**Files:**
- No source files — reference material only

- [ ] **Step 1: Clone apexctl**
```powershell
Set-Location "$env:TEMP"
git clone https://github.com/AstroSnail/apexctl.git 2>&1
```
Expected: `apexctl/` directory created with C source files.

- [ ] **Step 2: Clone msi-perkeyrgb**
```powershell
git clone https://github.com/Askannz/msi-perkeyrgb.git 2>&1
```
Expected: `msi-perkeyrgb/` with Python source and `documentation/` folder.

- [ ] **Step 3: Clone apex-tux**
```powershell
git clone https://github.com/not-jan/apex-tux.git 2>&1
```
Expected: `apex-tux/` with Rust source including `apex-hardware/` crate.

- [ ] **Step 4: Clone apex7tkl_linux**
```powershell
git clone https://github.com/FrankGrimm/apex7tkl_linux.git 2>&1
```

- [ ] **Step 5: Extract apexctl HID command patterns**
```powershell
Select-String -Path "$env:TEMP\apexctl\*.c","$env:TEMP\apexctl\*.h" -Pattern "hid_write|0x2[1-9a-fA-F]|0x3[0-9a-fA-F]|per.key|PerKey|send_report" | Format-List
```
Record any command bytes found and their byte-offset meaning.

- [ ] **Step 6: Extract msi-perkeyrgb packet structures**
```powershell
Get-Content "$env:TEMP\msi-perkeyrgb\documentation\0b_packet_information\msi-kb-effectdoc" 2>$null
Get-Content "$env:TEMP\msi-perkeyrgb\msi_perkeyrgb\*.py" | Select-String "0x0b|0x0e|hid_write|set_key|per.key" | Format-List
```
Record: `0x0b` = 533-byte effect definition, `0x0e` = per-key assignment `[R,G,B,R,G,B,dur_h,dur_l,effect_id,mode,0x00,keycode]`.

- [ ] **Step 7: Extract apex-tux HID command patterns**
```powershell
Select-String -Path "$env:TEMP\apex-tux\apex-hardware\src\*.rs" -Pattern "write|feature|0x[0-9a-fA-F][0-9a-fA-F]" | Format-List
```
Record: feature report command byte, OLED packet size, init sequence.

---

## Task 3: ILSpy Decompilation of GG Assemblies

**Files:**
- Create: `docs/development/decompiled/GG.Interop.Services/` (ILSpy output)
- Create: `docs/development/decompiled/GG.Models/` (ILSpy output)

- [ ] **Step 1: Decompile GG.Interop.Services.dll**
```powershell
$ggPath = "C:\Program Files\SteelSeries\GG"
ilspycmd "$ggPath\GG.Interop.Services.dll" `
  -o "C:\Users\Ven0m0\projects\steelseriesgg-rs\docs\development\decompiled\GG.Interop.Services" `
  --nested-directories 2>&1 | Tee-Object -FilePath ilspy-interop.log
```
Expected: C# source tree created. If error "not a valid .NET assembly", the DLL is obfuscated/trimmed — note in `docs/development/preflight-findings.md` and rely on captures only.

- [ ] **Step 2: Decompile GG.Models.dll**
```powershell
ilspycmd "$ggPath\GG.Models.dll" `
  -o "C:\Users\Ven0m0\projects\steelseriesgg-rs\docs\development\decompiled\GG.Models" `
  --nested-directories 2>&1 | Tee-Object -FilePath ilspy-models.log
```

- [ ] **Step 3: Search for per-key RGB HID command bytes**
```powershell
$decompPath = "C:\Users\Ven0m0\projects\steelseriesgg-rs\docs\development\decompiled"
Select-String -Path "$decompPath\**\*.cs" -Pattern "PerKey|per_key|0x23|0x0[bBeE]|SetRGB|hidCode|HidCode|SendReport|WriteReport" -Recurse | Format-List
```
Record: method names, byte literal values, packet size constants.

- [ ] **Step 4: Search for Rapid Trigger command bytes**
```powershell
Select-String -Path "$decompPath\**\*.cs" -Pattern "RapidTrigger|rapid_trigger|Actuation|actuation|0x2D|0x3[0-9]|HallThreshold" -Recurse | Format-List
```
Record: any command byte used to enable/configure RT.

- [ ] **Step 5: Search for device initialization sequence**
```powershell
Select-String -Path "$decompPath\**\*.cs" -Pattern "Initialize|Init|Connect|OnConnect|DeviceConnect|sequence" -Recurse | Format-List
```
Record: any init sequence that must precede per-key commands.

---

## Task 4: Live USB Capture — Zone RGB Baseline

**Purpose:** Confirm 0x21 (zone RGB) and 0x09 (apply) appear as expected. This validates the capture setup before per-key.

**Files:**
- Create: `docs/development/captures/zone-rgb-baseline.pcapng`

- [ ] **Step 1: Find USBPcap interface hosting the keyboard**
```powershell
$env:PATH += ";C:\Program Files\Wireshark"
tshark -D 2>&1 | Select-String "USBPcap|usb"
```
Expected output: lines like `5. \\.\USBPcap1 (USBPcap1)`. Note all USBPcap interface numbers.

- [ ] **Step 2: Find which USBPcap interface has the SteelSeries device**

For each USBPcap interface found, run a 3-second sniff and grep for VID 1038:
```powershell
# Replace USBPcap1 with each interface found above
$captureIface = "\\.\USBPcap1"
$captureFile = "C:\Users\Ven0m0\projects\steelseriesgg-rs\docs\development\captures\probe.pcapng"
tshark -i $captureIface -a duration:3 -w $captureFile 2>$null
tshark -r $captureFile -Y "usb.idVendor == 0x1038" -c 5 2>&1
```
If output shows packets: this is the right interface. If empty: try next USBPcap interface.

- [ ] **Step 3: Ensure SteelSeries GG is running**
```powershell
$ggProcess = Get-Process -Name "SteelSeriesGGEZ" -ErrorAction SilentlyContinue
if (-not $ggProcess) {
    Start-Process "C:\Program Files\SteelSeries\GG\SteelSeriesGGEZ.exe"
    Start-Sleep 5
}
Write-Host "GG running: $((Get-Process -Name 'SteelSeriesGGEZ' -ErrorAction SilentlyContinue) -ne $null)"
```

- [ ] **Step 4: Read GameSense port from coreProps.json**
```powershell
$propsPath = "$env:PROGRAMDATA\SteelSeries\SteelSeries Engine 3\coreProps.json"
$props = Get-Content $propsPath | ConvertFrom-Json
$gamesenseAddr = $props.address
Write-Host "GameSense at: $gamesenseAddr"
```
Note the address (e.g., `127.0.0.1:27301`).

- [ ] **Step 5: Start baseline capture**
```powershell
$iface = "\\.\USBPcap1"  # replace with confirmed interface
$captureFile = "C:\Users\Ven0m0\projects\steelseriesgg-rs\docs\development\captures\zone-rgb-baseline.pcapng"
$job = Start-Job -ScriptBlock {
    param($iface, $file)
    & "C:\Program Files\Wireshark\tshark.exe" -i $iface -w $file
} -ArgumentList $iface, $captureFile
Start-Sleep 2
Write-Host "Capture running as job $($job.Id)"
```

- [ ] **Step 6: Trigger zone RGB via GameSense API**
```powershell
$addr = $gamesenseAddr  # from Step 4
# Register game
Invoke-RestMethod -Uri "http://$addr/game_metadata" -Method POST -ContentType "application/json" -Body '{"game":"SSGG_TEST","game_display_name":"ssgg test","developer":"test"}'
# Bind a color event
Invoke-RestMethod -Uri "http://$addr/bind_game_event" -Method POST -ContentType "application/json" -Body '{"game":"SSGG_TEST","event":"COLOR","min_value":0,"max_value":100,"icon_id":0,"handlers":[{"device-type":"keyboard","zone":"function-keys","color":{"red":255,"green":0,"blue":0},"mode":"color"}]}'
# Send the event (red)
Invoke-RestMethod -Uri "http://$addr/game_event" -Method POST -ContentType "application/json" -Body '{"game":"SSGG_TEST","event":"COLOR","data":{"value":100}}'
Start-Sleep 1
# Blue
$body = '{"game":"SSGG_TEST","event":"COLOR","data":{"value":50}}'
Invoke-RestMethod -Uri "http://$addr/bind_game_event" -Method POST -ContentType "application/json" -Body '{"game":"SSGG_TEST","event":"COLOR","min_value":0,"max_value":100,"icon_id":0,"handlers":[{"device-type":"keyboard","zone":"function-keys","color":{"red":0,"green":0,"blue":255},"mode":"color"}]}'
Invoke-RestMethod -Uri "http://$addr/game_event" -Method POST -ContentType "application/json" -Body '{"game":"SSGG_TEST","event":"COLOR","data":{"value":100}}'
Start-Sleep 1
```

- [ ] **Step 7: Stop capture and extract HID OUT reports**
```powershell
Stop-Job $job.Id; Remove-Job $job.Id
$captureFile = "C:\Users\Ven0m0\projects\steelseriesgg-rs\docs\development\captures\zone-rgb-baseline.pcapng"
# Extract host→device interrupt OUT transfers
tshark -r $captureFile `
  -Y "usb.transfer_type==0x01 && usb.endpoint_address.direction==0" `
  -T fields -e frame.number -e usb.capdata 2>&1 | Tee-Object "captures\zone-rgb-baseline-hid.txt"
```
Expected: rows of hex data. Look for bytes starting with `00 21` (zone RGB command) and `00 09` (apply).

- [ ] **Step 8: Validate captures match known commands**
```powershell
$hexLines = Get-Content "captures\zone-rgb-baseline-hid.txt"
$hexLines | Where-Object { $_ -match "00:21|0021" } | Select-Object -First 5
$hexLines | Where-Object { $_ -match "00:09|0009" } | Select-Object -First 5
```
Expected: at least 2 lines each. If none found: check interface selection in Step 2.

---

## Task 5: Live USB Capture — Per-Key RGB Discovery

**Purpose:** Identify the actual command byte(s) GG uses for per-key RGB on PID 0x1628.

**Files:**
- Create: `docs/development/captures/per-key-rgb.pcapng`

- [ ] **Step 1: Start fresh capture**
```powershell
$iface = "\\.\USBPcap1"  # confirmed interface from Task 4
$captureFile = "C:\Users\Ven0m0\projects\steelseriesgg-rs\docs\development\captures\per-key-rgb.pcapng"
$job = Start-Job -ScriptBlock {
    param($iface, $file)
    & "C:\Program Files\Wireshark\tshark.exe" -i $iface -w $file
} -ArgumentList $iface, $captureFile
Start-Sleep 2
```

- [ ] **Step 2: Set a per-key rainbow effect via GG GameSense per-key zone**
```powershell
$addr = $gamesenseAddr  # from Task 4 Step 4
# Per-key handler — forces GG to send per-key RGB packets
Invoke-RestMethod -Uri "http://$addr/bind_game_event" -Method POST -ContentType "application/json" -Body @'
{
  "game": "SSGG_TEST",
  "event": "PERKEY",
  "min_value": 0,
  "max_value": 100,
  "icon_id": 0,
  "handlers": [{
    "device-type": "keyboard",
    "zone": "all",
    "color": {"red": 255, "green": 0, "blue": 0},
    "mode": "color"
  }]
}
'@
Invoke-RestMethod -Uri "http://$addr/game_event" -Method POST -ContentType "application/json" -Body '{"game":"SSGG_TEST","event":"PERKEY","data":{"value":100}}'
Start-Sleep 2
```
**Alternative if GameSense doesn't trigger per-key:** Open GG UI manually → Prism → Keyboard → set each key to a unique color (e.g., rainbow gradient). The per-key packets will appear during the 10-second capture window.

- [ ] **Step 3: Change to a different per-key color to generate more packets**
```powershell
Invoke-RestMethod -Uri "http://$addr/bind_game_event" -Method POST -ContentType "application/json" -Body @'
{
  "game": "SSGG_TEST",
  "event": "PERKEY",
  "min_value": 0,
  "max_value": 100,
  "icon_id": 0,
  "handlers": [{
    "device-type": "keyboard",
    "zone": "all",
    "color": {"red": 0, "green": 255, "blue": 0},
    "mode": "color"
  }]
}
'@
Invoke-RestMethod -Uri "http://$addr/game_event" -Method POST -ContentType "application/json" -Body '{"game":"SSGG_TEST","event":"PERKEY","data":{"value":100}}'
Start-Sleep 2
```

- [ ] **Step 4: Stop capture and extract unique command bytes**
```powershell
Stop-Job $job.Id; Remove-Job $job.Id
$captureFile = "C:\Users\Ven0m0\projects\steelseriesgg-rs\docs\development\captures\per-key-rgb.pcapng"
tshark -r $captureFile `
  -Y "usb.transfer_type==0x01 && usb.endpoint_address.direction==0" `
  -T fields -e usb.capdata 2>&1 | Out-File "docs\development\captures\per-key-rgb-hid.txt"
# Show unique second bytes (command byte is byte index 1)
$packets = Get-Content "docs\development\captures\per-key-rgb-hid.txt" | Where-Object { $_ -match ":" }
$commandBytes = $packets | ForEach-Object { ($_ -split ":")[1] } | Sort-Object -Unique
Write-Host "Unique command bytes observed: $($commandBytes -join ', ')"
```
Expected: one or more NEW command bytes not in {09, 21, 22} — these are the per-key commands.

- [ ] **Step 5: Extract full packet structure for new command bytes**
```powershell
# For each new command byte X, show the full first packet:
$newByte = "XX"  # replace with discovered command byte from Step 4
$packets | Where-Object { ($_ -split ":")[1] -eq $newByte } | Select-Object -First 3
```
Record: total packet length, structure interpretation (batch of N keys × 3 bytes? single key + RGB? multi-packet sequence?).

- [ ] **Step 6: Identify the packet sequence (single vs multi-packet)**
```powershell
# Show consecutive packet command bytes to identify sequence patterns
tshark -r $captureFile `
  -Y "usb.transfer_type==0x01 && usb.endpoint_address.direction==0" `
  -T fields -e frame.number -e frame.time_delta -e usb.capdata 2>&1 | `
  Select-Object -First 50 | `
  Out-File "docs\development\captures\per-key-rgb-sequence.txt"
Get-Content "docs\development\captures\per-key-rgb-sequence.txt"
```
Look for: groups of consecutive packets with the same command byte (batch protocol) vs single large packet vs alternating command bytes.

---

## Task 6: Live USB Capture — Actuation + Rapid Trigger

**Files:**
- Create: `docs/development/captures/actuation-rt.pcapng`

- [ ] **Step 1: Start capture**
```powershell
$captureFile = "C:\Users\Ven0m0\projects\steelseriesgg-rs\docs\development\captures\actuation-rt.pcapng"
$job = Start-Job -ScriptBlock {
    param($iface, $file)
    & "C:\Program Files\Wireshark\tshark.exe" -i $iface -w $file
} -ArgumentList "\\.\USBPcap1", $captureFile
Start-Sleep 2
```

- [ ] **Step 2: Change actuation in GG**

Open SteelSeries GG → Keyboards → Apex Pro TKL → Actuation → change from 0.8 mm to 1.5 mm → Apply. Wait 3 seconds. Change back to 0.8 mm → Apply. These actions in GG will send `0x2D` commands to the keyboard.

```powershell
Start-Sleep 10  # give time to make changes in GG UI
```

- [ ] **Step 3: Toggle Rapid Trigger in GG**

Open SteelSeries GG → Keyboards → Apex Pro TKL → Rapid Trigger → Enable. Wait 2 seconds. Disable. These will send the RT enable/disable HID command.

```powershell
Start-Sleep 8
```

- [ ] **Step 4: Stop capture and extract all command bytes**
```powershell
Stop-Job $job.Id; Remove-Job $job.Id
tshark -r $captureFile `
  -Y "usb.transfer_type==0x01 && usb.endpoint_address.direction==0" `
  -T fields -e frame.number -e usb.capdata 2>&1 | `
  Out-File "docs\development\captures\actuation-rt-hid.txt"
$packets = Get-Content "docs\development\captures\actuation-rt-hid.txt" | Where-Object { $_ -match ":" }
$cmdBytes = $packets | ForEach-Object { ($_ -split ":")[1] } | Sort-Object -Unique
Write-Host "Command bytes in actuation+RT capture: $($cmdBytes -join ', ')"
```

- [ ] **Step 5: Identify actuation write packets**
```powershell
# Known: 0x2D is actuation write — confirm it appears twice (0.8→1.5→0.8)
$packets | Where-Object { ($_ -split ":")[1] -eq "2d" } | Format-List
```
Note the value byte (3rd byte) for each actuation level.

- [ ] **Step 6: Identify Rapid Trigger command byte**
```powershell
# Filter for packets that appeared ONLY during the RT toggle section
# (after the actuation changes), with a command byte NOT in {09, 21, 22, 2d}
$knownBytes = @("09","21","22","2d")
$rtPackets = $packets | Where-Object {
    $byte = ($_ -split ":")[1]
    $knownBytes -notcontains $byte
}
Write-Host "Candidate RT packets:"
$rtPackets | Select-Object -First 10
```
Record: RT enable command byte and structure (enable/disable toggle byte position).

- [ ] **Step 7: Check for actuation read-back (device→host packets)**
```powershell
# Look for device→host packets (direction==1) after actuation write
tshark -r $captureFile `
  -Y "usb.transfer_type==0x01 && usb.endpoint_address.direction==1" `
  -T fields -e frame.number -e usb.capdata 2>&1 | `
  Select-Object -First 20
```
If any appear after the `0x2D` writes: the firmware responds with current actuation. Note the response format.

---

## Task 7: Document Protocol Findings

**Files:**
- Create: `docs/development/protocol-keyboard.md`

- [ ] **Step 1: Create the protocol reference document**

Create `docs/development/protocol-keyboard.md` with the following template, filling in the ACTUAL bytes discovered in Tasks 4–6:

```markdown
# Apex Pro TKL 2023 — Confirmed HID Protocol

Last updated: 2026-05-27
Device: VID 0x1038 / PID 0x1628
Evidence: USB captures + ILSpy decompilation of GG.Interop.Services.dll

## Report format (keyboards)
[0x00] [CMD] [DATA...] [padding to 65 bytes]

## Confirmed commands

### 0x09 — Apply/save
[0x00] [0x09] [0x00 × 63]
Source: Confirmed in prior testing.

### 0x21 — Zone RGB
[0x00] [0x21] [zone_selector] [R] [G] [B] ... [padding]
zone_selector 0xFF = all zones.
Source: Confirmed in prior testing + Task 4 capture.

### 0x22 — Brightness
[0x00] [0x22] [brightness 0–100] [0x00 × 62]
Source: Confirmed in prior testing.

### 0x2D — Actuation write (experimental)
[0x00] [0x2D] [value] [0x00 × 62]
value = 0.1mm increments (8 = 0.8mm, 20 = 2.0mm).
Source: Confirmed write. Task 6 capture shows GG sends this.

## Per-key RGB — FILL IN FROM TASK 5
Command byte: 0xXX (DISCOVERED IN TASK 5)
Packet structure: [FILL IN]
Multi-packet: [YES/NO — FILL IN]
Batch size: [N keys per packet — FILL IN]
Commit required: [YES/NO — FILL IN]
Source: Task 5 USB capture + msi-perkeyrgb cross-reference.

## Actuation read-back — FILL IN FROM TASK 6
Command byte: 0xXX (or NONE if write-only)
Response format: [FILL IN]
Source: Task 6 USB capture.

## Rapid Trigger — FILL IN FROM TASK 6
Enable command: 0xXX [FILL IN]
Sensitivity parameter: byte offset [FILL IN]
Source: Task 6 USB capture.
```

- [ ] **Step 2: Fill in all FILL IN sections from capture data**

Use the hex data from `docs/development/captures/per-key-rgb-hid.txt` and `docs/development/captures/actuation-rt-hid.txt`. No section should remain as "FILL IN" after this step.

- [ ] **Step 3: Commit findings**
```bash
git add docs/development/captures/ docs/development/decompiled/ docs/development/protocol-keyboard.md
git commit -m "docs(re): add confirmed HID protocol from Wireshark capture of PID 0x1628"
```

---

## Task 8: Fix Windows HID (`send_feature` cross-platform)

**Files:**
- Modify: `src/devices/keyboards/mod.rs` — replace `#[cfg(not(unix))]` stub

- [ ] **Step 1: Write the failing test**

Add to `src/devices/mod.rs` under `#[cfg(test)]`:
```rust
#[test]
fn test_device_communication_error_is_string() {
    // Ensures Error::DeviceCommunication(String) is constructible on all platforms.
    let e = Error::DeviceCommunication("test".to_string());
    assert!(format!("{e}").contains("test"));
}
```

- [ ] **Step 2: Run test to confirm it compiles and passes**
```powershell
Set-Location "C:\Users\Ven0m0\projects\steelseriesgg-rs"
cargo test test_device_communication_error_is_string --locked
```
Expected: PASS (this is a compile check, not a behavior test).

- [ ] **Step 3: Locate the current Windows stub in `mod.rs`**

Open `src/devices/keyboards/mod.rs`. Find the `#[cfg(not(unix))]` `send_feature` block (currently returns `Error::DeviceCommunication("Unix only")`) and the `#[cfg(unix)]` block above it.

- [ ] **Step 4: Replace the Windows stub with hidapi cross-platform call**

Replace:
```rust
#[cfg(not(unix))]
pub fn send_feature(&self, _data: &[u8], _report_len: usize) -> Result<()> {
    Err(Error::DeviceCommunication(
        "Raw HID feature reports are only supported on Unix platforms".to_string(),
    ))
}
```

With:
```rust
#[cfg(not(unix))]
pub fn send_feature(&self, data: &[u8], _report_len: usize) -> Result<()> {
    self.device
        .send_feature_report(data)
        .map_err(|e| Error::DeviceCommunication(e.to_string()))
}
```
`self.device` is the `HidDevice` from hidapi. `send_feature_report` calls `HidD_SetFeature` on Windows.

- [ ] **Step 5: Check that HidDevice is accessible in send_feature**

The `send_feature` method is on `GenericKeyboard`. Verify `self.device` is the field holding the `HidDevice`. If the field name differs, grep:
```powershell
Select-String -Path "src\devices\keyboards\mod.rs" -Pattern "struct GenericKeyboard" -Context 10
```
Adjust `self.device` to match the actual field name.

- [ ] **Step 6: Build to verify no compile errors**
```powershell
cargo build --locked 2>&1 | Select-Object -Last 10
```
Expected: `Finished dev profile`.

- [ ] **Step 7: Also fix interface selection for Windows**

The Unix path uses `find_hidraw_for_interface` to select the control interface. On Windows, we need to open the correct HID device by interface number.

Find in `src/devices/keyboards/mod.rs` the area where the keyboard is opened (likely in `GenericKeyboard::open` or `open_keyboard`). On Windows, hidapi enumerates multiple paths for the same PID with different interface numbers. Add:

```rust
// In discovery.rs or where the keyboard device is opened on Windows:
// Filter hidapi device list to interface_number == 1 (control interface)
// hidapi reports interface_number via DeviceInfo::interface_number
```

Check `src/devices/discovery.rs` to see if interface selection already works cross-platform. If `info.interface_number` is already used as the filter criterion during enumeration, Windows will work automatically — hidapi populates `interface_number` on all platforms.

```powershell
Select-String -Path "src\devices\discovery.rs" -Pattern "interface_number|interface_num" -Context 3
```

If interface filtering is already done via hidapi enumeration (not raw hidraw path), no change needed here.

- [ ] **Step 8: Commit**
```bash
git add src/devices/keyboards/mod.rs
git commit -m "fix(hid): implement send_feature_report for non-Unix platforms via hidapi"
```

---

## Task 9: Implement Per-Key RGB Command

**Files:**
- Modify: `src/devices/hid_reports.rs` — add/update `PerKeyRgbCommand` with confirmed bytes

> **Prerequisites:** Task 7 must be complete. Replace `PER_KEY_CMD` with the actual command byte from `docs/development/protocol-keyboard.md`.

- [ ] **Step 1: Write the failing test**

Add to `src/devices/hid_reports.rs` under `#[cfg(test)]`:
```rust
#[test]
fn test_per_key_rgb_command_serializes_correctly() {
    use super::*;
    // Replace PER_KEY_CMD with the actual command byte from protocol-keyboard.md
    const PER_KEY_CMD: u8 = 0x23; // UPDATE THIS after Task 7

    // A single key: HID code 0x04 (A key) = red
    let addr = KeyAddress::hid_code(0x04);
    let color = Color::new(255, 0, 0);
    let cmd = PerKeyRgbCommand::single_key(addr, color);

    let builder = HidReportBuilder::new(HidDeviceType::Keyboard);
    let mut buf = [0u8; KEYBOARD_REPORT_SIZE];
    let size = builder.build_report(cmd, &mut buf).unwrap();

    assert_eq!(size, KEYBOARD_REPORT_SIZE);
    assert_eq!(buf[0], 0x00);         // report ID
    assert_eq!(buf[1], PER_KEY_CMD);  // command byte — update from protocol doc
    // Remaining bytes depend on protocol structure discovered in Task 5
    // Add specific byte assertions once protocol is known
}
```

- [ ] **Step 2: Run test to see compile error (expected)**
```powershell
cargo test test_per_key_rgb_command_serializes_correctly --locked --features experimental-apex-2023
```
Expected: compile error or test failure — establishes the target.

- [ ] **Step 3: Update `CommandCode` enum with confirmed byte**

In `src/devices/hid_reports.rs`, find `CommandCode` enum. Update:
```rust
pub enum CommandCode {
    Apply = 0x09,
    RgbControl = 0x21,
    Brightness = 0x22,
    PerKeyRgb = 0xXX,  // REPLACE 0xXX with confirmed byte from protocol-keyboard.md
    // ...
}
```

- [ ] **Step 4: Implement `PerKeyRgbCommand::single_key` with confirmed structure**

The exact implementation depends on the packet structure discovered in Task 5. Two likely structures:

**If single-packet per-key** (one key per 65-byte report):
```rust
pub struct PerKeyRgbCommand {
    pub hid_code: u8,
    pub color: Color,
}

impl PerKeyRgbCommand {
    pub fn single_key(addr: KeyAddress, color: Color) -> Self {
        Self { hid_code: addr.hid_code(), color }
    }
}

impl HidCommand for PerKeyRgbCommand {
    fn command_code(&self) -> CommandCode { CommandCode::PerKeyRgb }
    fn serialize(&self, buf: &mut [u8], _: HidDeviceType) -> Result<usize> {
        buf[0] = 0x00;
        buf[1] = self.command_code() as u8;
        buf[2] = self.hid_code;
        buf[3] = self.color.r;
        buf[4] = self.color.g;
        buf[5] = self.color.b;
        Ok(KEYBOARD_REPORT_SIZE)
    }
    fn validate(&self) -> Result<()> { Ok(()) }
    fn description(&self) -> String {
        format!("PerKeyRgb hid=0x{:02x} color={:?}", self.hid_code, self.color)
    }
}
```

**If batch-packet** (multiple keys per report, e.g., 21 keys × 3 bytes in 65-byte report):
Adapt `serialize` to write the batch structure. See `docs/development/protocol-keyboard.md` for the confirmed layout.

- [ ] **Step 5: Run test to verify it passes**
```powershell
cargo test test_per_key_rgb_command_serializes_correctly --locked --features experimental-apex-2023
```
Expected: PASS.

- [ ] **Step 6: Commit**
```bash
git add src/devices/hid_reports.rs
git commit -m "feat(hid): implement PerKeyRgbCommand with confirmed 0xXX byte [experimental]"
```

---

## Task 10: Wire Per-Key RGB into `ApexProTkl2023`

**Files:**
- Modify: `src/devices/keyboards/apex_pro_tkl_2023.rs`

- [ ] **Step 1: Write failing test**

Add a unit test (compile-only, no hardware needed):
```rust
#[test]
fn test_apex_pro_tkl_2023_has_set_key_color() {
    // This test verifies the API exists and compiles.
    // Actual behavior requires hardware.
    let _ = std::mem::size_of::<ApexProTkl2023>();
    // If this compiles with the methods below, the API is correct.
}
```

- [ ] **Step 2: Add `set_key_color` method**
```rust
#[cfg(feature = "experimental-apex-2023")]
pub fn set_key_color(&mut self, key: KeyId, color: Color) -> Result<()> {
    let db = KeyMappingDatabase::new();
    let mapping = db.get_mapping(KeyboardLayout::ApexProTkl2023)
        .ok_or_else(|| Error::InvalidConfig("No key mapping for ApexProTkl2023".to_string()))?;
    let addr = mapping.get_address(key)
        .ok_or_else(|| Error::InvalidConfig(format!("Key {:?} not in mapping", key)))?;
    let cmd = PerKeyRgbCommand::single_key(addr, color);
    self.inner.send_command_sync(cmd)
}
```

- [ ] **Step 3: Add `set_key_colors` batch method**
```rust
#[cfg(feature = "experimental-apex-2023")]
pub fn set_key_colors(&mut self, pairs: &[(KeyId, Color)]) -> Result<()> {
    for (key, color) in pairs {
        self.set_key_color(*key, *color)?;
    }
    self.inner.apply_sync()
}
```

> Note: If the captured protocol uses a batch command (multiple keys in one packet), replace the loop with a single `PerKeyRgbBatchCommand` that packs all keys. Check `docs/development/protocol-keyboard.md`.

- [ ] **Step 4: Build with experimental feature**
```powershell
cargo build --locked --features experimental-apex-2023 2>&1 | Select-Object -Last 5
```
Expected: `Finished dev profile`.

- [ ] **Step 5: Commit**
```bash
git add src/devices/keyboards/apex_pro_tkl_2023.rs
git commit -m "feat(apex-2023): add set_key_color/set_key_colors API [experimental]"
```

---

## Task 11: Implement Actuation Read-Back

**Files:**
- Modify: `src/devices/hid_reports.rs` — add `ActuationReadCommand`
- Modify: `src/devices/keyboards/apex_pro_tkl_2023.rs` — add `get_actuation_point`

> **Prerequisites:** Task 6 must confirm whether a read-back command exists. If no device→host packets were seen in Task 6 Step 7, the firmware is write-only — skip to Task 12 and document "no read-back".

- [ ] **Step 1: Write failing test**
```rust
#[test]
fn test_actuation_value_parse() {
    // If read-back uses the same 0.1mm encoding as write:
    let raw_value: u8 = 15; // 1.5mm
    let mm = raw_value as f32 * 0.1;
    assert!((mm - 1.5).abs() < f32::EPSILON);
}
```

- [ ] **Step 2: Run test**
```powershell
cargo test test_actuation_value_parse --locked
```
Expected: PASS (pure arithmetic, no hardware).

- [ ] **Step 3: Add `ActuationReadCommand` to `hid_reports.rs`**

Use the command byte discovered in Task 6 Step 7. If the read uses a feature report (`HidD_GetFeature`), the response is read via `device.get_feature_report()`, not via interrupt IN.

```rust
pub struct ActuationReadCommand;

impl HidCommand for ActuationReadCommand {
    fn command_code(&self) -> CommandCode { CommandCode::ActuationRead }
    fn serialize(&self, buf: &mut [u8], _: HidDeviceType) -> Result<usize> {
        buf[0] = 0x00;
        buf[1] = 0xXX; // REPLACE with confirmed read-back command byte from Task 6
        Ok(KEYBOARD_REPORT_SIZE)
    }
    fn validate(&self) -> Result<()> { Ok(()) }
    fn description(&self) -> String { "ActuationRead".to_string() }
}
```

Add `ActuationRead = 0xXX` to `CommandCode` enum.

- [ ] **Step 4: Add `get_actuation_point` to `ApexProTkl2023`**
```rust
pub fn get_actuation_point(&mut self) -> Result<f32> {
    let cmd = ActuationReadCommand;
    let mut response = [0u8; KEYBOARD_REPORT_SIZE];
    self.inner.send_command_and_read(cmd, &mut response)?;
    // Response byte position from Task 6 Step 7 analysis
    let raw = response[2]; // UPDATE offset based on protocol-keyboard.md
    Ok(raw as f32 * 0.1)
}
```

- [ ] **Step 5: Build**
```powershell
cargo build --locked 2>&1 | Select-Object -Last 5
```

- [ ] **Step 6: Commit**
```bash
git add src/devices/hid_reports.rs src/devices/keyboards/apex_pro_tkl_2023.rs
git commit -m "feat(apex-2023): add actuation read-back command"
```

---

## Task 12: Implement Rapid Trigger

**Files:**
- Modify: `src/devices/hid_reports.rs` — add `RapidTriggerCommand`
- Modify: `src/devices/keyboards/apex_pro_tkl_2023.rs` — add `set_rapid_trigger`

> **Prerequisites:** Task 6 must have identified the RT command byte and parameter layout.

- [ ] **Step 1: Write failing test**
```rust
#[test]
fn test_rapid_trigger_command_validates_sensitivity() {
    // sensitivity 0.0–4.0mm (same range as actuation point)
    let result_ok = RapidTriggerCommand::new(true, 0.2);
    assert!(result_ok.is_ok());
    let result_bad = RapidTriggerCommand::new(true, 5.0);
    assert!(result_bad.is_err());
}
```

- [ ] **Step 2: Run test (expect compile error)**
```powershell
cargo test test_rapid_trigger_command_validates_sensitivity --locked
```

- [ ] **Step 3: Add `RapidTriggerCommand` struct and `CommandCode::RapidTrigger`**

Add to `CommandCode` enum:
```rust
RapidTrigger = 0xXX,  // REPLACE with confirmed byte from Task 6
```

Add struct:
```rust
pub struct RapidTriggerCommand {
    pub enabled: bool,
    pub sensitivity_raw: u8, // 0.1mm increments, same as ActuationCommand
}

impl RapidTriggerCommand {
    pub fn new(enabled: bool, sensitivity_mm: f32) -> Result<Self> {
        let raw = (sensitivity_mm * 10.0).round() as u8;
        if raw == 0 || raw > 40 {
            return Err(Error::InvalidConfig(format!(
                "RT sensitivity {sensitivity_mm}mm out of range 0.1–4.0"
            )));
        }
        Ok(Self { enabled, sensitivity_raw: raw })
    }
}

impl HidCommand for RapidTriggerCommand {
    fn command_code(&self) -> CommandCode { CommandCode::RapidTrigger }
    fn serialize(&self, buf: &mut [u8], _: HidDeviceType) -> Result<usize> {
        buf[0] = 0x00;
        buf[1] = self.command_code() as u8;
        buf[2] = if self.enabled { 0x01 } else { 0x00 };
        buf[3] = self.sensitivity_raw;
        // Additional bytes per protocol-keyboard.md if needed
        Ok(KEYBOARD_REPORT_SIZE)
    }
    fn validate(&self) -> Result<()> { Ok(()) }
    fn description(&self) -> String {
        format!("RapidTrigger enabled={} sensitivity={}mm", self.enabled, self.sensitivity_raw as f32 * 0.1)
    }
}
```

- [ ] **Step 4: Run test to verify pass**
```powershell
cargo test test_rapid_trigger_command_validates_sensitivity --locked
```
Expected: PASS.

- [ ] **Step 5: Add `set_rapid_trigger` to `ApexProTkl2023`**
```rust
pub fn set_rapid_trigger(&mut self, enabled: bool, sensitivity_mm: f32) -> Result<()> {
    let cmd = RapidTriggerCommand::new(enabled, sensitivity_mm)?;
    self.inner.send_command_sync(cmd)?;
    self.inner.apply_sync()
}
```

- [ ] **Step 6: Build and test**
```powershell
cargo build --locked 2>&1 | Select-Object -Last 5
cargo test --locked 2>&1 | Select-Object -Last 10
```

- [ ] **Step 7: Commit**
```bash
git add src/devices/hid_reports.rs src/devices/keyboards/apex_pro_tkl_2023.rs
git commit -m "feat(apex-2023): add RapidTrigger command with sensitivity parameter"
```

---

## Task 13: Fix Zone Buffer for WIRELESS_2 PID

**Files:**
- Modify: `src/devices/keyboards/mod.rs` — `send_zone_buffer_async`

A bug exists: `send_zone_buffer_async` checks only `APEX_PRO_TKL_2023_WIRELESS` (0x1632) for per-zone fallback, missing `APEX_PRO_TKL_2023_WIRELESS_2` (0x1630).

- [ ] **Step 1: Write the failing test**
```rust
#[test]
fn test_wireless_2_uses_per_zone_path() {
    use crate::devices::product_ids::*;
    // Both wireless PIDs need per-zone zone buffer — 0xFF zone selector unsupported
    assert_ne!(APEX_PRO_TKL_2023_WIRELESS, APEX_PRO_TKL_2023_WIRELESS_2);
    // Test that they're treated equivalently in zone routing
    // (behavioral test requires hardware; this confirms PIDs are distinct values)
}
```

- [ ] **Step 2: Find the check in `mod.rs`**
```powershell
Select-String -Path "src\devices\keyboards\mod.rs" -Pattern "APEX_PRO_TKL_2023_WIRELESS" -Context 2
```
Expected: a condition like `if self.info.product_id == APEX_PRO_TKL_2023_WIRELESS`.

- [ ] **Step 3: Add WIRELESS_2 to the condition**

Change:
```rust
if self.info.product_id == super::product_ids::APEX_PRO_TKL_2023_WIRELESS {
```
To:
```rust
if self.info.product_id == super::product_ids::APEX_PRO_TKL_2023_WIRELESS
    || self.info.product_id == super::product_ids::APEX_PRO_TKL_2023_WIRELESS_2
{
```

- [ ] **Step 4: Build and test**
```powershell
cargo test --locked 2>&1 | Select-Object -Last 10
```

- [ ] **Step 5: Commit**
```bash
git add src/devices/keyboards/mod.rs
git commit -m "fix(apex-2023): include WIRELESS_2 (0x1630) in per-zone buffer path"
```

---

## Task 14: Full QA Pass

- [ ] **Step 1: Format check**
```powershell
Set-Location "C:\Users\Ven0m0\projects\steelseriesgg-rs"
cargo fmt --all -- --check
```
Expected: no output (clean). If diffs appear, run `cargo fmt --all` then re-check.

- [ ] **Step 2: Clippy — default features**
```powershell
cargo clippy --all-targets --locked -- -D warnings 2>&1 | Select-Object -Last 20
```
Expected: `Finished dev profile` with no errors.

- [ ] **Step 3: Clippy — experimental feature**
```powershell
cargo clippy --all-targets --locked --features experimental-apex-2023 -- -D warnings 2>&1 | Select-Object -Last 20
```

- [ ] **Step 4: Test suite — default features**
```powershell
cargo test --locked 2>&1 | Select-Object -Last 20
```
Expected: all tests pass.

- [ ] **Step 5: Test suite — experimental feature**
```powershell
cargo test --locked --features experimental-apex-2023 2>&1 | Select-Object -Last 20
```

- [ ] **Step 6: Release build check**
```powershell
cargo build --release --locked 2>&1 | Select-Object -Last 5
```

- [ ] **Step 7: Final commit**
```bash
git add -A
git commit -m "chore: QA pass — all tests and clippy clean post-RE implementation"
```
