# Phase 09: File-Backed Storage

Goal: preserve application state across server restarts.

Design the storage type, file format, and module layout yourself.

## Expected Behavior

State loads from a configured file during startup and is saved after successful mutations. A missing file starts with empty state; malformed persisted data produces an explicit failure instead of silently inventing records.

## Requirements

- Define an unambiguous serialization format.
- Escape or encode field delimiters and line breaks correctly.
- Preserve IDs and calculate a safe next ID after loading.
- Enforce the same record and field limits used in memory.
- Write new data to a temporary file, flush and close it, then rename it over the destination on the same filesystem.
- Do not report a mutation as successful if persistence fails.
- Distinguish a missing file from permission, corruption, and other I/O errors.
- Define what happens to in-memory state when a save fails.

## Tests to Write

- empty state round-trips;
- multiple records round-trip exactly;
- delimiter, newline, and Unicode content round-trip;
- missing file produces empty state;
- malformed and oversized persisted data are rejected;
- truncated data is rejected;
- IDs remain valid after reload;
- failed persistence is not reported as success.

## Checkpoint

You are done when valid records survive restart, corrupt data fails explicitly, and replacement cannot expose a partially written destination file under the documented filesystem assumptions.

After this, continue with [Phase 10: Static Files and CSS](10-static-files-css.md).
