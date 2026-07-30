# Phase 08 Feedback

## Overall

You completed the Phase 08 redirect response goal: handlers can return a `303 See Other` response with one validated `Location` header and an empty body.

The redirect response flow is now:

```text
handler completes a mutation
  -> Response::see_other validates the destination
  -> Response serializes 303 See Other and Location
  -> the client can make a GET request to that location
```

The resource routes remain temporary scaffolding. Their domain behavior, validation messages, and endpoint design are not part of this assessment.

I verified that:

- `cargo test --no-fail-fast` passes with 111 tests.
- `cargo fmt --check` passes.
- `cargo check --all-targets` passes.
- `cargo clippy --all-targets -- -D warnings` passes.
- `git diff --check` passes.
- `StatusCode::SeeOther` serializes as `303 See Other`.
- `Response::see_other` includes exactly one `Location` header.
- Redirect responses have an empty body and serialize `Content-Length: 0`.
- Bare CR, bare LF, and CRLF in a redirect destination are rejected.

## What You Did Well

- You represented `303 See Other` as an explicit status variant instead of passing an unstructured status number through the application.
- You added a small redirect constructor, keeping status selection and `Location` construction out of handlers.
- You made `Response::see_other` fallible because its destination becomes a response header value.
- You reused `add_header` for validation instead of adding a second, weaker header path.
- You reject CR and LF before the destination is stored, preventing response-header injection.
- You kept the redirect body empty and let the serializer calculate the correct zero-byte `Content-Length`.
- You added an exact wire-format regression that covers the status line, `Location`, framing headers, blank line, and empty body together.
- You added focused injection regressions for bare CR, bare LF, and CRLF.
- Redirect creation examines the destination once through the existing header-validation boundary. Its time and temporary memory use are linear in the destination length, with no repeated scanning or pathological growth.

## Things To Improve Later

- `HttpError::RequestHeaderInvalid` is also used when constructing an invalid response header. Phase 14 can introduce error names that distinguish request parsing from response construction when callers need that precision.
- `add_header` and `set_header` currently duplicate their validation rules. A later cleanup can centralize those rules so the two methods cannot drift.
- The current phase rejects CR and LF, which closes the response-splitting risk required here. Broader response-header grammar and control-character validation can wait for Phase 16.
- If a future application accepts redirect destinations from users, it should also enforce an application-level destination policy to prevent open redirects. That is separate from HTTP header safety.
- The temporary handlers use fixed redirect destinations, so construction cannot fail in practice. For a future dynamic destination, decide whether to validate the redirect before mutating state so a redirect-construction failure cannot leave a successful mutation followed by an error response.

## Ready For Phase 09

You are ready to continue with [Phase 09: File-Backed Storage](../phases/09-file-backed-storage.md).

Carry forward the ordering rule from this phase: a client should receive the success redirect only after the mutation and its required persistence have both succeeded. If saving fails, return an error response and follow the Phase 09 policy for restoring or retaining the in-memory state instead of redirecting as though the change were durable.
