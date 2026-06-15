# TODOs

## Research

the [OpenRGB project](https://openrgb.org/) has [support for APEX 3 TKL](https://openrgb.org/devices.html?search=apex+3+tkl) and they do have an [overview of the protocol](https://gitlab.com/CalcProgrammer1/OpenRGB/-/blob/master/Controllers/SteelSeriesController/SteelSeriesApex8ZoneController/SteelSeriesApex8ZoneController.h?ref_type=heads#L45). Research that and implement it.

## Active backlog

1. Reproduce issue #6 on an Arch-like environment and fix it only if the failure still exists.
   - Validate the `cargo build --release --locked` path used by the package and release workflow.
   - Keep any fix limited to packaging, install metadata, manifests, or CI unless reproduction proves a code change is required.
2. Continue Apex Pro TKL 2023 protocol and RGB work.
   - Verify the real per-key RGB protocol on hardware and replace the placeholder `0x23` path when confirmed.
   - Discover an actuation read-back command if the firmware exposes one.
   - Validate unsupported-key handling plus ANSI/ISO layout differences on hardware.

## Deferred research

- Open-G-Hub (`https://github.com/Sharper-Flow/Open-G-Hub`)
  - Defer unless a concrete blocker suggests reusable logic for active backlog items.

## Reference projects by relevance

### Directly relevant

- Sonar / audio research
  - https://github.com/PrzemekkkYT/GGSonarRev
  - https://github.com/wex/sonar-rev
  - https://github.com/Mark7888/steelseries-sonar-py
  - https://codeberg.org/Birbwell/linuxmix
  - https://github.com/Dymstro/nova-chatmix-linux
- Apex keyboard protocol / RGB research
  - https://github.com/AstroSnail/apexctl
  - https://github.com/FrankGrimm/apex7tkl_linux
  - https://github.com/not-jan/apex-tux

### Research-only

- https://github.com/flozz/rivalcfg
- https://github.com/llMBQll/OmniLED

### Out of scope for the current backlog

- https://github.com/Gibtnix/Apex-Macros
- https://github.com/Gibtnix/MSIKLM
- https://github.com/stephenlacy/msi-keyboard
