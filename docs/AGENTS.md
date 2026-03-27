<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-27 | Updated: 2026-03-27 -->

# docs/

## Purpose

Documentation directory containing protocol research notes, reverse-engineering findings, and hardware analysis for SteelSeries device communication. Not compiled into the binary — reference material only.

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `development/` | Protocol research and reverse-engineering notes for HID communication (see `development/AGENTS.md`) |

## For AI Agents

### Working In This Directory

- These are reference documents — do NOT edit them to match code assumptions; update them when new protocol behavior is discovered or verified
- Cross-reference with `src/devices/hid_reports.rs` (source of truth for current HID report format) before relying on doc content
- Docs may lag behind implementation — always prefer live code over docs when they conflict

<!-- MANUAL: -->
