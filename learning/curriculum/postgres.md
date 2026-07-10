# PostgreSQL Curriculum

An ordered roadmap of modules. `/dl-next` walks this list in order for the
`postgres` topic, generating one lesson at a time. Difficulty grows top to
bottom; later modules assume earlier ones.

All examples target modern PostgreSQL (14+) and prefer standard SQL where it
exists, calling out Postgres-specific extensions explicitly.

| # | Module | Focus | Mastery signal |
|---|--------|-------|----------------|
| 1 | Querying basics | `SELECT`, `WHERE`, `ORDER BY`, `LIMIT`, operators | Filters and sorts a single table confidently |
| 2 | Data types and expressions | Numeric/text/boolean/date types, casts, `NULL` semantics | Predicts `NULL` behavior and picks correct types |
| 3 | Filtering deeper | `IN`, `BETWEEN`, `LIKE`/`ILIKE`, `IS NULL`, `CASE` | Writes a conditional expression with `CASE` |
| 4 | Aggregation | `COUNT`/`SUM`/`AVG`, `GROUP BY`, `HAVING` | Distinguishes `WHERE` from `HAVING` correctly |
| 5 | Joins | `INNER`/`LEFT`/`RIGHT`/`FULL`, join conditions, self-joins | Chooses the right join for a stated question |
| 6 | Set operations | `UNION`/`UNION ALL`, `INTERSECT`, `EXCEPT` | Combines result sets without duplicating rows wrongly |
| 7 | Subqueries | Scalar, `IN`, correlated subqueries, `EXISTS` | Rewrites a correlated subquery and reasons about cost |
| 8 | CTEs | `WITH`, chained CTEs, readability, recursive CTEs | Builds a recursive CTE for a hierarchy |
| 9 | Window functions | `OVER`, `PARTITION BY`, `ROW_NUMBER`/`RANK`/`LAG`/`LEAD` | Computes a running total and a per-group rank |
| 10 | Data modeling | Keys, normalization, FKs, constraints, `CHECK` | Designs a normalized schema for a small domain |
| 11 | DML and upserts | `INSERT`/`UPDATE`/`DELETE`, `RETURNING`, `ON CONFLICT` | Writes an idempotent upsert |
| 12 | Transactions | `BEGIN`/`COMMIT`/`ROLLBACK`, ACID, savepoints | Wraps a multi-step change safely |
| 13 | Isolation and locking | Isolation levels, MVCC, `SELECT ... FOR UPDATE`, deadlocks | Explains a lost-update and prevents it |
| 14 | Indexes | B-tree, unique, partial, expression, multicolumn indexes | Adds the right index for a slow query |
| 15 | Query planning | `EXPLAIN`/`EXPLAIN ANALYZE`, scan types, statistics | Reads a plan and names the bottleneck |
| 16 | JSON and JSONB | `->`/`->>`, `@>`, `jsonb_*`, GIN indexes on JSONB | Queries and indexes semi-structured data |
| 17 | Arrays and enums | Array columns, `unnest`, `ANY`/`ALL`, enum types | Uses arrays without reaching for a junction table needlessly |
| 18 | Full-text search | `tsvector`/`tsquery`, `to_tsvector`, ranking, GIN | Builds a searchable text column |
| 19 | Functions and triggers | `CREATE FUNCTION`, PL/pgSQL, triggers, `NEW`/`OLD` | Writes a trigger that maintains a derived value |
| 20 | Performance capstone | Denormalization tradeoffs, partitioning, `VACUUM`, connection cost | Diagnoses and fixes a realistic slow workload |

## Notes for lesson generation
- Every worked example must be runnable: provide the `CREATE TABLE` + seed
  `INSERT`s alongside the query so the learner can reproduce it.
- Show the expected output as a small table so the learner can self-check.
- Call out where behavior is Postgres-specific vs. ANSI SQL.
