# Phase 12B: Async I/O (Deferred)

Goal: handle many waiting connections using async network I/O while preserving Phase 12A's correctness and resource limits.

Return after Phase 12A when ready to learn async or when connection workloads justify it. This phase does not block Phase 13 or the rest of the roadmap. Design the runtime integration and execution boundaries yourself.

## Expected Behavior

Waiting for network I/O yields execution capacity to other connections. HTTP behavior and shared-state invariants established by completed phases remain intact.

## Requirements

- Use async connection acceptance, reads, and writes; document and justify any runtime dependency without adding a web framework.
- Give every connection task an explicit owner and bound active connections, queued work, and blocking work.
- Preserve overload behavior, read/write timeouts, and request deadlines from Phase 12A.
- Keep blocking storage, static-file operations, and lengthy computation off threads responsible for async progress; synchronous handlers may remain behind a bounded blocking-work boundary.
- Do not hold shared-state locks across network waits; preserve atomic storage operations and short lock scopes.
- Define cleanup for disconnects, timeouts, cancellation, and task panics, including blocking operations that may continue after their request ends.
- Reuse framing rules and byte-processing logic; making every handler async is not required.
- Keep request accumulation and parsing linear under fragmented input. Bound total memory across active tasks, buffers, queues, and in-flight blocking work.

## Tests to Write

- many waiting clients allow an unrelated ready request to complete within a defined deadline;
- slow blocking work does not stall unrelated network progress;
- connection, queue, and blocking-work limits hold under overload;
- disconnects, timeouts, and cancellation release capacity according to policy without corrupting shared state;
- concurrent mutations and panic handling preserve established invariants;
- existing HTTP regressions pass, including fragmented maximum-size requests with bounded work and memory.

## Checkpoint

You are done when async network waits allow other connections to progress, blocking work is isolated and bounded, and existing protocol and storage guarantees remain covered by tests. Record a reproducible comparison with Phase 12A under waiting-client workloads; do not assume async improves every workload.

After this, resume the main roadmap at [Phase 13: Better HTTP Behavior](13-better-http-behavior.md) or your next unfinished phase.
