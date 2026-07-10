# Rust Curriculum

An ordered roadmap of modules. `/dl-next` walks this list in order for the
`rust` topic, generating one lesson at a time. Each module is a coherent
concept; a module may take one or several daily lessons before the learner
demonstrates mastery and it is marked complete in `state.json`.

Difficulty grows top to bottom. Later modules assume earlier ones.

| # | Module | Focus | Mastery signal |
|---|--------|-------|----------------|
| 1 | Values, variables, and mutability | `let`, `mut`, shadowing, scalar types, `const` | Reasons about when a binding can change and why the compiler complains |
| 2 | Ownership | Move semantics, `Copy` vs move, scope-based drop | Predicts which lines invalidate a binding |
| 3 | References and borrowing | `&`, `&mut`, the borrow rules, dangling prevention | Explains why two `&mut` at once is rejected |
| 4 | Compound types | `struct`, tuple structs, `enum`, `impl` blocks, methods | Models a small domain with structs + enums |
| 5 | Pattern matching | `match`, `if let`, `while let`, exhaustiveness, guards | Destructures nested enums without a catch-all crutch |
| 6 | Option and Result | `Option<T>`, `Result<T, E>`, combinators, `?` operator | Replaces `unwrap()` with real error flow |
| 7 | Error handling in depth | Custom error enums, `From`, `Box<dyn Error>`, `thiserror`-style | Designs an error type for a small library |
| 8 | Collections | `Vec`, `String`, `HashMap`, entry API, slices | Chooses the right collection and mutates it safely |
| 9 | Iterators | `Iterator` trait, `map`/`filter`/`fold`, laziness, `collect` | Rewrites a loop as an iterator chain and explains cost |
| 10 | Generics | Generic fns/structs, trait bounds, `where` clauses | Writes a generic function constrained by a trait |
| 11 | Traits | Defining traits, default methods, trait objects vs generics | Knows when to reach for `dyn` vs a bound |
| 12 | Lifetimes | Lifetime annotations, elision, structs holding references | Annotates a function returning a borrowed value |
| 13 | Closures and fn types | `Fn`/`FnMut`/`FnOnce`, capturing, returning closures | Passes a closure to a higher-order function |
| 14 | Smart pointers | `Box`, `Rc`, `RefCell`, interior mutability, `Rc<RefCell<T>>` | Builds a shared/mutable structure and explains the tradeoff |
| 15 | Error-free concurrency | `thread::spawn`, `move`, `Arc`, channels (`mpsc`) | Fans work out to threads and collects results |
| 16 | Shared-state concurrency | `Mutex`, `Arc<Mutex<T>>`, deadlock avoidance | Guards shared state without data races |
| 17 | Testing and modules | `#[test]`, `mod`, visibility, integration tests | Structures a crate with unit + integration tests |
| 18 | Async foundations | `async`/`await`, futures, `tokio` basics, `.await` points | Runs concurrent async tasks and joins them |
| 19 | Idiomatic Rust | Newtype pattern, builder pattern, `From`/`Into`, clippy lints | Refactors naive code into idiomatic Rust |
| 20 | Unsafe and FFI (capstone) | `unsafe` blocks, raw pointers, calling C, invariants | Justifies every line inside an `unsafe` block |

## Notes for lesson generation
- Prefer standard library only through module 17; introduce crates (`tokio`,
  `serde`, `anyhow`) explicitly and pin why they earn their place.
- Every worked example must compile as-is on stable Rust.
- Tie each lesson back to ownership/borrowing — it is the throughline.
