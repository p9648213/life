# Phase 07: In-Memory State

Goal: keep application data available across multiple requests while the server process is running.

Design the types, modules, and ownership structure yourself.

## Expected Behavior

Implement a small temporary resource with enough fields to demonstrate state.

Support:

```text
POST /resources          -> create a resource
GET  /resources          -> list resources
GET  /resources?id=123   -> view one resource
```

Use the same state for every request handled by the running server. Restarting the process should clear it.

## Requirements

- Create the state once, outside the connection-accept loop.
- Pass state explicitly to handlers; do not use global mutable state.
- Keep application data separate from the TCP, HTTP, and routing code.
- Validate all input before changing state.
- Give created resources stable, increasing IDs.
- Handle ID overflow without wrapping or panicking.
- Set limits for retained record count and retained field sizes.
- Reject a failed creation without changing either the records or next ID.
- HTML-escape user-controlled data when rendering it.
- Keep the server single-threaded; do not add `Arc` or `Mutex` yet.

For `GET /resources`:

- missing `id` means list all resources;
- an empty, repeated, or non-numeric `id` is a client error;
- a valid but unknown ID returns `404 Not Found`.

A bounded linear scan for an ID is acceptable in this phase. Listing and lookup should not clone the entire state unnecessarily.

## Tests to Write

- the initial list is empty;
- one valid request creates one resource;
- multiple requests observe the same state;
- IDs increase without duplication;
- list and detail requests return the stored data;
- invalid and unknown IDs return the expected errors;
- invalid input does not mutate state;
- record-limit and ID-overflow failures are atomic;
- rendered user data is escaped;
- state behavior can be tested without opening a TCP socket.

## Checkpoint

You are done when resources can be created, listed, and retrieved across requests; memory growth and IDs are bounded; failed mutations leave state unchanged; and the backend core remains independent of the temporary application domain.

After this, continue with [Phase 08: Redirects](08-redirects.md).
