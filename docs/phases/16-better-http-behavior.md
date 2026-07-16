# Phase 16: Better HTTP Behavior

Goal: make your tiny HTTP server less fragile.

You still are not implementing all HTTP. You are adding limits and explicit behavior.

Phase 06A introduced only enough accumulation to deliver one complete `Content-Length`-framed request. This phase revisits that provisional reader and hardens it. Do not replace the connection/request-parser boundary established there.

## What to Learn

- Request size limits
- Header size limits
- Body size limits
- Connection close behavior
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
6. Choose and test one duplicate `Content-Length` policy. For this small server, rejecting every duplicate is the simplest safe rule, and the framing helper and `Request::parse` must share it.
7. While no transfer coding is implemented, reject every `Transfer-Encoding` header and reject requests containing both `Transfer-Encoding` and `Content-Length`.
8. Return `413 Payload Too Large` or `400 Bad Request` when appropriate.
9. Add `Connection: close` if you close after every response.
10. Add a read-timeout policy for slow or stalled clients.

## Scope Decision

It is acceptable to close every connection after one request.

If so, be explicit:

```text
Connection: close
```

Persistent connections are useful, but they add complexity.

It is also acceptable to keep chunked bodies, pipelining, and multiple requests per connection unsupported. Reject unsupported framing explicitly instead of interpreting it as a bodyless request.

## Questions to Answer

- Why can unlimited request size hurt the server?
- Why is keep-alive harder than one request per connection?
- What should happen when `Content-Length` is missing on a POST?

## Checkpoint

You are done when:

- Large headers are rejected.
- Large bodies are rejected.
- Fragmented requests that fit within the limits still pass the Phase 06A regression tests.
- Connection behavior is documented.
