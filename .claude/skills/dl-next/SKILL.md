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

## Step 5 — Save the lesson
- Increment `topics[topic].lesson_seq` by 1; call it `SEQ` (zero-padded to 4).
- Slugify the module title (lowercase, hyphens).
- Write the lesson to `learning/{lessons_dir}/{SEQ}-{slug}.md`.
- Prepend YAML frontmatter to that file:
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

## Where the learner puts work
When you present the lesson, remind them they can either paste their answers
into chat when grading, or drop files under
`learning/submissions/{topic}/{SEQ}-{slug}/` (e.g. `easy.rs`, `medium.sql`).
`/dl-grade` looks in both places.
