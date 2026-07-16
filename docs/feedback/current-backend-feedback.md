# Current Backend Feedback

This feedback is based on the code state after Phase 04.

## What Is Already Strong

- The project has no framework dependency, so the backend flow is still visible.
- `request.rs`, `response.rs`, `error.rs`, and `router.rs` are separated into clear HTTP concerns.
- `Server` owns TCP reading, parsing, and response writing, while `Router` owns method/path lookup.
- The response builder owns response serialization and calculates `Content-Length` from body bytes.
- The request parser returns `Result<Request, AppError>` instead of panicking on malformed input.
- The request parser keeps the raw target, normalized path, and query parameters distinct.
- Tests cover parser edge cases, response serialization, router behavior, and basic TCP integration.
- Router tests run without opening a TCP socket, which makes route behavior easy to exercise directly.
- Lifetimes are named by meaning in the request and response types, making the borrowing model easier to follow.

## Important Issues To Handle Next

- `handle_client` still performs one read into a fixed buffer. Phase 06A should replace that with minimal complete-request accumulation; Phase 16 should harden it with limits, timeouts, and explicit connection behavior.
- The router stores function pointers. This is simple now, but future application state may require a different handler shape.
- `Server` exposes `routes` publicly. Later, route registration methods on `Server` may give a cleaner application boundary.
- Query parsing behavior is now tested, but the supported subset is intentionally small. Percent-decoding, repeated keys, and malformed encodings may need a richer representation later.
- Header storage currently overwrites duplicate header names. Later hardening should reject ambiguous `Content-Length` and decide how to represent repeatable headers.
- The parser accepts HTTP/1.1 but does not yet enforce useful HTTP/1.1 behavior such as `Host`.
- Response serialization uses repeated `format!` calls. That is fine now; optimize only after benchmarks show serialization allocation matters.

## Phase 05 Implication

HTML rendering should build on the new routing boundary. Let the router select handlers, and let handlers call small rendering helpers once markup grows beyond tiny inline strings.
