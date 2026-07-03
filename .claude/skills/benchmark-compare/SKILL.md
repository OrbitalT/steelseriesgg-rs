---
name: benchmark-compare
description: Run steelseriesgg-rs's benchmark binaries before and after a change and report the timing delta.
user-invocable: true
disable-model-invocation: true
---

Compare performance before/after a change using the three benchmark binaries in `src/bin/`: `benchmark_rgb_alloc`, `benchmark_fragment`, `benchmark_validation_io`. They print plain timing text to stdout — there is no JSON output or stored baseline, so "before" must be captured from a clean checkout or stash.

## Steps

1. Identify which benchmark(s) are relevant to the change (e.g. touching `src/rgb/` → `benchmark_rgb_alloc`; touching HID report fragmentation → `benchmark_fragment`; touching `src/validation.rs` → `benchmark_validation_io`). Run all three if unsure or if the change is broad.
2. Capture the "before" baseline:
   - If the change isn't committed yet: `git stash`, run the benchmark(s), save output, `git stash pop`.
   - If comparing against a specific commit/branch: check it out in a worktree rather than switching the working tree out from under uncommitted work.
3. Build and run in release mode — debug-mode timings are not representative:
   ```
   cargo run --release --bin benchmark_rgb_alloc
   cargo run --release --bin benchmark_fragment
   cargo run --release --bin benchmark_validation_io
   ```
4. Capture "after" output the same way, on the current working tree.
5. Diff the two runs and report the delta as a percentage change per benchmark, not just raw numbers — call out any regression greater than ~5-10% explicitly.

## Notes

- These binaries use `Instant::now()` wall-clock timing with no warmup/statistical repeats beyond their own internal iteration loops — treat single-run deltas as noisy. Re-run at least twice on each side if the result is close to a decision threshold.
- `benchmark_validation_io` calls `MemorySample::new().await` in a loop and can fail mid-run (it reports the iteration count reached before aborting) — a lower iteration count on "after" is itself a regression signal, not just a missing number.
- Don't add a new benchmark binary or change existing ones to "make comparison easier" unless the user asks — this skill works with what exists.
