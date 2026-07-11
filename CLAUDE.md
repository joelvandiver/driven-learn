# CLAUDE.md — Driven Learn

This repo is a personal daily-learning system driven by three skills:
`/dl-next`, `/dl-grade`, `/dl-help` (see `.claude/skills/`). When the user
invokes one, follow that skill's `SKILL.md` exactly — it is the source of truth
for reading and writing `learning/state.json`.

## Conventions
- **Single source of progress:** `learning/state.json`. Always read it before
  acting and write it back atomically after. Never let it drift from the files
  on disk (lessons assigned, grades written).
- **Curricula are ordered roadmaps**, not scripts. Generate lessons on demand
  from the module row; don't pre-write the whole track.
- **Worked examples must be runnable.** Rust compiles on stable; Postgres ships
  with its schema + seed data + expected output.
- **Lessons are directories with code, not lone markdown files.** Every lesson
  is `learning/lessons/{topic}/{seq}-{slug}/` containing `lesson.md` plus
  scaffolded starter code the learner edits in place (Rust: a crate with
  `src/bin/{worked_example,easy,medium,hard}.rs`; Postgres: `setup.sql` +
  per-problem `.sql` starters). Rust lesson crates are picked up by the root
  `Cargo.toml` workspace glob so the learner's IDE (Zed/rust-analyzer) gets
  intellisense — never write a Rust lesson without its crate.
- **Never leak solutions.** Lesson files and `/dl-help` never contain full
  answers to unattempted problems. `/dl-grade` only shows idiomatic solutions
  for work the learner has genuinely attempted.
- **Honest grading.** Verify by running code when a toolchain exists; otherwise
  say you reasoned it by hand. Don't claim an unverified pass.

## Extending
Topics are data, not code. Add a curriculum file + a `state.json` topic entry;
the skills pick it up with no edits. See README.md.
