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
- Should headers be a `Vec<(String, String)>` or a map?
- Should `Content-Length` be inserted automatically?
- What default content type should HTML use?

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
- What is the difference between status code and reason phrase?
- Why should body length be calculated after encoding?

## Checkpoint

You are done when:

- Your server returns responses through your response type.
- `curl -i` shows valid headers.
- You can return at least `200` and `404`.

