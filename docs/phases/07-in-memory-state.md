# Phase 07: In-Memory State

Goal: keep application data available across multiple requests while the server process is running.

Design the types, modules, and ownership structure yourself.

## Expected Behavior

Create state once when the server starts and use that same state while handling every request.

Use a small temporary resource route only to demonstrate that state survives across requests. For example:

```text
POST /resources   -> change state
GET  /resources   -> observe the changed state
```

The resource model, fields, validation rules, IDs, limits, endpoint design, and response behavior are not part of this phase. They are disposable test scaffolding, not an application-domain implementation to review.

Restarting the process should clear the state.

## Requirements

- Create the state once, outside the connection-accept loop.
- Pass state explicitly to handlers; do not use global mutable state.
- Keep application data separate from the TCP, HTTP, and routing code.
- Allow a handler to mutate state and a later request to observe that mutation.
- Keep the server and router generic over the application state where practical.
- Keep the server single-threaded; do not add `Arc` or `Mutex` yet.

## Tests to Write

- the initial list is empty;
- one request changes the state;
- multiple requests observe the same state;
- state behavior can be tested without opening a TCP socket.

## Checkpoint

You are done when state is created once, passed explicitly through the backend flow, mutated by one request, and observed by a later request. The backend core should remain independent of the temporary resource domain.

Do not evaluate completion based on the design or completeness of the resource example. Resource behavior can be replaced or discarded after it proves that shared state works.

After this, continue with [Phase 08: Redirects](08-redirects.md).
