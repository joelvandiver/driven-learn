// Hard — "Ownership relay race"     run with: cargo run --bin hard
//
// Build a three-stage ownership relay with NO `.clone()` and NO `mut` on any
// binding you print from:
//
//   fn create_ticket() -> String       returns String::from("ticket-001")
//   fn stamp(ticket: String) -> String takes ownership, returns it with
//                                      "-stamped" appended
//   fn archive(ticket: String)         takes ownership, prints:
//                                      `archiving {ticket}`
//
// Wire them together in `main` so the String's ownership moves from
// create_ticket -> stamp -> archive, hand-off by hand-off (shadowing or
// direct chaining, your choice).
//
// Then define a unit struct `Session` with a `Drop` impl that prints
// `closing session`. Create a `Session` value inside a nested block placed
// immediately AFTER the archive() call, so it is dropped at the end of that
// block. After the block, print exactly: `main continues`.
//
// Expected full output:
//   archiving ticket-001-stamped
//   closing session
//   main continues
//
// Acceptance: compiles on stable; every String hand-off is a move (no
// `.clone()` anywhere); output matches exactly, in order. Be ready to explain,
// when grading, which line(s) would fail to compile if you tried to reuse
// `ticket` after handing it to `stamp` or `archive`.

fn create_ticket() -> String {
    String::from("ticket-001")
}

fn stamp(ticket: String) -> String {
    ticket + "-stamped"
}

fn archive(ticket: String) {
    println!("archiving {ticket}")
}

struct Session;

impl Drop for Session {
    fn drop(&mut self) {
        println!("closing session")
    }
}

fn main() {
    let ticket = create_ticket();
    let stamped = stamp(ticket);
    archive(stamped);

    {
        let _session = Session;
    }
    println!("main continues")
}
