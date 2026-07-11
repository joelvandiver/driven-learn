---
name: dl-next
description: Deliver the next daily lesson in the Driven Learn self-study system. Invoke when the user types /dl-next (optionally with a topic like "rust" or "postgres", or "review"). Reads learning/state.json and the topic curriculum, generates one progressive lesson (objective, worked example, and easy/medium/hard problems), saves it, and advances progress state.
---

# /dl-next — generate the next lesson

You are the learner's tutor for a daily self-study habit. Produce ONE lesson,
save it, update state, and present it in chat. Keep it tight and motivating.

## Step 1 — Load state
Read `learning/state.json`. If it is missing, tell the user to run the setup
(the repo should contain it) and stop.

## Step 2 — Choose the topic
- If the user passed an argument that names a topic in `state.topics`
  (e.g. `/dl-next postgres`), use it.
- If the argument is `review`, regenerate fresh problems for the learner's
  most recent *incomplete* module instead of advancing — skip Step 6's
  advancement.
- Otherwise pick `rotation[rotation_index]`, then advance
  `rotation_index = (rotation_index + 1) % rotation.length` so topics alternate
  day to day.

## Step 3 — Find the module
Open the chosen topic's `curriculum` file. Take the module whose number equals
`topics[topic].module`. That module's row (focus + mastery signal) is your
spec. If `module` exceeds the last module, congratulate the learner on finishing
the track and offer `review` or a new topic; stop.

## Step 4 — Compose the lesson
Write a self-contained lesson with EXACTLY these sections:

1. **Learning objective** — one or two sentences naming the skill and why it
   matters. Anchor it to the module's "mastery signal".
2. **Concept** — a short, plain-language explanation (a few paragraphs). No
   fluff. Name the common beginner trap for this concept.
3. **Worked example** — real, runnable code with a comment on nearly every
   meaningful line explaining *why*, not just *what*.
   - Rust: must compile on stable. Prefer std lib per the curriculum notes.
   - Postgres: include `CREATE TABLE` + seed `INSERT`s + the query, and show
     the expected result as a small table so it is self-checking.
4. **Problems** — exactly three, each clearly labeled with its difficulty and a
   short title:
   - **Easy** — direct application of the worked example.
   - **Medium** — combines the concept with something from an earlier module.
   - **Hard** — an open-ended or edge-case problem that rewards real
     understanding; state the goal, not the steps.
   For each problem give: the prompt, the expected inputs/outputs or acceptance
   criteria, and (Rust) a starter signature or (Postgres) the schema to use.
   Do NOT include solutions anywhere in the lesson file.

Calibrate difficulty to the module's position in the curriculum — early modules
stay gentle; later modules assume prior modules.

## Step 5 — Save the lesson and scaffold the code
Each lesson is a DIRECTORY holding the markdown plus runnable starter code, so
the learner can test their work in place with full IDE intellisense.

- Increment `topics[topic].lesson_seq` by 1; call it `SEQ` (zero-padded to 4).
- Slugify the module title (lowercase, hyphens).
- Create `learning/{lessons_dir}/{SEQ}-{slug}/` and write the lesson to
  `lesson.md` inside it.
- Scaffold the code next to it:
  - **Rust** — a crate in the lesson directory: `Cargo.toml` with
    `name = "lesson-{SEQ}-{slug}"`, current stable edition, `publish = false`;
    plus `src/bin/example.rs` — an EMPTY `fn main()` stub whose header comment
    tells the learner to type the worked example in from `lesson.md` (typing,
    not pasting, is deliberate retention practice; do NOT put the example code
    in the file) — and `src/bin/easy.rs` / `medium.rs` / `hard.rs` — each a
    starter `fn main()` with the problem prompt and acceptance criteria as
    header comments (no solutions). Also copy the previous Rust lesson's `run` script into the new
    lesson directory (`chmod +x`), updating its header comment to name this
    lesson's problems — it lists and runs the lesson's binaries
    (`./run easy` etc.) and resets work from the pristine snapshot
    (`./run restart [program]`). Debugging needs no per-lesson setup: the repo-level
    `.zed/debug.json` config works for any lesson binary via the open file.
    Finally, snapshot the untouched scaffold with `cp -r src starter` — the
    learner diffs their work against `starter/` and restores from it; cargo
    ignores it. Never edit `starter/` after scaffolding.
    The root `Cargo.toml` workspace glob
    (`learning/lessons/rust/*`) picks the crate up automatically — do not edit
    it. Verify with `cargo build` from the repo root, and verify the worked
    example by compiling and running the lesson.md code in the session
    scratchpad (NOT in the lesson crate — `example.rs` stays a stub); its
    output must match what the lesson claims.
  - **Postgres** — `setup.sql` (the worked example's `CREATE TABLE`s + seed
    `INSERT`s + the example query) and `easy.sql` / `medium.sql` / `hard.sql`
    starters, each with the prompt as a header comment. Snapshot pristine
    copies into `starter/` (`mkdir starter && cp *.sql starter/`), same as the
    Rust `starter/` convention.
  - In the lesson's Problems section, tell the learner to edit those starter
    files in place and how to run them (Rust: `cargo run --bin easy` from the
    lesson directory).
- Prepend YAML frontmatter to `lesson.md`:
  ```
  ---
  topic: <topic>
  module: <n>
  module_title: <title>
  seq: <SEQ>
  date: <today's date>
  status: assigned
  ---
  ```

## Step 6 — Update state
Unless this was a `review` run:
- Set `active_lesson` to `{ "topic": ..., "module": ..., "seq": ..., "path": ..., "assigned_date": <today> }`.
- Set `last_lesson_date` to today.
- If `last_lesson_date` was yesterday or this is the first lesson, increment
  `streak`; if a day was skipped, reset `streak` to 1. (Today's date is given
  in the session context.)
- Append `{ "seq": SEQ, "topic": ..., "module": ..., "event": "assigned", "date": <today> }` to `history`.
Write `state.json` back.

## Step 7 — Present
Show the full lesson in chat (the same content you saved). End with a short
line telling the learner they can attempt the problems, then run `/dl-grade`
to submit, or `/dl-help` if they get stuck — and note their current streak.
Add a light reminder that they can run `/dl-log` to save the session's
conversation notes whenever they're wrapping up.

## Step 8 — Persist progress
The runtime is ephemeral, so progress only survives once it's pushed. Commit
the updated `learning/state.json` and the new lesson file and push to the
current branch:
`git add learning/ && git commit -m "dl-next: assign {topic} module {n} ({SEQ})" && git push`
If the push fails on a network error, retry with backoff. If it fails because
there's no upstream, set one with `git push -u origin <current-branch>`. Never
switch branches to do this.

## Where the learner puts work
The learner edits the scaffolded starter files in the lesson directory
(`src/bin/easy.rs` etc. for Rust, `easy.sql` etc. for Postgres) — their work
lives right next to `lesson.md`. When you present the lesson, remind them of
this and that they can alternatively paste answers into chat when grading.
`/dl-grade` looks in both places.
