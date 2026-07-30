# Phase 09: File-Backed Storage

Goal: preserve application data across server restarts through a file-backed storage boundary.

Design the storage type, file layout, file format, ownership, module layout, and API yourself. This phase does not prescribe one file or multiple files, a connection type, or particular method names.

## Expected Behavior

Storage is configured during startup and remains available during the server lifetime. Persisted records are loaded and mutated through that storage when needed instead of being copied into application state during startup. Application state may retain the long-lived storage dependency, but how it is represented is your design.

A missing backing file represents empty stored data. Malformed persisted data produces an explicit failure instead of silently inventing records. A mutation is reported as successful only after its persistent replacement succeeds.

## Requirements

- Define an unambiguous serialization format for each stored file.
- Escape or encode field delimiters and line breaks correctly.
- Preserve IDs and obtain a safe next ID after reopening storage.
- Enforce the configured record, field, and stored-data limits while reading and writing.
- Keep reading and parsing work bounded by those limits, without repeatedly rescanning a growing input.
- Write new data to a temporary file, flush and close it, then rename it over the destination on the same filesystem.
- Do not report a mutation as successful if persistence fails.
- Distinguish a missing file from permission, corruption, and other I/O errors.
- If one operation can modify more than one file, document its atomicity boundary and what a partial failure means.

## Tests to Write

- empty storage can be opened and read;
- multiple records remain exact after reopening storage;
- delimiter, newline, and Unicode content round-trip;
- a missing backing file produces empty stored data;
- malformed and oversized persisted data are rejected;
- truncated data is rejected;
- IDs remain valid after reopening storage;
- failed persistence is not reported as success.

## Checkpoint

You are done when records can be loaded and mutated through the storage boundary, valid records survive constructing a new storage instance after restart, corrupt data fails explicitly, and replacement cannot expose a partially written destination file under the documented filesystem assumptions.

After this, continue with [Phase 10: Static Files and CSS](10-static-files-css.md).
