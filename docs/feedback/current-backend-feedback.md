# Current Backend Feedback

This feedback is based on the code state after Phase 03.

## What Is Already Strong

- The project has no framework dependency, so the code flow is still visible.
- `request.rs`, `response.rs`, and `error.rs` are separated from `main.rs`.
- The response builder owns response serialization and calculates `Content-Length` from body bytes.
- The request parser returns `Result<Request, AppError>` instead of panicking on malformed input.
- Tests cover many parser edge cases: malformed request lines, invalid headers, unsupported versions, invalid UTF-8, and body length mismatches.
- The request parser borrows slices from the input buffer, which is a good learning step toward avoiding unnecessary copies.

## Important Issues To Handle Next

- `handle_client` still mixes network reading, parsing, default behavior, error response generation, and writing. Phase 04 should split routing out of it.
- `Request` stores method and path, but the useful fields are private to `request.rs`. A sibling `router.rs` will need small public accessors or a deliberate public route key.
- `handle_client` performs one `read` into a 256-byte buffer. That is acceptable for Phase 03, but it is not a serious request-reading strategy.
- Header storage currently overwrites duplicate header names. Later hardening should reject ambiguous `Content-Length` and decide how to represent repeatable headers.
- The parser accepts HTTP/1.1 but does not yet enforce useful HTTP/1.1 behavior such as `Host`.
- Error variants are good enough for learning, but serious routing and logging will benefit from more precise failure categories.
- Response serialization uses repeated `format!` calls. That is fine now; optimize only after benchmarks show serialization allocation matters.

## Phase 04 Implication

Do not start Phase 04 by inventing a large framework layer. Start by exposing the minimum request accessors, adding a router that returns a `Response`, and moving app behavior out of `handle_client`.
