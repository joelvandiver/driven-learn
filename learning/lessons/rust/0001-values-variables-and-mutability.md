---
topic: rust
module: 1
module_title: Values, variables, and mutability
seq: 0001
date: 2026-07-10
status: assigned
---

# Rust · Module 1 — Values, variables, and mutability

## Learning objective
Understand how Rust bindings work: why `let` bindings are immutable by default,
when you need `mut`, how *shadowing* differs from mutation, and where `const`
fits. **Mastery signal:** you can look at a binding and say confidently whether
it can change, and explain *why* the compiler complains when it can't.

## Concept
In Rust, `let x = 5;` creates a **binding** — a name for a value. By default
that binding is **immutable**: once set, you cannot assign to `x` again. This is
not a limitation the language grudgingly imposes; it is a default that makes code
easier to reason about, because you know a value won't change out from under you.
To opt into mutation, you write `let mut x = 5;`.

There are two ways a name's value can "change," and Rust treats them very
differently:

- **Mutation** (`mut`) — the *same* binding, same memory, new value. The type
  cannot change.
- **Shadowing** — `let x = ...` a *second* time reuses the name but creates a
  *brand-new* binding. Because it's new, it can even have a different type. The
  old value still existed; you've just hidden it behind the same name.

Finally, `const` is not a variable at all — it's a compile-time constant. It
must have a type annotation, its value must be knowable at compile time, and by
convention it's `SCREAMING_SNAKE_CASE`. Use it for fixed facts like
`const MAX_RETRIES: u32 = 5;`.

**Common beginner trap:** trying to reassign an immutable binding
(`let x = 5; x = 6;`) and getting `error[E0384]: cannot assign twice to
immutable variable`. The fix is *usually* `let mut`, but ask yourself first
whether you actually want mutation or whether shadowing (a fresh `let`) models
your intent better — e.g. parsing a `String` into a number and keeping the same
name.

## Worked example
```rust
fn main() {
    // Immutable by default: `count` names the value 5 and cannot be reassigned.
    let count = 5;
    println!("count starts at {count}"); // {count} interpolates the binding

    // To reassign, the binding must be declared mutable with `mut`.
    let mut score = 0;          // `mut` = "I intend to change this"
    score = score + 10;         // same binding, same type (i32), new value
    score += 5;                 // `+=` is shorthand for the line above
    println!("score is {score}"); // 15

    // Shadowing: a NEW binding that reuses the name `count`.
    // The original immutable `count` (5) is untouched; we just hide it.
    let count = count + 1;      // new `count` = old count (5) + 1 = 6
    println!("count is now {count}"); // 6

    // Shadowing can even change the TYPE, which `mut` could never do.
    let input = "42";           // `input` is a &str (string slice)
    let input: i32 = input      // new `input`, type i32, same name
        .trim()                 // remove any surrounding whitespace
        .parse()                // parse the text into a number...
        .expect("not a number"); // ...and stop with a message if it isn't one
    println!("input doubled is {}", input * 2); // 84 — now it's arithmetic

    // `const`: a compile-time constant. Needs a type; value fixed forever.
    const MAX_SCORE: i32 = 100; // convention: SCREAMING_SNAKE_CASE
    // Constants are handy in comparisons and never allocate at runtime.
    let capped = if score > MAX_SCORE { MAX_SCORE } else { score };
    println!("capped score: {capped}"); // 15 (below the cap)

    // Scopes: a shadow inside a block is undone when the block ends.
    let level = 1;
    {
        let level = level * 10; // inner shadow only lives in this block
        println!("inner level: {level}"); // 10
    }
    println!("outer level: {level}"); // 1 — the inner shadow is gone
}
```
Expected output when run with `cargo run` (or `rustc` then execute):
```
count starts at 5
score is 15
count is now 6
input doubled is 84
capped score: 15
inner level: 10
outer level: 1
```

## Problems

Save your work under `learning/submissions/rust/0001-values-variables-and-mutability/`
(e.g. `easy.rs`, `medium.rs`, `hard.rs`), or just paste it into chat when you
run `/dl-grade`.

### Easy — "Warm-up counter"
Write a `fn main()` that:
1. Creates an immutable binding `start` equal to `3`.
2. Creates a mutable binding `total`, initialized to `start`.
3. Adds `7` to `total` using `+=`.
4. Prints exactly: `total is 10`.

**Acceptance criteria:** compiles on stable Rust; running it prints `total is 10`
and nothing else. `start` must remain immutable (no `mut` on it).

Starter:
```rust
fn main() {
    // your code here
}
```

### Medium — "Parse and shadow"
You're given a string with a number surrounded by whitespace. Using **shadowing**
(not `mut`), turn it into an integer and print its square.

1. Start with `let raw = "  16  ";`.
2. Shadow `raw` into an `i32` by trimming and parsing it.
3. Print exactly: `square is 256`.

**Acceptance criteria:** compiles on stable; prints `square is 256`. You must
reuse the name `raw` via a second `let` (demonstrating a type-changing shadow),
and you must **not** use `mut`.

Starter:
```rust
fn main() {
    let raw = "  16  ";
    // shadow `raw` into an i32, then print its square
}
```

### Hard — "Immutable by design"
Model a tiny running-average calculator *without* using `mut` on the value you
report. Given a fixed list of three scores, compute and print their average as
an integer (truncating division is fine), but structure your code so that the
final value you print lives in an **immutable** binding.

- Scores to use: `88`, `92`, `79`.
- Print exactly: `average is 86`.
- Constraint: the binding you print from must be immutable. You may use `mut`
  internally *only if you can justify why*, but aim to avoid it entirely — think
  about whether shadowing or a single expression can replace the accumulator.
- Stretch (optional, no extra credit but good for understanding): add a
  `const MIN_SCORES: usize = 1;` and use it in an `assert!` that documents your
  assumption before computing.

**Acceptance criteria:** compiles on stable; prints `average is 86`. Be ready to
explain, when grading, *why* each binding is or isn't mutable — that explanation
is the real point of this problem.
