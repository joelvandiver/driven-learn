---
topic: postgres
module: 1
module_title: Querying basics
seq: 0001
date: 2026-07-22
status: assigned
---

# Postgres · Module 1 — Querying basics

## Learning objective

Read data out of a single table on your terms: pick the rows you want with
`WHERE`, put them in the order you want with `ORDER BY`, and take only as many as
you need with `LIMIT`. By the end you should be able to filter and sort one
table confidently — the foundation every later query is built on.

## Concept

A `SELECT` statement answers one question: *"from this table, show me these
columns, for these rows, in this order."* The clauses run in a fixed logical
order even though you write them top-to-bottom:

1. `FROM books` — start from the whole table.
2. `WHERE genre = 'fiction'` — keep only rows where the condition is **true**.
3. `ORDER BY price ASC` — sort the surviving rows (`ASC` low→high, `DESC` high→low).
4. `LIMIT 3` — take the first N rows *after* sorting.

The comparison operators are what you'd expect: `=`, `<>` (not equal — SQL's
spelling, though `!=` also works in Postgres), `<`, `<=`, `>`, `>=`. Combine
conditions with `AND` / `OR`, and negate with `NOT`. Strings are compared with
single quotes (`'fiction'`); double quotes mean something else entirely (an
identifier), which is a classic beginner trip-up.

**The beginner trap — `LIMIT` without `ORDER BY`.** A table has no inherent
order. If you write `SELECT ... LIMIT 3` with no `ORDER BY`, Postgres is free to
return *any* three rows, and the set can change between runs or after an update.
"Give me the 3 cheapest books" is only correct if you sort by price first. When
you `LIMIT`, always `ORDER BY` something — and if that something can tie, add a
tiebreaker column so the order is fully determined.

## Worked example

*Question: the 3 cheapest fiction books published after 2000 — cheapest first.*

```sql
-- A tiny catalog to query against.
CREATE TABLE books (
    id             int PRIMARY KEY,   -- unique row id
    title          text NOT NULL,     -- book title
    author         text NOT NULL,     -- last name is enough for us
    genre          text NOT NULL,     -- 'fiction' or 'tech'
    published_year int  NOT NULL,     -- year of publication
    pages          int  NOT NULL,     -- length in pages
    price          numeric(5,2) NOT NULL  -- price with 2 decimal places
);

-- Seed rows. Mixed genres, years, and prices so filters have something to bite.
INSERT INTO books (id, title, author, genre, published_year, pages, price) VALUES
    (1, 'The Rust Programming Language',           'Klabnik',   'tech',    2018, 560, 39.95),
    (2, 'Dune',                                    'Herbert',   'fiction', 1965, 412,  9.99),
    (3, 'Project Hail Mary',                       'Weir',      'fiction', 2021, 496, 14.99),
    (4, 'The Pragmatic Programmer',                'Hunt',      'tech',    1999, 352, 49.99),
    (5, 'Klara and the Sun',                       'Ishiguro',  'fiction', 2021, 320, 12.50),
    (6, 'Neuromancer',                             'Gibson',    'fiction', 1984, 271,  8.99),
    (7, 'Designing Data-Intensive Applications',   'Kleppmann', 'tech',    2017, 616, 44.99),
    (8, 'The Midnight Library',                     'Haig',      'fiction', 2020, 304, 11.00);

-- The query:
SELECT title,                        -- only the columns we care about, not SELECT *
       published_year,
       price
FROM books                           -- 1. start from every row in books
WHERE genre = 'fiction'              -- 2. keep fiction only ...
  AND published_year > 2000          --    ... AND published this century
ORDER BY price ASC                   -- 3. cheapest first (ASC is the default, shown for clarity)
LIMIT 3;                             -- 4. take the top 3 after sorting
```

Rows that survive the `WHERE`: *Project Hail Mary* (14.99), *Klara and the Sun*
(12.50), *The Midnight Library* (11.00) — exactly three fiction books after
2000. Sorted cheapest-first and limited to 3, the result is:

| title                | published_year | price |
|----------------------|---------------:|------:|
| The Midnight Library |           2020 | 11.00 |
| Klara and the Sun    |           2021 | 12.50 |
| Project Hail Mary    |           2021 | 14.99 |

*(Expected output reasoned by hand — no Postgres toolchain is installed in this
environment. Run it yourself with `psql -f setup.sql` if you have one and
confirm the table matches.)*

## Problems

All three problems use the `books` table from `setup.sql`. **Edit the starter
files in this directory in place** — `easy.sql`, `medium.sql`, `hard.sql` — and
run them against a database seeded with `setup.sql` (`psql -f setup.sql` then
`psql -f easy.sql`), or just paste your queries into chat when you run
`/dl-grade`.

### Easy — Longest tech books
Write one query that lists every **tech** book, longest first by page count.
Show `title` and `pages`.

- **Acceptance:** exactly the 3 tech books, ordered by `pages` descending
  (the 616-page book first, the 352-page book last).

### Medium — A slice of mid-length fiction
List the **fiction** books whose length is **between 250 and 450 pages**
(inclusive), oldest first by `published_year`, breaking any ties by `title`
alphabetically. Show `title`, `published_year`, and `pages`.

- **Acceptance:** 4 rows, all fiction, every one in the 250–450 page range,
  ordered by year ascending (the 1965 book first). Use a comparison on both
  ends of the range and a two-level `ORDER BY`.

### Hard — Page 2 of the price list
Imagine the catalog is shown 3 books per page, sorted by `price` from cheapest
to most expensive. Return **page 2** — the 4th, 5th, and 6th cheapest books.
Show `title` and `price`. Your ordering must be **fully deterministic**: if two
books ever shared a price, the same three rows must come back every single time.

- **Goal, not steps:** figure out how to skip the first page and take the next
  three, and how to guarantee a stable order even under price ties. There are
  8 books, so page 2 is the middle slice of the price ranking.
- **Acceptance:** exactly 3 rows — the books ranked 4th–6th by ascending price —
  with a tiebreaker that makes the result repeatable.

---

Attempt the three problems, then run `/dl-grade` to submit (or `/dl-grade easy`
for just one). Stuck on any of them? `/dl-help` gives a nudge, never the answer.
This is your **1st** lesson on the postgres track — streak: **1**. When you're
wrapping up, `/dl-log` saves the session notes and commits your progress.
