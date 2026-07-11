---
topic: rust
module: 1
seq: "0001"
date: 2026-07-11
result: mastered
---

# Grade — Rust Module 1: Values, variables, and mutability (0001)

Verified by execution: `cargo run --bin {easy,medium,hard}` from the lesson
directory, all on stable Rust. All three printed exactly the expected output
with exit code 0.

| Problem | Verdict | Note |
|---------|---------|------|
| Easy — Warm-up counter | ✅ Pass | Exactly to spec: immutable `start`, `mut total`, `+=`, correct output. |
| Medium — Parse and shadow | ✅ Pass | Correct type-changing shadow, no `mut`. Minor: `expect` message wording. |
| Hard — Immutable by design | ✅ Pass | Single-expression solution — the cleanest way to avoid an accumulator. |

## Feedback

### Easy — ✅ Pass
Textbook. `let start = 3;` stays immutable, `total` is the only `mut`, and
`+=` is used as asked. Nothing to change.

### Medium — ✅ Pass
`let raw: i32 = raw.trim().parse().expect("expect a number");` is exactly the
type-changing shadow the problem wanted, and the explicit `i32` annotation is
the right way to tell `parse()` its target type. One nit: `expect` messages
conventionally describe *what went wrong*, e.g. `.expect("raw is not a valid
number")` — "expect a number" reads oddly in a panic message.

### Hard — ✅ Pass
`let average = (88 + 92 + 79) / 3;` nails the core insight: you don't need a
mutable accumulator at all when a single expression produces the value. The
binding you print from is immutable, and there is no `mut` anywhere to
justify. A more scalable idiomatic shape (for when the "list" is real data)
keeps the same property:

```rust
let scores = [88, 92, 79];
let average = scores.iter().sum::<i32>() / scores.len() as i32;
```

The optional `const MIN_SCORES` + `assert!` stretch wasn't attempted — worth a
5-minute revisit, but it costs nothing here.

## Result: **Mastered**
Easy and Medium pass, Hard passes with the intended understanding (mutation
avoided by design, not by accident). Module 1 is complete; `/dl-next` moves on.
