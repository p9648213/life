# Phase 16: Better HTTP Behavior

Goal: make your tiny HTTP server less fragile.

You still are not implementing all HTTP. You are adding limits and explicit behavior.

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

1. Decide a maximum header size.
2. Read until `\r\n\r\n` or the limit is reached.
3. Return `413 Payload Too Large` or `400 Bad Request` when appropriate.
4. Decide a maximum body size.
5. Use `Content-Length` to reject huge bodies before reading them.
6. Add `Connection: close` if you close after every response.
7. Consider read timeouts.

## Scope Decision

It is acceptable to close every connection after one request.

If so, be explicit:

```text
Connection: close
```

Persistent connections are useful, but they add complexity.

## Questions to Answer

- Why can unlimited request size hurt the server?
- Why is keep-alive harder than one request per connection?
- What should happen when `Content-Length` is missing on a POST?

## Checkpoint

You are done when:

- Large headers are rejected.
- Large bodies are rejected.
- Connection behavior is documented.

