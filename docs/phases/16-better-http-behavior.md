# Phase 16: Better HTTP Behavior

Goal: make your tiny HTTP server less fragile.

You still are not implementing all HTTP. You are adding limits and explicit behavior.

Phase 06A introduced only enough accumulation to deliver one complete `Content-Length`-framed request. This phase revisits that provisional reader and hardens it. Do not replace the connection/request-parser boundary established there.

## What to Learn

- Request size limits
- Header size limits
- Body size limits
- Reads bounded by the current request boundary
- Connection close behavior
- Surplus-byte handling for the chosen connection policy
- Slow clients
- Denial-of-service basics

## Where to Look

- MDN HTTP messages: https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages
- MDN Connection header: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection
- `TcpStream` timeouts: https://doc.rust-lang.org/std/net/struct.TcpStream.html

## Step-by-Step Work

1. Review the temporary total-request capacity from Phase 06A.
2. Decide a separate maximum header size.
3. Enforce that limit while accumulating until `\r\n\r\n`.
4. Decide a maximum body size.
5. Use `Content-Length` to reject huge bodies before deliberately reading any additional body bytes; the header read may already contain part of the body.
6. Once the expected total request length is known, limit each socket read to `min(remaining_request_bytes, scratch_buffer_length)`. Append only the number of bytes that `read` actually returned, and continue when it returns fewer bytes than requested.
7. Choose the connection policy before deciding what to do with bytes already buffered after the first request boundary:
   - If every connection closes after one response, discard the suffix and send `Connection: close`.
   - If the connection can process another request, preserve the suffix in the connection buffer and begin the next accumulation pass with those bytes.
8. Choose and test one duplicate `Content-Length` policy. For this small server, rejecting every duplicate is the simplest safe rule, and the framing helper and `Request::parse` must share it.
9. While no transfer coding is implemented, reject every `Transfer-Encoding` header and reject requests containing both `Transfer-Encoding` and `Content-Length`.
10. Return `413 Payload Too Large` or `400 Bad Request` when appropriate.
11. Add a read-timeout policy for slow or stalled clients.

Limiting later reads avoids deliberately consuming bytes beyond a known request boundary. It does not remove the need for an explicit surplus policy: the read that first reveals `\r\n\r\n` may already contain the complete body and part of a following request. TCP read boundaries are arbitrary, so do not reject an otherwise complete first request merely because that read also contained a suffix.

## Scope Decision

It is acceptable to close every connection after one request.

If so, be explicit:

```text
Connection: close
```

With this policy, discarding bytes after the first request boundary is deliberate and safe because the connection will not be reused. Still limit later reads once the boundary is known so the reader does not consume surplus unnecessarily.

Persistent connections are useful, but they add complexity.

If you choose to support them, the connection layer must retain already-buffered surplus bytes. Those bytes are not part of the first request body and must not be passed to its strict parser; they become the starting bytes for the next request.

It is also acceptable to keep chunked bodies, pipelining, and multiple requests per connection unsupported. Reject unsupported framing explicitly instead of interpreting it as a bodyless request.

## Questions to Answer

- Why can unlimited request size hurt the server?
- Why is keep-alive harder than one request per connection?
- Why can limiting later reads still leave surplus bytes in the connection buffer?
- When is it safe to discard bytes after the first request boundary?
- What should happen when `Content-Length` is missing on a POST?

## Checkpoint

You are done when:

- Large headers are rejected.
- Large bodies are rejected.
- Once the expected request length is known, each later read is bounded by the remaining request length.
- Tests cover a short final body read and a bounded final read that leaves available suffix bytes unread.
- A test covers suffix bytes accumulated before the request boundary was known. Those bytes follow the documented connection policy: discard them only when closing after the response, or preserve them when another request may follow.
- Fragmented requests that fit within the limits still pass the Phase 06A regression tests.
- Connection behavior is documented.
