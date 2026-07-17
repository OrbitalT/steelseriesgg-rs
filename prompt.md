# Role & Context
You are an expert systems engineer specializing in Rust, Linux hardware enablement (CachyOS), and USB HID protocols. You are implementing native, GG-free support for the SteelSeries Apex Pro keyboard and Arena 7 desktop speakers inside the `steelseriesgg-rs` daemon architecture.

Your objective is to map decoded USB packet trace structures into the daemon pipelines (`src/daemon.rs`) and target device drivers (`src/devices/arena_7.rs` and `src/devices/apex_pro.rs`).

Ensure all implementations are strictly structured, highly optimized for low-latency operations, and comply with the following technical specifications.

---

### Task 1: Interface Discovery & Driver Registration
1. **Device Enumeration Configuration**:
   - [cite_start]Register the SteelSeries Vendor ID (`0x1038`) and the specific Product IDs (PIDs) for the Apex Pro and Arena 7 within the registration config pipelines[cite: 17].
   - [cite_start]**Crucial Multi-Interface Matching**: Modify the HID matching architecture using the `hidapi` crate[cite: 39]. [cite_start]Do not match on VID/PID alone[cite: 38]. 
   - [cite_start]Parse and match the `interface_number` during enumeration to direct traffic cleanly[cite: 39]:
     * [cite_start]**Interface 0 (3.4.0)**: Route to the Apex Pro LED configuration and matrix streaming pipeline[cite: 30].
     * [cite_start]**Interface 1 (3.4.1)**: Target for continuous HID interrupt input reports (URB_INTERRUPT in, 8-byte payload) to monitor key states/keystrokes[cite: 31].

---

### Task 2: Implementing the Arena 7 Speaker Driver (`src/devices/arena_7.rs`)
[cite_start]Implement the following methods inside the Arena 7 driver module using Class-Interface Control Transfers (`Request Type: 0x21`, `SET_REPORT: 0x09`, `Report ID: 0x06`)[cite: 8]:

1. [cite_start]**Static Single-Color Mode (`subtype a1`)**[cite: 9]:
   - [cite_start]**Header**: `06 a1`[cite: 10].
   - [cite_start]**Zone Layout Map**: 4 physical zones ordered Left Top (LT), Left Bottom (LB), Right Top (RT), and Right Bottom (RB)[cite: 10].
   - [cite_start]**Zone Buffer Structure**: Format each zone as exactly 7 bytes of payload[cite: 11]:
     `[Red, Green, Blue, Effect (0x01), Constant (0x1e), Brightness (0x01 to 0x0a)]`[cite: 11].
   - **Footer**: Write a terminal trailing byte of `0f` at byte array index 30 to complete the raw block write[cite: 11].

2. [cite_start]**Dynamic Prism ColorShift Streaming Mode (`subtype a7`)**[cite: 12]:
   - [cite_start]**Header**: `06 a7 0f`[cite: 13, 93].
   - [cite_start]**Zone Payload Structure**: Consists of raw 3-byte RGB arrays directly back-to-back mapping across the 4 physical zones[cite: 13]:
     [cite_start]`[R1, G1, B1, R2, G2, B2, R3, G3, B3, R4, G4, B4, 00, 00, ...]`[cite: 13].
   - [cite_start]**Buffer Requirements**: Ensure the command wraps inside a 100-byte packet containing a 71-byte HID buffer payload under Report ID `0x06`[cite: 87]. [cite_start]Pad the rest of the array with zeroed trailing bytes to hit target structures[cite: 13].

3. [cite_start]**Equalizer Control Engine (`subtype 33`)**[cite: 14]:
   - [cite_start]**Header**: `06 33`[cite: 14].
   - [cite_start]**Frequency Targets**: Expect 10 sequential bytes targeting hardware bands (from 32Hz to 16kHz)[cite: 14].
   - [cite_start]**Step Scaling Logic**: Convert human-readable decibels (dB) using a 0.5 dB step per integer unit, mapping a `0x14` hex value (20 decimal) as the median zero-point base (0 dB)[cite: 15]:
     * [cite_start]`0x00` = -10 dB [cite: 15]
     * [cite_start]`0x14` = 0 dB [cite: 15]
     * [cite_start]`0x28` = +10 dB [cite: 15]

---

### Task 3: Implementing the Apex Pro Keyboard Driver (`src/devices/apex_pro.rs`)
[cite_start]Implement configuration registers and canvas writing targets utilizing the control interfaces[cite: 69]:

1. [cite_start]**650-Byte Dynamic RGB Canvas Matrix**[cite: 32]:
   - [cite_start]**Streaming Initialization**: When real-time matrix animation updates are triggered, prefix the canvas command sequence with the re-entry handshake bytes: `09 00 03 01 00 82 02` to prep the controller[cite: 97].
   - [cite_start]**Stream Matrix Buffer**: Format a driver write method that transforms the daemon's internal key grid states into a 650-byte payload representing raw sequential per-key RGB coordinates across the key matrix[cite: 33].
   - [cite_start]**Packet Frame Structure**: Transmit the total frame using 678-byte packets (containing a 649-byte USB control payload) to Interface 0 (`3.4.0`)[cite: 85, 97].

2. [cite_start]**Onboard Key Actuation Config (`subtype 2b`)**[cite: 70]:
   - [cite_start]**Header**: `06 2b`[cite: 70].
   - [cite_start]**Layout Mapping**: Accepts key switch indices alongside raw physical depth settings[cite: 71, 72].
   - [cite_start]**Actuation Boundaries**: Map standard actuation heights from 0.1mm up to 4.0mm directly to their hex range equivalents[cite: 21, 72, 73]:
     * [cite_start]`0x01` represents 0.1mm [cite: 72]
     * [cite_start]`0x28` represents 4.0mm (40 raw increments for OmniPoint hardware)[cite: 21, 73].

3. [cite_start]**Dynamic Rapid Trigger Logic (`subtypes 2b & 2c`)**[cite: 81]:
   - Write properties handling dynamic trigger threshold maps. [cite_start]Set dual travel thresholds ranging from `0x01` to `0x28` to track immediate change actions[cite: 76, 77]:
     * [cite_start]**Activation Sensitivity**: `0x01` to `0x28`[cite: 76].
     * [cite_start]**Deactivation Sensitivity**: `0x01` to `0x28`[cite: 77].
     * [cite_start]**Dynamic Tracking Flags**: Configure state flags (`0x01` for standard static key actuation, `0x03` to toggle active dynamic top/bottom Rapid Trigger tracking)[cite: 78].

---

### Task 4: Daemon Synchronization and Loop Integration (`src/daemon.rs`)
Implement the real-time background scheduling loop to power localized "Prism" synchronization:

1. **Interleaved Multi-Device RGB Streaming Loop**:
   - [cite_start]Establish a global daemon-tick loop running on a shared **60ms to 65ms heartbeat window**[cite: 36].
   - [cite_start]Synchronize outputs using low-latency, interleaved writes to prevent multi-device timing drift[cite: 99]:
     * [cite_start]First, serialize and write the **72-byte SET_REPORT** (with the `06 a7 0f` payload subtype) directly to the Arena 7 speaker interface (`3.1.0`)[cite: 34, 93].
     * Within **1ms to 5ms immediately following**, serialize and stream the corresponding **650-byte lighting matrix payload** to the Apex Pro keyboard interface (`3.4.0`)[cite: 35].
   - Implement array slice memory updates inside your animation canvas structures for performant frame generation[cite: 45].
   - Ensure a clean break routine when animations cease: drop the active dynamic streaming loops and send uniform off/reset commands to zero out active device matrices[cite: 89, 90, 91].

---

### Verification and Quality Standards
1. Integrate CLI debug checks utilizing `--experimental-apex-2023` flags and leverage `discover_actuation` utilities to verify magnetic OmniPoint switch states directly from system devices[cite: 20, 23].
2. Prior to submitting code, verify alignment with repository workspace requirements: format all additions using `cargo fmt`, execute standard workspace test suites, and compile using optimization tags[cite: 24, 42].
