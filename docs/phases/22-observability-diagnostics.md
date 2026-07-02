# Phase 22: Observability and Diagnostics

Goal: make the server explain what happened without exposing private details to users.

Good diagnostics help you debug correctness first and performance later.

## What to Learn

- Request logging
- Error logging
- Request IDs
- Timing measurements
- Debug output versus user-facing output
- Simple counters

## Where to Look

- Rust time: https://doc.rust-lang.org/std/time/
- `eprintln!`: https://doc.rust-lang.org/std/macro.eprintln.html
- HTTP status codes: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status

## Step-by-Step Work

1. Log one line per request.
2. Include method, path, status code, and duration.
3. Add a simple request ID.
4. Log parser and handler errors server-side.
5. Keep browser responses safe and short.
6. Add counters for total requests and error responses.

## Questions to Answer

- What information helps debug a failed request?
- What information should not be logged?
- Where should timing start and stop?
- How can logs stay useful under many requests?

## Checkpoint

You are done when:

- A request can be traced through logs.
- Internal errors are visible server-side.
- User-facing errors do not leak internals.
