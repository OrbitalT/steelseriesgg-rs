# Experimental Protocol Rules

Protocol work has three states. Never mix them up or upgrade a status without hardware evidence.

## Status table

| Status | Meaning | Doc label |
|--------|---------|-----------|
| Confirmed | Tested on real hardware | (none) |
| Experimental | Reverse-engineered, untested | `[EXPERIMENTAL]` |
| Placeholder | Code exists, actual command unknown | `[PLACEHOLDER]` |

## Current command code status

| Constant | Code | Status |
|----------|------|--------|
| `CommandCode::PerKeyRgb` | `0x23` | Placeholder — actual per-key command unknown |
| `CommandCode::Apex2023Direct` | `0x40` | Experimental — unverified on hardware |
| `CommandCode::ActuationControl` | `0x2D` | Experimental — firmware command unknown |

## Rules

**Adding new protocol code:** Label it `[EXPERIMENTAL]` or `[PLACEHOLDER]` — silence implies confirmed.

**Promoting experimental to confirmed:** Only after testing on physical hardware. Include device model and date in the commit. Remove the label.

**Never:** State experimental protocol as production-ready in docs or user-facing output. Gate experimental paths behind the relevant feature flag.
