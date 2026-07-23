# Phase 22: Observability and Diagnostics

Goal: make request behavior diagnosable without exposing private data.

Design the logging and metrics boundary yourself.

## Expected Behavior

One request can be traced from acceptance to response using a request ID, status, duration, and safe error context. Basic counters describe traffic and failures.

## Requirements

- Give each request a bounded, safe identifier.
- Record method, normalized path, status, and duration.
- Log internal errors with enough context to diagnose the responsible boundary.
- Keep user-facing errors short and separate from diagnostic detail.
- Do not log passwords, session IDs, cookies, authorization values, sensitive form bodies, or query secrets.
- Define whether client addresses and user identifiers are logged.
- Keep log volume bounded under invalid or high-rate traffic.
- Make counters safe under concurrency.
- Ensure diagnostics do not change response correctness.

## Tests to Write

- successful and failed requests produce the expected safe fields;
- request IDs correlate related entries;
- secrets and sensitive bodies are absent or redacted;
- user-facing responses do not contain internal errors;
- counters remain correct under concurrent updates;
- logging failure does not corrupt the HTTP response.

## Checkpoint

You are done when failures can be traced server-side, basic traffic can be counted, and diagnostic output has an explicit privacy boundary.

After this, continue with [Phase 23: Benchmarking and Profiling](23-benchmarking-profiling.md).
