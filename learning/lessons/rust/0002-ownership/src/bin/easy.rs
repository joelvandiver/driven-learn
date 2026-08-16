// Easy — "Move or copy"             run with: cargo run --bin easy
//
// 1. Create a String binding `greeting = String::from("hi")`.
// 2. Move it into a new binding `greeting2` (a plain `let`, not a method call).
// 3. Create an i32 binding `count = 3`.
// 4. Copy it into a new binding `count2` (again, a plain `let`).
// 5. Print exactly two lines:
//      greeting2: hi
//      count is 3 and count2 is 3
//
// Acceptance: compiles on stable; do not reuse `greeting` after it's moved
// (it will not compile) — only read from `greeting2` afterward. `count` MAY
// still be used after `count2` is created, since i32 is Copy.

fn main() {
    // your code here
}
