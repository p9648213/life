# Phase 12A: Thread-Based Concurrency

Goal: handle multiple clients with bounded blocking workers while preserving shared-state correctness.

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
- Use a fixed worker count and a bounded connection queue with explicit overload behavior.
- Define read/write timeouts and an overall request-read deadline so stalled or trickling clients cannot occupy workers indefinitely.
- Keep connection I/O, request processing, and shared-state synchronization separate from worker scheduling.
- Keep framing decisions independent of socket reads so a later async reader can reuse the same rules.
- Preserve atomic state invariants established in earlier phases.
- Analyze worst-case memory from active connections, buffers, queued work, and retained state.
- Keep request accumulation and parsing linear in permitted input size, including fragmented reads.

## Tests to Write

- multiple clients can complete concurrently;
- simultaneous mutations do not lose updates or duplicate IDs;
- reads observe valid state rather than partial mutations;
- a slow client does not unnecessarily hold the state lock;
- concurrency and queue limits are enforced;
- stalled and trickling clients release worker capacity within the documented deadlines;
- fragmented maximum-size requests preserve bounded scanning work and memory;
- panic or lock-poison behavior follows the documented policy.

## Checkpoint

You are done when concurrency is bounded, shared mutations remain correct under stress, and lock ownership and duration can be explained precisely.

After this, continue with [Phase 13: Better HTTP Behavior](13-better-http-behavior.md). [Phase 12B: Async I/O](12b-async-io.md) is deferred and does not block later phases.
