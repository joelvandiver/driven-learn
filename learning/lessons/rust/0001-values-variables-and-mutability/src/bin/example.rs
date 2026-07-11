// Worked example — type it in yourself from lesson.md ("Worked example"),
// then check it with: ./run example
// Typing it (not pasting) is the point: expected output is in the lesson.

fn main() {
    // type the worked example here
    let count = 5;
    println!("count starts at {count}");

    let mut score = 0;
    score = score + 10;
    score += 5;
    println!("score is {score}");

    let count = count + 1;
    println!("count is now {count}");

    let input = "42";
    let input: i32 = input.trim().parse().expect("not a number");
    println!("input doubled is {}", input * 2);

    const MAX_SCORE: i32 = 100;
    let capped = if score > MAX_SCORE { MAX_SCORE } else { score };
    println!("capped score: {capped}");

    let level = 1;
    {
        let level = level * 10;
        println!("inner level: {level}");
    }
    println!("outer level: {level}");

    let mut level = 1;
    {
        level = level * 10;
        println!("inner level: {level}");
    }
    println!("outer level from mut inner: {level}");
}
