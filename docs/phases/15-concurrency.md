# Phase 15: Concurrency

Goal: handle multiple clients while preserving shared-state correctness.

Design the worker and synchronization structure yourself.

## Expected Behavior

More than one client can make progress concurrently, and shared application state remains consistent under simultaneous reads and mutations.

## Requirements

- Use safe Rust synchronization; do not introduce `unsafe` shared mutation.
- Give every connection an explicit execution owner.
- Share only data that must be shared.
- Keep lock scope short and visible.
- Do not hold a state lock while reading a slow request, rendering unrelated work, or writing a response to the network.
- Define behavior for poisoned locks and handler panics.
- Bound concurrency so clients cannot create unlimited threads or queued work.
- Preserve atomic state invariants established in earlier phases.
- Analyze worst-case memory from active connections, buffers, queued work, and retained state.

## Tests to Write

- multiple clients can complete concurrently;
- simultaneous mutations do not lose updates or duplicate IDs;
- reads observe valid state rather than partial mutations;
- a slow client does not unnecessarily hold the state lock;
- concurrency and queue limits are enforced;
- panic or lock-poison behavior follows the documented policy.

## Checkpoint

You are done when concurrency is bounded, shared mutations remain correct under stress, and lock ownership and duration can be explained precisely.

After this, continue with [Phase 16: Better HTTP Behavior](16-better-http-behavior.md).
