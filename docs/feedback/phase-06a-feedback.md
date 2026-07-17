# Phase 06A Feedback

## Overall

You completed the main Phase 06A goal: the connection layer now accumulates one complete HTTP request before it calls `Request::parse`. This means a valid request no longer fails simply because TCP delivered its headers and body in separate reads.

The request flow is now clear:

```text
TcpStream
  -> Server::read_one_request
  -> Request::parse
  -> Router
  -> handler
```

I verified that:

- Headers and bodies can arrive in separate reads.
- Body bytes that arrive with the headers count toward `Content-Length`.
- A header terminator split across reads is found correctly.
- Bodyless and `Content-Length: 0` requests stop at the end of the headers.
- A premature EOF before a complete head or declared body returns an error.
- Invalid, non-UTF-8, and overflowing `Content-Length` values return errors rather than panicking.
- The reader retains only the first request when a read also contains suffix bytes.
- The temporary 64 KiB total-request limit is enforced without rejecting an exact-capacity request whose final read also contains a suffix.
- `Content-Length` text in the request line is not treated as a header.

The deterministic reader tests pass. The two real-TCP integration tests could not bind a loopback listener in the review environment because of sandbox permissions, not because of an application failure.

## What You Did Well

- You moved the completion rule into `Server::read_one_request`, keeping TCP accumulation separate from HTTP syntax parsing.
- You search the accumulated bytes for `\r\n\r\n`, rather than assuming one `read` contains a whole request.
- You wait for the exact length declared by `Content-Length`, including any body bytes already present in the same read as the headers.
- Once the first request length is known, you append at most the remaining required bytes. This prevents an already-buffered suffix from becoming part of the parsed request.
- You use checked arithmetic when calculating the total request length, so client-controlled lengths cannot overflow `usize`.
- Your tests use a small scripted `Read` implementation. That makes fragmented-read behavior deterministic instead of relying on TCP timing.
- You added useful boundary regressions: a request-line `Content-Length` lookalike, a `Content-Length` string inside a body, a second `\r\n\r\n` inside a body, and exact-capacity suffix cases.
- The one-request, close-after-response design makes discarding suffix bytes intentional and safe for this phase.

## Things To Improve Later

- Run `cargo fmt`; the current `src/server.rs` edit has a minor spacing issue in the `enumerate()` loop. There is also an unrelated formatting suggestion in `src/main.rs`.
- `read_one_request` and `Request::parse` each inspect `Content-Length`. They currently agree on case-insensitive lookup, which is enough here. If the rule becomes more complicated, extract a small shared head-inspection helper so the two layers cannot drift.
- Duplicate `Content-Length` headers currently use the last observed value. Phase 16 should choose and enforce a deliberate duplicate-header policy.
- The temporary 64 KiB limit is appropriate for this learning phase. Phase 16 should separate header, body, and total-request limits, add timeouts, and map errors precisely to HTTP responses.
- The reader deliberately discards data after the first request. Preserve that data instead of discarding it only when you add persistent connections or HTTP pipelining.
- Form decoding does not belong here. Continue to Phase 06B with the raw bytes exposed by `request.body()`.

## Ready For Phase 06B

You are ready to continue with [Phase 06B: Form Parsing](../phases/06b-form-parsing.md). Keep the boundary from this phase: the connection reader delivers one complete request, while form parsing should work only from `request.body()` and a handler-level `Content-Type` decision.
