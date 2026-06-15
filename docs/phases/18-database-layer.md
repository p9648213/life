# Phase 18: Optional Database Layer

Goal: understand what a database gives you after building file storage.

Do this only after file-backed storage works.

## What to Learn

- Tables
- Rows
- Primary keys
- Indexes
- Transactions
- Migrations
- Connection lifecycle

## Where to Look

- SQLite docs: https://www.sqlite.org/docs.html
- SQLite transactions: https://www.sqlite.org/lang_transaction.html
- PostgreSQL docs, later: https://www.postgresql.org/docs/

## Step-by-Step Work

1. Choose SQLite first.
2. Design a `notes` table.
3. Write schema manually.
4. Add a small migration step.
5. Replace file load/save with SQL queries.
6. Use transactions for multi-step changes.
7. Compare code complexity with file storage.

## Dependency Rule

It is reasonable to use a database crate here. The learning target is SQL and database behavior, not reimplementing the SQLite wire format.

## Questions to Answer

- What problems did the database solve?
- What new problems did it add?
- What does a transaction protect?
- How is a primary key different from your in-memory counter?

## Checkpoint

You are done when:

- Notes persist in SQLite.
- You can explain the schema.
- You understand why transactions matter.

