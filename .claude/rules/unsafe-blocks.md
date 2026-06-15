# Unsafe Block Rules

Every `unsafe` block must carry an inline comment that answers: **why is this block sound?**

## Required format

```rust
// SAFETY: <one sentence naming the invariant that makes this sound>
unsafe { ... }
```

The sentence must be concrete — not "the caller ensures this" but the specific invariant:

```rust
// SAFETY: `ptr` is non-null and points to a `u8` slice of at least `len` bytes,
//         allocated by hidapi and valid for the duration of this call.
let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
```

## What makes a block sound

1. All accessed memory is valid, aligned, initialized, and not aliased
2. All FFI call preconditions (null checks, lifetimes, alignment) are met at the call site
3. No simultaneous `&T` and `&mut T` aliasing

## Review checklist

- Every `unsafe` block has a `// SAFETY:` comment
- The comment names a concrete invariant, not a vague promise
- The invariant is actually enforced at the call site
- No `unsafe` block was added to bypass a safe abstraction that already exists
