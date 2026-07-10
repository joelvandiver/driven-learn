---
name: dl-log
description: Snapshot the current conversation into the repo journal and commit all pending Driven Learn progress. Invoke when the user types /dl-log, asks to "save this conversation", or when a scheduled reminder fires. Writes a dated session summary to journal/ and commits + pushes any uncommitted lessons, submissions, grades, and state.
---

# /dl-log — journal the session and persist progress

Capture what happened so it lives in the repo, then make sure nothing is left
uncommitted. The runtime is ephemeral: if it isn't pushed, it's gone.

## Step 1 — Write the journal entry
Create a new markdown file at `journal/{YYYY-MM-DD}-{NN}-{slug}.md`, where:
- `YYYY-MM-DD` is today's date (given in session context).
- `NN` is a two-digit counter — the next unused number for today (start at `01`;
  if a file for today already exists, increment).
- `slug` is a few-word kebab-case summary of the session's theme.

Give it frontmatter:
```
---
date: <today>
kind: session-journal
topics_touched: [<any dl topics worked on, else omit>]
---
```
Then write a faithful, useful summary — NOT a verbatim transcript (you don't
have one). Include:
- **Summary** — 2–4 sentences on what this session was about.
- **Decisions & changes** — bullet list of what was built, changed, or decided,
  referencing files by path.
- **Learning progress** — if any lesson was assigned or graded this session,
  note the topic/module/result and current streak (read `learning/state.json`).
- **Open threads / next steps** — anything left to do.
Be honest and concrete. If the session had no learning activity (e.g. it was
housekeeping), say so plainly.

## Step 2 — Commit and push everything pending
Stage the journal entry plus any uncommitted work under `learning/` and the
skills, then commit and push to the current branch:
```
git add -A
git commit -m "dl-log: session journal {date} + progress snapshot"
git push
```
- If there is nothing to commit, say so and skip the commit — don't create an
  empty commit.
- Set an upstream with `-u origin <current-branch>` if none exists.
- Retry the push with exponential backoff on network errors (2s, 4s, 8s, 16s).
- Never switch branches to do this; commit on whatever branch is checked out.

## Step 3 — Confirm
Tell the user what you journaled, the commit hash, and that it's pushed. If a
scheduled reminder invoked this and there was nothing new to save, keep the
report to one line.

## Note on what can and cannot be captured
This skill journals the session it runs in. A scheduled run in a *fresh* session
can only snapshot committed repo state and write a housekeeping entry — it
cannot recover a conversation from a different session it never saw. To save a
specific chat, run `/dl-log` inside that chat before the session ends.
