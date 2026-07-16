# Phase 06A: Minimal Request Body Accumulation

Goal: accumulate one complete HTTP request before parsing it, even when TCP delivers the request across multiple reads.

This phase fixes only the immediate single-read limitation that blocks browser form submissions. It does not try to implement production-grade connection handling. Phase 16 will harden this reader with deliberate limits, timeouts, and connection policy.

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
- The existing parser, router, handler, and response flow
- A temporary total-request capacity so accumulation is not unbounded

## Deferred to Phase 16

- Separate configurable header, body, and total-request limits
- Slow-client protection and read timeouts
- Multiple requests on one persistent connection
- HTTP pipelining and preserving bytes for a following request
- Full `Connection: close` and keep-alive behavior
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
received > expected  -> reject surplus that is already accumulated
```

Once the exact request length is known, limit later reads to the remaining length. Do not perform another read merely to discover whether a second request is waiting. Extra bytes already accumulated can be rejected; bytes still in the socket are left unread when this one-request connection ends. Phase 16 will make the connection policy explicit.

Use checked arithmetic when adding the lengths. A client controls `Content-Length`, so it must not be allowed to overflow a `usize` or bypass the temporary capacity.

The framing inspection and `Request::parse` must agree about `Content-Length`, including case-insensitive header names. Prefer extracting or reusing one small header-inspection rule instead of creating two different interpretations in the connection and request layers.

## Step-by-Step Work

1. Add a regression test that presents the request head and body as separate read chunks.
2. Move the "read one request" work behind a small connection-level helper so it can be tested independently from routing.
3. Track how many bytes in the request buffer are actually filled.
4. Read only into unused capacity; never parse the zero-filled remainder of the buffer. If capacity is exhausted before the request is complete, return an error without calling `read` with an empty slice.
5. Search the complete accumulated prefix for `\r\n\r\n`, not only the newest chunk.
6. If the marker is absent, read another chunk.
7. Once the marker exists, inspect the request head for `Content-Length`.
8. Calculate the expected total length, including any body bytes already received with the headers, and reject it if it exceeds the temporary capacity.
9. If more body bytes are needed, limit later reads to the remaining request length.
10. If `read` returns `0` before the expected length arrives, report an incomplete request.
11. When the exact request length is available, call `Request::parse` once with that complete slice.
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
  +-------------------------------> read remaining bytes
  |
  | exact body length received
  v
PARSE REQUEST -> ROUTE -> HANDLE
```

## Tiny Pseudocode Shape

```text
received = 0
expected_total = unknown

loop:
  if expected_total is known:
    writable_length = min(unused_capacity, expected_total - received)
  otherwise:
    writable_length = unused_capacity

  if writable_length is zero before the request is complete:
    report temporary capacity exceeded

  read bytes into that writable request-buffer region

  if read returned EOF:
    if received is zero:
      finish the empty connection
    otherwise:
      report an incomplete request

  increase received by bytes_read

  if expected_total is unknown:
    find header terminator in buffer[0..received]
    if it is not present:
      continue

    inspect Content-Length in the complete request head
    calculate expected_total with checked arithmetic
    reject if expected_total exceeds temporary capacity

  if received is less than expected_total:
    continue

  if received is greater than expected_total:
    reject surplus bytes that were already accumulated

  parse buffer[0..expected_total]
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
- `surplus_already_in_the_accumulator_is_rejected`

For a deterministic test, consider making the accumulation helper accept a `Read` implementation. A small scripted reader can then return predetermined chunks. That is more reliable than using timing or sleeping between two TCP writes.

The existing `Request::parse` tests should remain strict:

- A complete body of the declared length succeeds.
- A shorter body fails.
- A complete slice containing surplus bytes fails while pipelining is unsupported.

The new tests prove that the connection layer waits before invoking that strict parser. They do not require an extra socket read to search for a possible following request.

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

- Part of the body already arrived with the request head. Read only `Content-Length - body_bytes_already_received` additional bytes.

Problem: a full request buffer is reported as EOF.

Possible cause:

- You called `read` with an empty destination slice. Check remaining capacity before reading and report capacity exhaustion separately.

## Questions to Answer

- Why does a 65,536-byte buffer not guarantee a complete 100-byte request?
- What does `\r\n\r\n` finish: the request head or the whole request?
- Why must body bytes already in the first read count toward `Content-Length`?
- Why is EOF different from temporarily having no bytes available?
- Why can `read_to_end` deadlock with a keep-alive client?
- Why should the connection reader avoid decoding form fields?
- Which parts of request reading remain deliberately unfinished until Phase 16?

## Checkpoint

You are done when:

- A request split across several reads is accumulated before parsing.
- A browser form POST no longer fails merely because its body arrives later than its headers.
- `Request::parse` still receives exactly one complete request slice.
- A premature EOF becomes an error instead of an infinite loop.
- The implementation retains a temporary total capacity instead of growing without bound.
- A deterministic regression test covers separated header and body chunks.
- The handler can print the exact raw body, without parsing its form fields yet.
- You can explain why neither one large `read` nor `read_to_end` is the correct completion rule.

## Continue

After the raw body reliably reaches the handler, continue with [Phase 06B: Form Parsing](06b-form-parsing.md).
