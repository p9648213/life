# Phase 02: HTTP Response Builder

Goal: stop hand-writing full HTTP responses in handlers.

In phase 1, a raw response string is fine. Soon you will return many responses: success, redirects, errors, HTML pages, CSS, and maybe JSON. A small response builder keeps the rules in one place.

## What to Learn

- HTTP status line
- Headers
- Body
- Byte length versus character count
- Converting structured data into bytes

## Where to Look

- MDN HTTP messages: https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages
- MDN status codes: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
- Rust `String`: https://doc.rust-lang.org/std/string/struct.String.html
- Rust `Vec`: https://doc.rust-lang.org/std/vec/struct.Vec.html

## Target Shape

You want a small type that can represent:

```text
status code: 200
reason phrase: OK
headers:
  Content-Type: text/html; charset=utf-8
body:
  <h1>Hello</h1>
```

Then it should produce bytes like:

```text
HTTP/1.1 200 OK\r\n
Content-Type: text/html; charset=utf-8\r\n
Content-Length: 14\r\n
\r\n
<h1>Hello</h1>
```

## Design Questions Before Coding

Answer these first:

- Should the body be stored as `String` or `Vec<u8>`?
    + I think the body should be stored as `Vec<u8>`, because HTTP response bodies are bytes and may contain text, HTML, images, or other binary data.
- Should headers be a `Vec<(String, String)>` or a map?
    + I think headers should be a `Vec<(String, String)>` for now. It is simple to serialize and lets me see how headers are written on the wire. A map may be useful later for lookup or replacement.
- Should `Content-Length` be inserted automatically?
    + Yes. `Content-Length` should be inserted automatically during serialization, using the final body length in bytes.
- What default content type should HTML use?
    + I think HTML responses should default to `text/html; charset=utf-8`.

For learning, a `Vec` of header pairs is easier to understand than a map.

## Step-by-Step Work

1. Create a response type in `main.rs` or a new `response.rs` file.
2. Give it fields for status, headers, and body.
3. Add a constructor for a simple HTML response.
4. Add a method that serializes the response into bytes.
5. Make that method add `Content-Length`.
6. Replace your phase 1 raw response with this type.

## Tiny Pseudocode Shape

```text
Response:
  status_code
  reason_phrase
  headers
  body_bytes

method to_bytes:
  start with "HTTP/1.1 {code} {reason}\r\n"
  append each header
  append Content-Length using body byte length
  append blank line
  append body bytes
```

## Experiments

Try returning:

- `200 OK` with plain text
- `200 OK` with HTML
- `404 Not Found` with HTML

Use:

```bash
curl -i http://127.0.0.1:8080/
```

Look carefully at the headers.

## Common Problems

Problem: browser shows only part of the page.

Likely cause:

- `Content-Length` is too small.

Problem: browser waits or behaves strangely.

Likely cause:

- `Content-Length` is too large.

Problem: non-ASCII text breaks length.

Likely cause:

- You counted characters, not bytes.

Use byte length for HTTP body length.

## Questions to Answer

- Why should response serialization be centralized?
    + Response serialization should be centralized because HTTP responses have repeated wire-format rules: the status line, `\r\n` line endings, headers, the blank line before the body, and the body byte length. If each handler builds these by hand, it is easy for responses to become inconsistent or invalid.
- What is the difference between status code and reason phrase?
    + The status code is the machine-readable number that tells the client what happened, such as `200` or `404`. The reason phrase is the human-readable text for that status, such as `OK` or `Not Found`.
- Why should body length be calculated after encoding?
    + Body length should be calculated after encoding because HTTP `Content-Length` counts bytes, not characters. Text like UTF-8 may use more than one byte per character, and future responses may contain binary data.

## Checkpoint

You are done when:

- Your server returns responses through your response type.
- `curl -i` shows valid headers.
- You can return at least `200` and `404`.
