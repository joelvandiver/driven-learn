// Worked example from the lesson — run with: cargo run --bin example
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
