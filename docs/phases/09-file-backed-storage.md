# Phase 09: File-Backed Storage

Goal: persist app state across server restarts.

Before using a database, feel the problems that databases solve.

## What to Learn

- Reading files
- Writing files
- Serialization
- Deserialization
- Atomic replacement
- Handling corrupted data

## Where to Look

- Rust file I/O: https://doc.rust-lang.org/std/fs/
- Rust `File`: https://doc.rust-lang.org/std/fs/struct.File.html
- Rust error handling: https://doc.rust-lang.org/book/ch09-00-error-handling.html

## Storage Format

Start simple. You can invent a line-based format, for example:

```text
id<TAB>title<TAB>body-with-escaped-newlines
```

This teaches serialization. Later, using JSON is fine.

## Step-by-Step Work

1. Decide a data file path, for example `data/records.txt`.
2. On startup, try to read the file.
3. If the file does not exist, start with empty records.
4. Parse each line into a record.
5. After creating, updating, or deleting records, write all records back.
6. Write to a temporary file first.
7. Rename the temporary file over the real file.
8. Handle malformed lines explicitly.

## Why Temporary File Then Rename?

If your process crashes halfway through writing, the real file may be corrupted. A common pattern is:

```text
write new content to records.txt.tmp
flush/close it
rename records.txt.tmp to records.txt
```

Rename is usually atomic on the same filesystem.

## Questions to Answer

- What happens if the file does not exist?
- What happens if a line is malformed?
- What happens if the process crashes while writing?
- Why is escaping needed in your storage format?

## Checkpoint

You are done when:

- Records survive restart.
- Missing data file is handled.
- Malformed data does not silently create nonsense state.
