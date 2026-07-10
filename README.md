# Driven Learn

A daily, self-driven study habit powered by Claude Code skills. You explore a
topic each day — currently **Rust** and **PostgreSQL** — by getting a fresh
lesson, attempting three graded problems, and asking for hints when you're
stuck. Claude tracks your progress and walks you through a real curriculum,
one module at a time.

## The three commands

| Command | What it does |
|---------|--------------|
| `/dl-next` | Generates the next lesson: a learning objective, a fully-commented worked example, and **easy / medium / hard** problems. Alternates Rust ↔ Postgres each day. |
| `/dl-grade` | Grades your attempt on the active lesson — honestly, running your code when a toolchain is available — and advances you to the next module once you've shown mastery. |
| `/dl-help` | Gives a Socratic hint for the problem you're stuck on. It nudges; it never hands over the answer. |

### Useful arguments
- `/dl-next rust` or `/dl-next postgres` — pick the topic instead of taking the rotation.
- `/dl-next review` — fresh problems on your current module without advancing.
- `/dl-grade medium` — grade just one problem.
- `/dl-help hard` — hint for a specific problem.

## How a day looks
1. Run `/dl-next`. Read the lesson and the worked example.
2. Attempt the three problems. Paste answers in chat, or save files under
   `learning/submissions/<topic>/<seq>-<slug>/` (e.g. `easy.rs`, `medium.sql`).
3. Stuck? `/dl-help`. Ready? `/dl-grade`.
4. Master Easy + Medium (and make a real dent in Hard) and the module is marked
   complete — `/dl-next` moves you forward. Otherwise, keep practicing.

## Repository layout
```
.claude/skills/
  dl-next/SKILL.md      # lesson generator
  dl-grade/SKILL.md     # grader
  dl-help/SKILL.md      # hint-giver
learning/
  state.json            # progress: current module per topic, rotation, streak, history
  curriculum/
    rust.md             # 20-module ordered roadmap
    postgres.md         # 20-module ordered roadmap
  lessons/<topic>/      # generated lessons (assigned by /dl-next)
  submissions/<topic>/  # your work
  grades/<topic>/       # grade reports from /dl-grade
```

## Adding a new topic later
1. Write `learning/curriculum/<topic>.md` as an ordered module table (copy the
   shape of `rust.md`).
2. Add an entry to `state.json` under `topics`:
   ```json
   "go": { "curriculum": "curriculum/go.md", "module": 1, "lesson_seq": 0,
            "completed_modules": [], "lessons_dir": "lessons/go" }
   ```
3. Add the topic name to the `rotation` array if you want it in the daily cycle.
4. `mkdir -p learning/lessons/go learning/grades/go`.

The skills are topic-agnostic — they read whatever topics `state.json` defines,
so no skill changes are needed to add Go, TypeScript, Kubernetes, or anything
else.
