// Easy — "Warm-up counter"          run with: cargo run --bin easy
//
// 1. Create an immutable binding `start` equal to 3.
// 2. Create a mutable binding `total`, initialized to `start`.
// 3. Add 7 to `total` using `+=`.
// 4. Print exactly: `total is 10`
//
// Acceptance: compiles on stable; prints `total is 10` and nothing else.
// `start` must remain immutable (no `mut` on it).

fn main() {
    let start = 3;
    let mut total = start;
    total += 7;
    println!("total is {total}");
}
