# Phase 18: Database Layer

Goal: replace file persistence with a database-backed storage adapter.

Design the schema, repository boundary, and migration layout yourself.

## Expected Behavior

Application behavior remains the same while records persist in SQLite through explicit queries and transactions.

## Requirements

- Use SQLite first unless the chosen application requires something else.
- Use a proven database crate; do not reimplement a database protocol.
- Define primary keys, constraints, and useful indexes explicitly.
- Add versioned, repeatable migrations.
- Keep SQL and connection lifecycle outside HTTP parsing and response code.
- Use transactions for multi-step changes that must be atomic.
- Map constraint, unavailable, and unexpected database failures appropriately.
- Bound query results and avoid accidental unbounded loads.
- Preserve data or provide an explicit migration path from file storage.

## Tests to Write

- migrations work on an empty database;
- migrations can recognize an already-current database;
- create, list, and detail behavior matches the prior storage adapter;
- constraints reject invalid state;
- transaction failure rolls back all steps;
- restart preserves records;
- query limits are enforced.

## Checkpoint

You are done when SQLite persists the application correctly, migrations are repeatable, and storage can change without altering the HTTP core.

After this, continue with [Phase 19: JSON API](19-json-api.md).
