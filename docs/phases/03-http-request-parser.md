# Phase 03: HTTP Request Parser

Goal: parse enough of an HTTP request to route it.

You are not implementing the full HTTP specification. You are building a small parser for the subset your app needs.

## What to Learn

- Request line
- Method
- Path
- Version
- Headers
- Body
- `Content-Length`
- Why TCP reads may be partial

## Where to Look

- MDN HTTP messages: https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages
- Rust string slices: https://doc.rust-lang.org/book/ch04-03-slices.html
- Rust `str`: https://doc.rust-lang.org/std/primitive.str.html
- Rust `Result`: https://doc.rust-lang.org/std/result/

## Supported Subset

Support now:

- HTTP/1.1
- `GET`
- `POST`
- Headers separated by `\r\n`
- Body length from `Content-Length`

Explicitly do not support yet:

- Chunked transfer encoding
- HTTP/2
- Pipelining
- Multipart upload
- Compression

Writing down what you do not support is part of the learning.

## Request Shape

A simple request looks like:

```text
GET /notes HTTP/1.1\r\n
Host: 127.0.0.1:8080\r\n
User-Agent: curl/...\r\n
\r\n
```

A form POST looks like:

```text
POST /notes HTTP/1.1\r\n
Host: 127.0.0.1:8080\r\n
Content-Length: 22\r\n
Content-Type: application/x-www-form-urlencoded\r\n
\r\n
title=Hello&body=World
```

## Step-by-Step Work

1. Capture raw request bytes from phase 1.
2. Find where headers end. The marker is `\r\n\r\n`.
3. Convert the header bytes to text.
4. Split the header text into lines.
5. Parse the first line into three parts:
   - method
   - target path
   - version
6. Parse each following header line into name and value.
7. Find `Content-Length` if present.
8. Read body bytes according to that length.
9. Return a structured request type.
10. Return an error if required pieces are missing.

## Tiny Pseudocode Shape

```text
parse_request(bytes):
  find header/body separator
  parse request line
  parse headers
  calculate expected body length
  ensure body bytes are available
  return Request
```

## Important Design Choice

At first, it is acceptable to read into a fixed-size buffer and parse what you got. But you must understand the weakness:

- TCP is a stream.
- One `read` call is not guaranteed to contain the full request.
- Large or slow requests may arrive in pieces.

Later, phase 16 will improve this.

For now, document this limitation.

## Experiments

Use these:

```bash
curl -v http://127.0.0.1:8080/
curl -v http://127.0.0.1:8080/notes
curl -v -X POST http://127.0.0.1:8080/notes -d "title=Hello&body=World"
```

Print your parsed request in debug form.

## Common Problems

Problem: parser fails on normal browser requests.

Possible cause:

- Browser sends more headers than expected.
- Your parser assumes a fixed number of lines.

Problem: body is empty for POST.

Possible causes:

- You stopped parsing at the blank line but ignored bytes after it.
- You did not use `Content-Length`.

Problem: header value has spaces.

Possible cause:

- You split on every `:` instead of only the first one.

## Questions to Answer

- What is the exact first line of a request called?
- What separates headers from the body?
- Why should malformed input return `400 Bad Request`?
- Why is one `read` call not enough in a real server?

## Checkpoint

You are done when:

- `GET /` parses into method and path.
- `POST /notes` parses headers and body.
- Bad request text returns a parse error.
- You have documented parser limitations.

