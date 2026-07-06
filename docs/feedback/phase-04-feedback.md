# Phase 04 Feedback

## Overall

You completed the main goal of Phase 04: a parsed request can now be routed to different behavior by method and exact path, without the connection code knowing the route lookup details.

The backend now has a clear first routing boundary:

- `Server` owns TCP reading, request parsing, and response writing.
- `Router` owns method/path lookup and the fallback `404 Not Found` response.
- Handlers receive a parsed `Request` and return a `Response`.
- `Request` keeps the raw target, normalized path, and query parameters as separate concepts.

I verified that:

- `cargo fmt --check` passes.
- `cargo check` passes.
- `cargo clippy --all-targets -- -D warnings` passes.
- Full `cargo test` passes with 36 tests.
- Router tests run without opening a TCP socket.
- Query-bearing `GET` and `POST` targets match routes using the path without the query string.
- Unknown paths return `404 Not Found`.
- A path registered for a different method returns `404 Not Found`, matching the current project policy.
- Query parsing behavior is covered for valid values, empty values, repeated keys, encoded values, `+`, empty keys, empty query strings, and malformed pairs.

## What You Did Well

- You moved routing out of `handle_client`, so the connection code no longer performs route lookup itself.
- You placed `Router` under the HTTP module and made it operate on `Request` and `Response`, not `TcpStream`.
- You introduced explicit route registration with `get` and `post`, keeping the supported method set obvious.
- You made `Router::handle_request` the routing boundary. That is a better design than exposing route maps and making callers perform lookup.
- You used `request.path()` for route matching, which keeps query strings out of route lookup.
- You kept the raw request target available through `target_path()`, so debugging and future HTTP behavior can still see what the client actually sent.
- You named request lifetimes by meaning, such as `'buf`, which makes the parser ownership model easier to understand.
- You named the response lifetime `'header`, making it clear that the response lifetime is about borrowed header names and values, not the body.
- You kept dependencies minimal and did not introduce a web framework.
- You added focused tests for routing behavior instead of relying only on TCP integration tests.

## Things To Improve Later

- `handle_client` still performs one read into a fixed 512-byte buffer. This is acceptable for the current phase, but real request reading needs a loop, size limits, and clearer handling for partial reads.
- The router currently stores function pointers. That is simple and good for learning, but later application state may require a more flexible handler shape.
- `Server` exposes `routes` publicly. This is fine for now, but later you may want route registration methods on `Server` so callers do not depend directly on its internal fields.
- Query parsing is intentionally small. The current behavior is tested, but future phases may need a dedicated query representation if repeated keys, decoding, or malformed encodings become important.
- Wrong-method requests currently return `404`. That is your current policy. If you later choose `405 Method Not Allowed`, remember that a correct `405` response should include an `Allow` header.
- `Response` still borrows header names and values. That is useful for learning lifetimes, but owning headers as `String`s may eventually make handler APIs simpler.
- Route lookup currently supports exact static paths only. Keep that constraint until a real application requirement justifies dynamic path segments.

## Answer Review

Your answers and follow-up changes show that you understand the important Phase 04 boundaries:

- The router should not know about `TcpStream`.
- A handler should return a `Response`, not write directly to the client.
- Route matching should use method plus normalized path.
- The query string belongs to request parsing and handler validation, not route lookup.
- Request buffer lifetimes, request borrows, route storage lifetimes, and response header lifetimes are separate concepts.

## Ready For Phase 05

You are ready to move on to Phase 05: HTML rendering.

The main idea to carry forward is that handlers should stay small and return responses through clear boundaries. As HTML grows, avoid mixing large strings directly into route matching code. Let routing choose the handler, and let the handler call rendering helpers when the markup becomes large enough to need structure.
