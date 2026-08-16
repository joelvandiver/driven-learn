// Medium — "Return to sender"       run with: cargo run --bin medium
//
// Write `fn process(s: String) -> String` that takes ownership of a String,
// appends " (processed)" to it, and returns the new owned String — ownership
// travels into the function and back out again, no reference involved.
//
// In `main`, start with `let message = String::from("draft");`, call
// `process`, and SHADOW `message` with the result (a second `let`, not `mut`).
// Print exactly: `draft (processed)`
//
// Acceptance: compiles on stable; `process` takes `String` by value (not
// `&String`) and returns `String`; `message` is shadowed, not mutated.

fn process(s: String) -> String {
    // your code here
    s + " (processed)"
}

fn main() {
    let message = String::from("draft");
    // call process(message), shadow `message` with the result, then print it
    let message = process(message);
    println!("message={message}");
}
