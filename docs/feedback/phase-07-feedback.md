# Phase 07 Feedback

## Overall

You completed the Phase 07 goal: application state is created once, owned for the lifetime of the server, passed explicitly through the request flow, and shared across sequential requests.

The state flow is now:

```text
main creates State
  -> Server<T> owns it
  -> Server::handle_client borrows it mutably
  -> Router<T> passes it to the selected handler
  -> one handler mutates it
  -> a later request observes the mutation
```

The resource routes are temporary scaffolding used only to demonstrate this flow. Their domain model, endpoint design, validation, and response behavior are not part of the Phase 07 assessment.

I verified that:

- `cargo test --all --all-targets` passes with 109 tests.
- `cargo clippy --all-targets --all-features -- -D warnings` passes.
- `cargo fmt --all -- --check` passes.
- `git diff --check` passes.
- State is constructed before the server enters its connection-accept loop.
- `Server<T>` owns one state value instead of creating state per connection.
- `Router<T>` and its handlers receive state explicitly rather than using global mutable state.
- A socket-free test proves that one request can mutate state and a later request can observe the same value.
- Existing stateless router and server tests use `()` as their state, demonstrating that the backend remains generic over the application state type.
- The implementation remains single-threaded and does not introduce `Arc` or `Mutex`.

## What You Did Well

- You made state ownership visible in the type system: `Server<T>` owns `T`, and handlers receive `&mut T`.
- You kept the state flow explicit from server to router to handler instead of hiding it behind a global or framework-style extractor.
- You changed `handle_client` and `run` to borrow the server mutably, matching the fact that request handling may mutate server-owned application state.
- You kept `Server` and `Router` independent of the temporary resource type by making both generic.
- You create the state once in `main`, so every accepted connection handled by that server instance reaches the same value.
- You preserved the single-threaded model. Exclusive `&mut T` access is sufficient here; synchronization belongs in Phase 15.
- You repaired the older routing and server tests after changing the handler API instead of leaving stale tests behind.
- You added a focused state test that exercises the real router-to-handler boundary without depending on TCP timing or socket permissions.

## Things To Improve Later

- `Server` currently exposes both `routes` and `state` publicly. This keeps the learning flow easy to inspect. A later API-boundary phase can decide whether registration and state access should be encapsulated.
- The router stores function pointers. That is small and explicit for the current backend. Revisit the handler representation only when closures, captured dependencies, middleware, or another concrete requirement makes function pointers insufficient.
- The temporary `State` type contains the resource demonstration data inside the library crate. The generic backend does not depend on it, so it can move into a clearer application layer when Phase 24 formalizes the backend-core boundary.
- State exists only in process memory. Phase 09 should add loading and saving while preserving the ownership path established here.
- Do not add `Arc`, `Mutex`, or concurrent mutation behavior early. Phase 15 should introduce synchronization together with a deliberate concurrency model and concurrent tests.

## Ready For Phase 08

You are ready to continue with [Phase 08: Redirects](../phases/08-redirects.md).

Carry forward the central boundary from this phase: handlers may change application state, but they should still communicate the next client action by returning a `Response`. Phase 08 can add redirect responses without changing how state is owned or passed.
