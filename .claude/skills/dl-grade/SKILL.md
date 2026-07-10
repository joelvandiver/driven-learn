---
name: dl-grade
description: Grade the learner's work on their active Driven Learn lesson. Invoke when the user types /dl-grade (optionally pasting their solutions, or naming a problem like "easy"). Locates the active lesson and the learner's submission, evaluates each problem honestly (running code when a toolchain is available), writes a grade report, and advances progress on a pass.
---

# /dl-grade — grade the active lesson

Be an honest, encouraging tutor. Praise what is right, name what is wrong
precisely, and never inflate a grade. A wrong-but-close answer is more useful
feedback than a gold star.

## Step 1 — Find the lesson
Read `learning/state.json`. Take `active_lesson`. If it is null, tell the user
to run `/dl-next` first and stop. Read the lesson file at `active_lesson.path`
to recover the three problems and their acceptance criteria.

## Step 2 — Collect the submission
Gather the learner's work, in priority order:
1. Code/answers pasted in the current chat message (or recent messages).
2. Files under `learning/submissions/{topic}/{SEQ}-{slug}/`.
If you find work for only some problems, grade those and mark the rest
"not attempted". If the user named a single problem (e.g. `/dl-grade medium`),
grade only that one.
If you find nothing, ask the user to paste their answer or point you to the
files, then stop.

## Step 3 — Evaluate each attempted problem
For each problem, judge on three axes and VERIFY, don't just eyeball:
- **Correctness** — does it meet the acceptance criteria?
  - Rust: if `cargo`/`rustc` is available, compile and run it in a scratch dir
    (use the session scratchpad). Report compiler errors verbatim if it fails.
  - Postgres: if `psql`/a local server is available, run it against the lesson's
    schema. Otherwise, trace the query by hand against the seed data and state
    that you reasoned it through rather than executing.
  - If you cannot execute, say so explicitly and reason carefully instead of
    claiming a pass you did not verify.
- **Idiomatic style** — is it written the way a fluent practitioner would? Point
  to the specific line and name the better idiom.
- **Understanding** — does the approach show they grasped the concept, or did
  they pattern-match their way to a lucky answer?

Give each problem a verdict: ✅ Pass / 🟡 Partial / ❌ Not yet, plus 2–4 lines
of specific feedback. Show the corrected/idiomatic version ONLY for problems
that already passed or that the learner has clearly wrestled with — do not hand
over solutions to untried problems (that is `/dl-help`'s job, and even it
withholds full answers).

## Step 4 — Overall result and mastery
Summarize with a short rubric table (problem | verdict | one-line note).
Decide mastery of the module:
- **Mastered** if Easy and Medium both pass and Hard is at least Partial with
  genuine understanding shown.
- Otherwise **keep practicing** — recommend `/dl-help` or a `/dl-next {topic}
  review` for fresh problems on the same module.

## Step 5 — Write the grade report
Write to `learning/grades/{topic}/{SEQ}-{slug}.md` with frontmatter
(`topic`, `module`, `seq`, `date`, `result: mastered|practicing`) followed by
the rubric and feedback. Update the lesson file's frontmatter `status` to
`graded`.

## Step 6 — Update state
- Append a `history` entry: `{ "seq": ..., "topic": ..., "module": ..., "event": "graded", "result": ..., "date": <today> }`.
- If **mastered**:
  - Add the module number to `topics[topic].completed_modules`.
  - Set `topics[topic].module += 1`.
  - Clear `active_lesson` to null.
  - Tell the learner the module is complete and `/dl-next` will move on.
- If **practicing**: leave `module` and `active_lesson` as-is.
Write `state.json` back.

## Step 7 — Persist progress
The runtime is ephemeral, so progress only survives once it's pushed. Stage the
learner's submission (if it lives in `learning/submissions/`), the grade report,
the updated lesson frontmatter, and `learning/state.json`, then commit and push
to the current branch:
`git add learning/ && git commit -m "dl-grade: {topic} module {n} ({SEQ}) — {result}" && git push`
If the learner pasted their code in chat rather than saving files, first write
it to `learning/submissions/{topic}/{SEQ}-{slug}/` so their attempt is captured
in the repo too. Retry the push with backoff on network errors.

## Tone
Lead with something they got right. Be concrete about fixes. Close with one
sentence on what to focus on next. Never shame; never flatter.
