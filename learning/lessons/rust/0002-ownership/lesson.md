---
topic: rust
module: 2
module_title: Ownership
seq: 0002
date: 2026-08-16
status: assigned
---

# Rust · Module 2 — Ownership

## Learning objective
Understand Rust's ownership model: when assignment **moves** a value versus
when it **copies** one, and when a value is **dropped** as its scope ends.
**Mastery signal:** you can look at a line of code and predict whether it
invalidates an earlier binding — and explain why the compiler accepts or
rejects the next line that tries to use it.

## Concept
Every value in Rust has exactly one **owner** — the binding responsible for
cleaning it up. When that owner's scope ends, Rust automatically calls `drop`
on the value and frees its resources. No garbage collector, no manual `free`:
the compiler works this out at compile time by tracking ownership through
your code.

The part that surprises newcomers coming from other languages is what
`let b = a;` actually does. It depends entirely on the type of `a`:

- **Move** — for types that own a heap allocation (`String`, `Vec<T>`, and
  most custom structs), assignment transfers ownership. `a` is no longer
  valid after this line; only `b` is. This isn't a copy of the data — it's
  the *same* data with a new, single owner. The old binding is deliberately
  left unusable so there's never a moment where two owners could both try to
  free (or mutate) the same memory.
- **Copy** — for small, stack-only types (`i32` and the other integer/float
  types, `bool`, `char`, and tuples of `Copy` types), assignment duplicates
  the bits instead. Both `a` and `b` remain valid, because there's no shared
  resource to worry about — they're independent copies from the start.

The same rule applies when you pass a value into a function or return one out
of it: passing a `String` *moves* it in (the caller's binding becomes
invalid), and returning a `String` *moves* it back out (the caller gets a
fresh, valid owner). Passing an `i32` just copies it — the caller's binding
is untouched.

**Common beginner trap:** writing `let s2 = s1;` (or passing `s1` into a
function) and then trying to use `s1` again below, expecting it to behave
like a copy. The compiler stops you with `error[E0382]: borrow of moved
value`. The fix is not usually to add `mut` — it's to either restructure the
code so only one binding needs the value at a time, or (in a later module)
borrow it with `&` instead of moving it. For now, the skill this module
builds is *predicting* the move before the compiler tells you about it.

## Worked example
Type this code yourself into `src/bin/example.rs` (scaffolded next to this
file as an empty `fn main()`), then run it from this lesson's directory with
`./run example` — your output should match the expected output below.
```rust
struct Droppable {
    name: String,
}

// Implementing Drop lets us observe exactly when a value's scope ends.
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("dropping {}", self.name);
    }
}

fn takes_ownership(s: String) {
    println!("took ownership of {s}");
} // `s` is dropped right here, at the end of this function

fn makes_copy(n: i32) {
    println!("received a copy: {n}");
}

fn main() {
    // Move semantics: String owns heap data, so assignment MOVES ownership.
    let s1 = String::from("hello");
    let s2 = s1; // s1's ownership moves to s2; s1 is no longer valid
    // println!("{s1}"); // would fail to compile: borrow of moved value `s1`
    println!("s2 is {s2}");

    // Copy types: i32 implements Copy, so assignment duplicates the value.
    let n1 = 5;
    let n2 = n1; // n1 is copied, not moved — both remain valid
    println!("n1 is {n1}, n2 is {n2}");

    // Passing to a function also moves non-Copy types.
    let owned = String::from("world");
    takes_ownership(owned);
    // println!("{owned}"); // would fail: owned was moved into the function

    // Copy types pass by value without losing the caller's binding.
    let number = 10;
    makes_copy(number);
    println!("number is still {number}"); // still valid — i32 is Copy

    // Scope-based drop: values are dropped in reverse order when their scope ends.
    let outer = Droppable { name: String::from("outer") };
    {
        let inner = Droppable { name: String::from("inner") };
        println!("inner created");
    } // inner dropped here, before outer — its scope ends first
    println!("outer still alive");
} // outer dropped here, as main's scope ends
```
Expected output when run with `cargo run` (or `rustc` then execute):
```
s2 is hello
n1 is 5, n2 is 5
took ownership of world
received a copy: 10
number is still 10
inner created
dropping inner
outer still alive
dropping outer
```

## Problems

Starter code for each problem is scaffolded next to this file in
`src/bin/easy.rs`, `src/bin/medium.rs`, and `src/bin/hard.rs` — edit those in
place (Zed gives you full rust-analyzer intellisense via the root workspace)
and test with `cargo run --bin easy` (or `medium` / `hard`) from this lesson's
directory. `/dl-grade` reads those files; you can also paste answers into chat.
A pristine copy of the scaffold is kept in `starter/` — compare against it any
time with `diff -ru starter src`, or start over with `./run restart` (all of
`src/`) or `./run restart medium` (one program).

### Easy — "Move or copy"
Write a `fn main()` that:
1. Creates a String binding `greeting = String::from("hi")`.
2. Moves it into a new binding `greeting2` (a plain `let`, not a method call).
3. Creates an i32 binding `count = 3`.
4. Copies it into a new binding `count2` (again, a plain `let`).
5. Prints exactly two lines:
   ```
   greeting2: hi
   count is 3 and count2 is 3
   ```

**Acceptance criteria:** compiles on stable Rust; do not reuse `greeting`
after it moves — only `greeting2` should be read from after that point.
`count` may still be used after `count2` is created, since `i32` is `Copy`.

Starter:
```rust
fn main() {
    // your code here
}
```

### Medium — "Return to sender"
Write `fn process(s: String) -> String` that takes ownership of a `String`,
appends `" (processed)"` to it, and returns the new owned `String` —
ownership travels into the function and back out again, no reference
involved (that's module 3's topic).

In `main`, start with `let message = String::from("draft");`, call
`process`, and **shadow** `message` with the result (a second `let`, not
`mut` — this ties back to module 1). Print exactly: `draft (processed)`.

**Acceptance criteria:** compiles on stable; `process` takes `String` by
value (not `&String`) and returns `String`; `message` is shadowed, not
mutated.

Starter:
```rust
fn process(s: String) -> String {
    // your code here
    s
}

fn main() {
    let message = String::from("draft");
    // call process(message), shadow `message` with the result, then print it
}
```

### Hard — "Ownership relay race"
Build a three-stage ownership relay with **no `.clone()`** and **no `mut`**
on any binding you print from:

- `fn create_ticket() -> String` — returns `String::from("ticket-001")`.
- `fn stamp(ticket: String) -> String` — takes ownership, returns it with
  `"-stamped"` appended.
- `fn archive(ticket: String)` — takes ownership, prints
  `archiving {ticket}`.

Wire them together in `main` so the `String`'s ownership moves
`create_ticket -> stamp -> archive`, hand-off by hand-off (shadowing or
direct chaining — your choice).

Then define a unit struct `Session` with a `Drop` impl that prints
`closing session`. Create a `Session` value inside a nested block placed
immediately **after** the `archive()` call, so it is dropped at the end of
that block. After the block, print exactly: `main continues`.

Expected full output:
```
archiving ticket-001-stamped
closing session
main continues
```

**Acceptance criteria:** compiles on stable; every `String` hand-off is a
move (no `.clone()` anywhere); output matches exactly, in order. Be ready to
explain, when grading, which line(s) would fail to compile if you tried to
reuse `ticket` after handing it to `stamp` or `archive` — that prediction is
the real point of this problem.
