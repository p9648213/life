# Backend Engine Plan

This project is no longer centered on a toy app. Treat it as a custom backend engine that you can understand, modify, and optimize. The application you eventually build should sit on top of this backend core instead of being baked into the core design.

## Direction

The backend should make the flow easy to trace:

```text
TcpListener
  -> accept TcpStream
  -> read request bytes with limits
  -> parse HTTP request
  -> route by method and target
  -> run handler with app context
  -> build HTTP response
  -> write response bytes
  -> close or explicitly keep alive
```

Each layer should own one job. If a future optimization is needed, you should know exactly which layer to change.

## Current Feedback

Current implementation feedback lives in [feedback/current-backend-feedback.md](feedback/current-backend-feedback.md).

Phase-specific feedback lives in:

- [feedback/phase-01-feedback.md](feedback/phase-01-feedback.md)
- [feedback/phase-02-feedback.md](feedback/phase-02-feedback.md)
- [feedback/phase-03-feedback.md](feedback/phase-03-feedback.md)

## Architecture Boundaries

Keep these boundaries visible:

- `main.rs`: process startup, listener binding, accept loop.
- `connection`: read/write one TCP connection and apply connection-level limits.
- `request`: parse bytes into structured request data.
- `response`: serialize response data into bytes.
- `router`: map method/path to a handler.
- `handler`: app-specific behavior that returns a response.
- `state`: app context, shared state, configuration, storage handles.
- `storage`: persistence implementation behind a small interface.
- `security`: cookies, sessions, authentication, and authorization.
- `diagnostics`: logs, request IDs, metrics, and debug output.

Do not create every module immediately. Split when the next phase needs the boundary.

## Roadmap Pivot

### Milestone A: Protocol Core

Phases 00 through 03 already created the TCP, response, and request-parser foundation.

Phase 04 should create a router that consumes parsed request data and returns a `Response` without touching `TcpStream`.

Phase 05 and Phase 06 should add HTML/form support as adapters. They should not decide the final product domain.

Phase 07 should introduce an application state boundary, not a hard-coded to-do or notes domain.

### Milestone B: Correctness Before Speed

Add redirects, persistence, static files, cookies, sessions, authentication, and error mapping with clear tests. Keep the behavior explicit even if it is simple.

By the end of this milestone, bad client input should become a reasonable HTTP response instead of a panic or silent success.

### Milestone C: Serious Request Handling

Improve the read loop, request size limits, body limits, timeout behavior, `Host` handling, duplicate header policy, and connection-close rules.

This is where the current 256-byte single-read limitation must be removed.

### Milestone D: App Integration

After the core can route, parse bodies, hold state, persist data, and return HTML/JSON, define the boundary for real application code.

The backend core should not know your product domain. The app layer should provide handlers, state, templates or JSON serializers, and storage choices.

### Milestone E: Measurement and Optimization

Only optimize after tests and benchmarks exist. Measure:

- requests per second
- latency percentiles
- allocations in parsing and response generation
- lock contention
- file or database latency
- behavior under slow clients and larger bodies

Performance work should be tied to one measured bottleneck at a time.

## Immediate Next Phase Guidance

For Phase 04, do not start by making a big framework abstraction. Start with a small routing function and tests.

Recommended first goals:

1. Add public read access to the parsed request method and target path.
2. Create a router function that accepts a parsed request and returns a response.
3. Keep `TcpStream` out of the router.
4. Support `GET /`, `GET /health`, and a deliberate `404`.
5. Add `405 Method Not Allowed` only after you can distinguish "path exists" from "path does not exist."
6. Move route-specific HTML strings into small handler functions once the match gets noisy.
7. Add route tests that do not open TCP sockets.

The important design test: you should be able to read `handle_client` and see only connection-level flow, not app behavior.

## Dependency Policy

Default to the standard library while you are learning the mechanism.

Dependencies are acceptable when the learning target is not the unsafe primitive itself:

- Password hashing: use a proven crate such as `argon2`.
- Random session IDs: use OS-backed randomness.
- TLS: use a reverse proxy or a proven TLS stack.
- Database access: use a database crate when the goal becomes SQL, transactions, and persistence behavior.
- JSON: manual strings are fine for tiny experiments; use `serde_json` once shapes grow.
- Benchmarking and profiling tools are acceptable because their job is measurement.

Every dependency should have a short written reason.

## Done Criteria For The New Roadmap

This project is in a strong state when:

- The connection, parser, router, handler, response, and storage layers can be explained independently.
- Core behavior has tests that do not depend on opening a browser.
- Request limits and error mappings are explicit.
- App code can be changed without rewriting the server core.
- Benchmarks exist before performance rewrites.
- Security-sensitive work uses proven primitives.
- Remaining production gaps are documented instead of hidden.
