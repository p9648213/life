# Step-by-Step Implementation Path

Complete and test one small boundary before moving to the next.

## 1. Specify the Format

Document magic bytes, version, byte order, integer widths, record framing, and
one concrete record field order.

Checkpoint: you can draw every byte in an empty file and one record.

## 2. Choose Limits and Errors

Define file, record-count, record, field, and decoded-allocation bounds. Define
errors that distinguish invalid identifiers, I/O, corruption, unsupported
versions, truncation, limits, UTF-8, IDs, and replacement.

Checkpoint: tests can distinguish missing, corrupt, unsupported, and inaccessible
collections.

## 3. Build Fixed-Width Primitive Codecs

Implement the integer types your format needs. Test documented byte order,
round-trips, boundaries, and truncation.

Checkpoint: primitive tests do not depend on `Resource` or files.

## 4. Build Bounded String Codecs

Encode UTF-8 byte length followed by exact bytes. Validate lengths before
allocation. Test empty, Unicode, delimiters, invalid UTF-8, truncation, and
over-limit declarations.

Checkpoint: string tests operate entirely on in-memory bytes.

## 5. Encode and Decode One Record

Give one application record a manual stable field order. Require its decoder to
consume the exact record payload.

Checkpoint: `decode(encode(value))` reproduces the value, and the record codec
knows no file paths.

## 6. Validate Collection Identifiers

Give `Store` a narrow identifier grammar and resolve accepted names beneath its
root. Test traversal, absolute paths, separators, and boundary length.

Checkpoint: an accepted collection cannot escape the storage root.

## 7. Read Complete Collections

Implement missing-file behavior, header validation, record framing, exact
consumption, every limit, and explicit errors.

Checkpoint: a fresh storage instance reopens multiple valid records, while all
malformed fixtures fail deliberately.

## 8. Define Stable IDs

Choose the identity boundary and safely derive the next ID after reopening.

Checkpoint: restart and deletion cannot change or reuse an existing identity.

## 9. Encode Complete Snapshots

Combine header and framed record payloads while enforcing the total file limit.

Checkpoint: zero, one, and multiple records round-trip entirely in memory.

## 10. Add Temporary-File Replacement

Write, flush, close, and same-filesystem rename the complete replacement. Never
report mutation success before rename succeeds.

Checkpoint: an injected failure before rename preserves the old valid file.

## 11. Add Typed Operations

Connect the pieces into only the operations the application currently needs,
such as listing and inserting. Prefer typed operations over SQL-like strings.

Checkpoint: application code never manipulates collection bytes directly.

## 12. Integrate Handlers

Only now connect storage to the temporary resource handlers. Map errors
deliberately; do not default corruption to empty data or ignore mutation results.

Checkpoint: a handler cannot return a successful mutation response after
persistence failed.

## 13. Audit Worst-Case Work

For a file of `F` bytes, confirm `O(F)` decoding with bounded memory. Add a
maximum-size valid test and re-evaluate work whenever a limit changes.

Checkpoint: every input byte is examined only a bounded number of times.

## Current Prototype Reminder

Replace these behaviors deliberately as their steps arrive:

- `read_to_string` treats the binary collection as UTF-8;
- the current read is unbounded;
- unchecked path joining permits escape attempts;
- listing creates a missing file;
- unconstrained `T` provides no codec;
- insertion ignores its item and reports success;
- defaulting list errors hides corruption and I/O failures.

Do not begin with handler integration or attempt to fill all of `insert_one` at
once. Start with Step 1, then keep each checkpoint green.
