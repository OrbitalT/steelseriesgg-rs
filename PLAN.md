---
goal: Execution-ready implementation plan for the open backlog (issues + TODO).
date_created: 2026-03-17
last_updated: 2026-06-15
last_reviewed: 2026-06-15
status: In Progress
sources: GitHub issues #173 · TODO.md · OpenRGB SteelSeriesApex8ZoneController · prior PLAN.md
---

# PLAN: implementation priorities

Converts the open GitHub issues (excluding the Renovate Dependency Dashboard #126) and the
active TODO backlog into one prioritized, execution-ready order for a future agent session.

---

## 1. Overview

The only open code-bug issue is **#173 (Apex 3 TKL RGB not applied)** — now the top priority
because a concrete protocol reference exists (OpenRGB's 8-zone controller). The remaining work
is reverse-engineering and accuracy/maintainability cleanup carried over from `TODO.md`.

Device-registration issues #211 (Arctis Nova Pro Omni `0x2290`) and #165 (Apex Pro TKL Wireless
2023 `0x1630`) are **implemented in code** (`src/devices/mod.rs`, `discovery.rs`, with tests).
They remain open on GitHub only because they have not been closed; verify on hardware / close
them out separately. Issue **#6 (Arch compile)** was **closed `not_planned` on 2026-05-23** and
is dropped from this plan.

---

## 2. Constraints to preserve

- Read `CLAUDE.md` (same content as `AGENTS.md`) before making code changes.
- Source-of-truth files when prose drifts:
  `Cargo.toml`, `rust-toolchain.toml`, `.github/workflows/ci.yml`, `src/devices/hid_reports.rs`.
- Keep `hidapi = "=2.6.6"` pinned unless the task explicitly requires changing it.
- Do not loosen the localhost-only GameSense CORS policy.
- Keep `audio` and `sonar` feature gating independent.
- Always build HID reports with `HidReportBuilder` / typed helpers — never hand-rolled byte arrays.
- No `.unwrap()` / `.expect()` in production paths.
- Do not present placeholder or unverified protocol support as confirmed hardware behavior.
- Minimal scope: fix what the task asks, no opportunistic refactors.

---

## 3. Status summary

### Completed

| Item | Notes |
|------|-------|
| Issue #120 — audio hang | 5 s timeout in `src/audio/pulse.rs` |
| `experimental-apex-2023` feature flag | present in `Cargo.toml` |
| CLI expansion (HidLogs, TestDevice, VerifyPerformance, Fuzz) | subcommands in `src/main.rs` |
| Issue #211 — Arctis Nova Pro Omni PID `0x2290` | `ARCTIS_NOVA_PRO_OMNI` const + match arms + test (`src/devices/mod.rs:453`); GitHub issue still open — close after hardware confirmation |
| Issue #165 — Apex Pro TKL Wireless 2023 PID `0x1630` | `APEX_PRO_TKL_2023_WIRELESS_2` registered in `mod.rs` + `discovery.rs` + tests; GitHub issue still open — close after hardware confirmation |
| Issue #6 — Arch compile | Closed `not_planned` 2026-05-23 — no action |

### Open — ranked

| # | Item | Priority | Phase |
|---|------|----------|-------|
| #173 | Apex 3 TKL — commands succeed but RGB does nothing | High | 1 |
| TODO | Apex Pro TKL 2023 key matrix RE + actuation read-back | Medium | 2 |
| TODO | Protocol docs reconciliation | Low | 3 |
| TODO | Apex Pro capability accuracy cleanup | Low | 4 |
| — | `src/main.rs` refactor | Low | 5 |

---

## 4. Phase 1 — Apex 3 TKL RGB not applied (Issue #173) · High

### Context

`ssgg rgb solid #ff0000` on an Apex 3 TKL (PID `0x1622`) reports success but nothing changes;
multiple users confirm (`#173`), and runtime logs `WARN No key mapping available for product ID
0x1622 - per-key RGB disabled`. The Apex 3 TKL is a **zone** keyboard (8–10 zones), not per-key,
so the missing key map is expected — the real bug is the zone-color command path.

`Apex3Tkl` (`src/devices/keyboards/apex.rs`) delegates `set_color` / `set_zone_colors` to
`GenericKeyboard`, and its own `CMD_RGB_EFFECT = 0x23` (line 24) was written speculatively.

### Concrete lead (OpenRGB 8-zone protocol)

Reference: OpenRGB `SteelSeriesApex8ZoneController.h` (linked by a user in #173, owner acked
2026-06-15). 65-byte report (report ID byte + 64 payload):

| Command | Byte 0 | Layout |
|---------|--------|--------|
| Set zone colors | `0x21` | byte 1 = LED bitmask (`0xFF` = all 8), bytes 2–25 = `R G B` × 8 zones, rest zero-padded |
| Rainbow wave | `0x22` | byte 1 = `0xFF` |
| Brightness | `0x23` | byte 1 = `0x00`–`0x10` (multiplier, persists across Mod+F11/F12) |

**Likely root cause:** the current code uses `0x23` (brightness in this dialect) to push color, so
the firmware applies a brightness write and ignores the intended color. The correct set-color
command is **`0x21`** with the per-zone RGB layout above.

### Primary files

- `src/devices/keyboards/apex.rs` — `Apex3Tkl` (`CMD_RGB_EFFECT`, color delegation lines 118–146)
- `src/devices/keyboards/mod.rs` — `GenericKeyboard::set_color` / `set_zone_colors`
- `src/devices/hid_reports.rs` — add/confirm the `0x21` zone-color command code via `HidReportBuilder`
- `src/devices/zone_mapping.rs:311` — `APEX_3_TKL` zone mapping (currently registered)

### Tasks

1. Add a typed command code for the 8-zone set-color (`0x21`) to `hid_reports.rs` if absent;
   build the report with `HidReportBuilder` (no raw arrays).
2. Override `set_color` and `set_zone_colors` in `Apex3Tkl` instead of delegating to the Apex Pro
   path: map the requested color(s) onto bytes 2–25 with bitmask `0xFF`.
3. Map `set_color(c)` → all 8 zones = `c`; map `set_zone_colors` onto the zone order in
   `zone_mapping.rs`. Reconcile the zone count (see Open Questions — code says 9, OpenRGB says 8).
4. Confirm whether a separate `0x23` brightness write is needed before/after color for the LEDs
   to be visible at non-zero brightness.
5. Mark anything still unverified with an explicit `experimental` doc comment.

### Validation

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
# On hardware (Apex 3 TKL, PID 0x1622):
ssgg rgb solid "#ff0000"   # red should appear
ssgg rgb solid "#000000"   # off
```

### Success criteria

- `ssgg rgb solid` visibly changes the keyboard on real hardware.
- `GenericKeyboard` behavior for other keyboards is unchanged.
- `cargo test --locked` passes.

### Complexity

⭐⭐ Medium — protocol now documented; small code change, but **final confirmation needs the
Apex 3 TKL hardware** (the OpenRGB layout is a strong lead, not a guarantee for this SKU).

---

## 5. Phase 2 — Apex Pro TKL 2023 key matrix RE + actuation read-back (TODO) · Medium

### Context

All 87 `KeyId → KeyAddress` mappings for `ApexProTkl2023` in `src/devices/key_mapping.rs` are
placeholder data; runtime falls back to `simulate_per_key_with_zones()`. Real per-key RGB and an
actuation read-back command remain unverified. ANSI/ISO layout and unsupported-key handling also
need hardware validation. This is the "Continue Apex Pro TKL 2023 protocol and RGB work" backlog
item.

### Research methods (in order)

1. Binary RE of `SteelSeriesGG107.exe` for key-address lookup tables.
2. USB capture (Wireshark/USBPcap) while SteelSeries GG sets per-key colors; decode byte offsets.
3. Community sources: `AstroSnail/apexctl`, `FrankGrimm/apex7tkl_linux`, `not-jan/apex-tux`.

### Primary files

- `src/devices/key_mapping.rs` — `KeyMappingDatabase::new()` for `ApexProTkl2023`
- `src/devices/keyboards/apex_pro_tkl_2023.rs`
- `src/bin/discover_actuation.rs` — probe actuation firmware commands
- `src/bin/verify_key_mapping.rs` — validate mappings on hardware
- `docs/development/KEY_MAPPING_RESEARCH.md` — per-key status

### Tasks

1. Extract verified HID codes / matrix addresses for the keys (at least alphanumeric + modifiers).
2. Replace placeholder `KeyAddress::new(...)` calls with verified values.
3. Replace the speculative `0x23` per-key path once the real command is confirmed; keep
   `CommandCode::PerKeyRgb (0x23)` and `Apex2023Direct (0x40)` labeled experimental until then.
4. Use `discover_actuation.rs` to find an actuation read-back command if firmware exposes one.
5. Validate ANSI/ISO differences and unsupported-key handling on hardware.
6. Update `KEY_MAPPING_RESEARCH.md` to reflect verified vs. still-placeholder keys.

### Validation

```bash
cargo build  --locked --features experimental-apex-2023
cargo test   --locked --features experimental-apex-2023
```

### Complexity

⭐⭐⭐ High — gated on RE/capture; code change is small once addresses are known.

---

## 6. Phase 3 — Protocol docs reconciliation (TODO) · Low

### Objective

Make `docs/development/` safe to follow: separate confirmed behavior, placeholder code, and
speculation; remove dead references.

### Primary files

- `docs/development/APEX_PRO_PROTOCOL.md`, `PROTOCOL_RESEARCH.md`, `KEY_MAPPING_RESEARCH.md`

### Tasks

1. Confirm every helper-binary reference still resolves (`discover_actuation`, `verify_key_mapping`,
   `benchmark_rgb_alloc`, `benchmark_fragment`, `sonar_control`); fix any dangling ones.
2. Add `> ⚠️ UNVERIFIED` callouts wherever a command code or address is a placeholder.
3. Cross-link the OpenRGB 8-zone reference (Phase 1) into the Apex 3 TKL notes.

### Success criteria

No dev doc points contributors at a nonexistent tool; confirmed vs. speculative is clearly marked.

### Complexity

⭐ Low — docs only.

---

## 7. Phase 4 — Apex Pro capability accuracy cleanup (TODO) · Low

### Objective

Ensure placeholder per-key support is never presented as verified. Accuracy/maintainability only —
no runtime behavior change.

### Primary files

- `src/devices/hid_reports.rs`, `src/devices/key_mapping.rs`,
  `src/devices/keyboards/mod.rs`, `src/devices/keyboards/apex_pro_tkl_2023.rs`

### Tasks

1. Find every place placeholder command codes (`PerKeyRgb (0x23)`, `Apex2023Direct (0x40)`,
   `ActuationControl (0x2D)`) are described/logged without an `⚠️ experimental` qualifier.
2. Add qualifiers in doc comments, log messages, or CLI help text.
3. Do not change runtime behavior.

### Validation

```bash
cargo test --locked
cargo test --locked --features experimental-apex-2023
```

### Complexity

⭐ Low — comments/strings only.

---

## 8. Phase 5 — `src/main.rs` refactor (deferred) · Low

### Objective

Improve maintainability of the CLI dispatch layer without behavior change.

### Tasks

1. Extract low-churn command families into focused modules.
2. Preserve clap behavior, output, and feature gating exactly.
3. Start only after Phases 1–4 are stable.

### Validation

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

### Complexity

⭐⭐⭐ High — large surface; lowest priority.

---

## 9. Dependencies

```
Phase 1 (Apex 3 TKL RGB)  ── protocol known; needs hardware to confirm ──┐
Phase 2 (Key matrix RE)   ── needs RE/USB capture session ───────────────┤
Phase 3 (Docs reconcile)  ── unblocked ──────────────────────────────────┤──► Phase 5 (main.rs)
Phase 4 (Capability acc.) ── unblocked ──────────────────────────────────┘
```

| Phase | Blocked by | Reason |
|-------|------------|--------|
| 1 | Apex 3 TKL hardware (confirmation only) | OpenRGB layout is a lead, not SKU-confirmed |
| 2 | RE / USB capture | Key addresses + actuation command unknown |
| 3, 4 | Nothing | Documentation / accuracy only |
| 5 | 1–4 stable | Refactor risk |

---

## 10. Open questions / blockers

1. **Apex 3 TKL zone count:** code registers **9** zones (`mod.rs:581`, `zone_mapping.rs:311`)
   but OpenRGB models it as **8**. Confirm the true count before mapping `set_zone_colors`.
2. **Report ID / framing:** confirm whether this codebase's `HidReportBuilder` expects a leading
   `0x00` report-ID byte for the `0x21` write, matching the 65-byte (1+64) OpenRGB report.
3. **Brightness coupling:** does color show only after a `0x23` brightness write (non-zero), or is
   `0x21` self-sufficient? Determines whether Phase 1 must also send brightness.
4. **Hardware access:** Phases 1 and 2 both need the physical devices for final confirmation; the
   owner has the Apex Pro TKL 2023 and acked checking the OpenRGB reference for #173 (2026-06-15).
5. **Close-out:** #211 and #165 are implemented but still open on GitHub — confirm on hardware and
   close, or leave open pending user reports.

---

## 11. Deferred research (reference only)

- **Open-G-Hub** (`https://github.com/Sharper-Flow/Open-G-Hub`) — defer unless a concrete blocker
  suggests reusable logic.
- Apex protocol: `AstroSnail/apexctl`, `FrankGrimm/apex7tkl_linux`, `not-jan/apex-tux`;
  OpenRGB `SteelSeriesApex8ZoneController` (primary lead for Phase 1).
- Sonar/audio (when Sonar work resumes): `PrzemekkkYT/GGSonarRev`, `wex/sonar-rev`,
  `Mark7888/steelseries-sonar-py`, `codeberg.org/Birbwell/linuxmix`, `Dymstro/nova-chatmix-linux`.
- Research-only: `flozz/rivalcfg`, `llMBQll/OmniLED`.

---

## 12. Suggested next move

Start **Phase 1** — it is the only open code bug, multi-user confirmed, and now has a documented
protocol. Land the `0x21` zone-color path behind hardware confirmation; **Phases 3 & 4** are
low-effort, fully unblocked fillers if hardware is unavailable.
