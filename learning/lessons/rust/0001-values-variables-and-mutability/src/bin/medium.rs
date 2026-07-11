// Medium — "Parse and shadow"       run with: cargo run --bin medium
//
// Using SHADOWING (not `mut`), turn `raw` into an integer and print its square.
//
// 1. Start with `let raw = "  16  ";` (already below).
// 2. Shadow `raw` into an i32 by trimming and parsing it.
// 3. Print exactly: `square is 256`
//
// Acceptance: compiles on stable; prints `square is 256`. You must reuse the
// name `raw` via a second `let` (a type-changing shadow), and must NOT use `mut`.

fn main() {
    let raw = "  16  ";
    // shadow `raw` into an i32, then print its square
    let raw: i32 = raw.trim().parse().expect("expect a number");
    println!("square is {}", raw * raw);
}
