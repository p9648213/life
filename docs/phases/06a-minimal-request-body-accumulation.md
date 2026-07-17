# Phase 06A: Minimal Request Body Accumulation

Goal: accumulate one complete HTTP request before parsing it, even when TCP delivers the request across multiple reads.

This phase fixes only the immediate single-read limitation that blocks browser form submissions. It deliberately handles one request per connection and closes the connection after the response. It does not try to implement production-grade connection handling. Phase 16 will harden this reader with deliberate limits, timeouts, and a fuller connection policy.

## Why This Phase Exists

`TcpStream::read` returns bytes that are currently available. It does not understand HTTP messages and is not required to return a complete request.

A browser can send one request like this:

```text
first read:
POST /demo/form HTTP/1.1\r\n
Host: localhost:8080\r\n
Content-Length: 22\r\n
\r\n

second read:
name=Bob&message=Hello
```

The HTTP request is valid, but parsing the first read by itself makes the body look incomplete:

```text
declared body length = 22
received body length = 0
```

A larger buffer does not solve this. Buffer capacity controls how many bytes can fit; it does not make `read` wait until the buffer or HTTP request is complete.

## What to Learn

- TCP is a byte stream, not a sequence of HTTP messages.
- One HTTP request may arrive across many `read` calls.
- The `\r\n\r\n` marker ends the request head, not necessarily the whole request.
- `Content-Length` tells you how many body bytes follow the request head.
- A read containing the end of the headers may also contain some or all of the body.
- A `read` result of `0` means EOF, not "try again later."
- Connection reading and HTTP request parsing are separate responsibilities.

## Where to Look

- Rust `Read::read`: https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read
- Rust `TcpStream`: https://doc.rust-lang.org/std/net/struct.TcpStream.html
- MDN HTTP messages: https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages
- The completed-slice parser from Phase 03

## Supported in Phase 06A

- One HTTP/1.1 request per accepted connection
- Requests whose bodies are framed with `Content-Length`
- Bodyless requests such as the current `GET` routes
- Headers and bodies split across multiple TCP reads
- Bytes returned after the first request boundary are discarded before parsing
- The existing parser, router, handler, and response flow
- A temporary total-request capacity so accumulation is not unbounded

## Deferred to Phase 16

- Separate configurable header, body, and total-request limits
- Slow-client protection and read timeouts
- Multiple requests on one persistent connection
- HTTP pipelining and preserving bytes for a following request
- Keep-alive behavior beyond the current close-after-one-request policy
- `Transfer-Encoding: chunked`
- Final duplicate `Content-Length` policy
- Precise `400`, `408`, and `413` error mapping

Do not pull those concerns into this phase. The purpose here is to make one fixed-length request reliable enough for the next form-parsing lesson.

## Responsibility Boundary

Keep the flow visible:

```text
TcpStream
  -> connection reader accumulates one complete request
  -> Request::parse validates and structures the complete bytes
  -> Router selects a handler
  -> handler reads request.body()
```

The connection layer owns waiting for enough bytes. The request parser owns HTTP syntax and validation. The form parser in Phase 06B will own decoding the body format.

`Request::parse` currently treats a short body as invalid. That is correct when it receives a supposedly complete request. The connection reader must therefore avoid calling it while the request is merely incomplete.

## The Completion Calculation

First, search all accumulated bytes for the header terminator:

```text
\r\n\r\n
```

Once it is found, calculate:

```text
head_length = separator_index + 4
body_length = parsed Content-Length, or 0 when no body length is declared
expected_total_length = head_length + body_length
```

Then compare the accumulated byte count with `expected_total_length`:

```text
received < expected  -> read more
received = expected  -> parse the complete request
received > expected  -> keep the first request, discard the suffix, then parse
```

TCP read boundaries are arbitrary. The same stream bytes might arrive in one read or several reads, so request validity must not depend on how the operating system groups them. Bytes after `expected_total_length` are not an oversized body; they are unused stream data that could belong to a following request. Phase 06A does not support following requests, so keep only `received[..expected_total_length]`, discard any already-buffered suffix, parse the first request, and close the connection after responding.

Each iteration may still read up to the fixed scratch-buffer size. Once the exact request length is known, append no more than the number of bytes the first request still needs. If the scratch buffer also contains a suffix, discard that suffix and stop as soon as the first request is complete. Do not perform another read merely to discover whether a second request is waiting. Bytes still in the socket are left unread when this one-request connection ends.

Use checked arithmetic when adding the lengths. A client controls `Content-Length`, so it must not be allowed to overflow a `usize` or bypass the temporary capacity.

The framing inspection and `Request::parse` must agree about `Content-Length`, including case-insensitive header names. Prefer extracting or reusing one small header-inspection rule instead of creating two different interpretations in the connection and request layers.

## Step-by-Step Work

1. Add a regression test that presents the request head and body as separate read chunks.
2. Move the "read one request" work behind a small connection-level helper so it can be tested independently from routing.
3. Track how many bytes in the request buffer are actually filled.
4. Read into a fixed scratch buffer and only inspect `scratch[..bytes_read]`; never inspect or append its untouched zero-filled remainder.
5. Search the complete accumulated prefix for `\r\n\r\n`, not only the newest chunk.
6. If the marker is absent, read another chunk.
7. Once the marker exists, inspect the request head for `Content-Length`.
8. Calculate the expected total length, including any body bytes already received with the headers, and reject it if it exceeds the temporary capacity.
9. If more body bytes are needed, read another scratch-buffer chunk and append at most the remaining request length. Discard any suffix returned in that same read.
10. If `read` returns `0` before the expected length arrives, report an incomplete request.
11. When at least the expected request length is available, keep the exact first-request slice, discard any suffix, and call `Request::parse` once with that slice.
12. Confirm that the handler can print the raw `request.body()` bytes.
13. Stop before splitting, decoding, or validating form fields; that belongs to Phase 06B.

## Tiny State Machine

```text
READING HEAD
  | separator not found
  +----------------------> read another chunk
  |
  | separator found
  v
READING BODY
  | received body < Content-Length
  +-------------------------------> read scratch chunk; append only needed bytes
  |
  | complete body received
  v
KEEP FIRST REQUEST -> DISCARD SUFFIX -> PARSE -> ROUTE -> HANDLE -> CLOSE
```

## Tiny Pseudocode Shape

```text
received = empty byte buffer
expected_total = unknown

loop:
  read bytes into a fixed-size scratch buffer

  if read returned EOF:
    if expected_total is unknown:
      report an incomplete request head
    if len(received) is less than expected_total:
      report an incomplete request body

  if expected_total is known:
    remaining = expected_total - len(received)
    append scratch[0..min(bytes_read, remaining)]
  otherwise:
    reject if len(received) + bytes_read exceeds temporary capacity
    append scratch[0..bytes_read]

  if expected_total is unknown:
    find header terminator in received bytes
    if it is not present:
      continue

    inspect Content-Length in the complete request head
    calculate expected_total with checked arithmetic
    reject if expected_total exceeds temporary capacity

  if len(received) is less than expected_total:
    continue

  discard received[expected_total..]
  parse received[0..expected_total]
  stop reading this request
```

This is intentionally a shape, not a finished implementation. Decide which helper returns the expected length and which error type represents EOF before completion.

## Tests to Write First

Start at the connection-reading boundary rather than adding another complete-slice parser test.

Useful cases:

- `reads_body_when_headers_and_body_arrive_separately`
- `counts_body_bytes_that_arrive_with_headers`
- `finds_header_terminator_split_across_reads`
- `bodyless_get_completes_at_end_of_headers`
- `content_length_zero_needs_no_more_body_bytes`
- `eof_before_declared_body_is_an_error`
- `header_fills_capacity_without_terminator_is_an_error`
- `declared_total_larger_than_capacity_is_an_error`

For a deterministic test, consider making the accumulation helper accept a `Read` implementation. A small scripted reader can then return predetermined chunks. That is more reliable than using timing or sleeping between two TCP writes.

The existing `Request::parse` tests should remain strict:

- A complete body of the declared length succeeds.
- A shorter body fails.
- A slice whose post-header byte count differs from `Content-Length` fails.

The connection reader is responsible for giving that strict parser exactly `received[..expected_total_length]`. It waits for incomplete requests, discards an already-buffered suffix, and does not perform an extra socket read to search for a possible following request.

## Manual Experiment

After the split-read regression passes, exercise the real server:

```bash
curl -i -X POST http://127.0.0.1:8080/demo/form \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "name=Bob&message=Hello"
```

Also submit the browser form that originally produced `Content Length Size Error`. The handler should now receive:

```text
name=Bob&message=Hello
```

The manual check proves integration. The scripted fragmented-reader test proves the behavior deterministically.

## Common Problems

Problem: the server still sometimes returns `Content Length Size Error`.

Possible causes:

- You call `Request::parse` as soon as the header terminator arrives.
- You search only the latest read instead of all accumulated bytes.
- You forgot to count body bytes that arrived with the headers.

Problem: the server hangs after switching to a read loop.

Possible cause:

- You read until EOF. A keep-alive client can wait for the response without closing its side of the connection.

Problem: the body begins with zero bytes or unrelated data.

Possible causes:

- You parsed the entire fixed-size buffer instead of only its filled prefix.
- You wrote the next read at the beginning of the buffer instead of after the bytes already received.

Problem: a delimiter split across two reads is not found.

Possible cause:

- You searched each chunk separately. Search the accumulated request prefix so `\r\n\r\n` can span chunk boundaries.

Problem: `read_exact(Content-Length)` waits for too much data.

Possible cause:

- Part of the body already arrived with the request head. Track how many request bytes remain, append no more than that amount from each scratch-buffer read, and stop as soon as the declared body is complete.

Problem: a valid near-capacity request is rejected when the final read also contains a suffix.

Possible cause:

- You counted every byte returned in the scratch buffer against the first request. Once `expected_total_length` is known, append at most the remaining request bytes; the suffix does not count toward the first request's capacity.

## Questions to Answer

- Why does a 65,536-byte buffer not guarantee a complete 100-byte request?
- What does `\r\n\r\n` finish: the request head or the whole request?
- Why must body bytes already in the first read count toward `Content-Length`?
- Why are bytes after the first request boundary not necessarily an oversized body?
- Why is discarding those bytes safe only while the connection closes after one response?
- Why is EOF different from temporarily having no bytes available?
- Why can `read_to_end` deadlock with a keep-alive client?
- Why should the connection reader avoid decoding form fields?
- Which parts of request reading remain deliberately unfinished until Phase 16?

## Checkpoint

You are done when:

- A request split across several reads is accumulated before parsing.
- A browser form POST no longer fails merely because its body arrives later than its headers.
- `Request::parse` still receives exactly one complete request slice.
- The response announces `Connection: close`, so discarded suffix bytes cannot be needed later on that connection.
- A premature EOF becomes an error instead of an infinite loop.
- The implementation retains a temporary total capacity instead of growing without bound.
- A deterministic regression test covers separated header and body chunks.
- The handler can print the exact raw body, without parsing its form fields yet.
- You can explain why neither one large `read` nor `read_to_end` is the correct completion rule.

## Continue

After the raw body reliably reaches the handler, continue with [Phase 06B: Form Parsing](06b-form-parsing.md).
