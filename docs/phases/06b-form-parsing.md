# Phase 06B: Form Parsing

Goal: decode and validate browser form submissions after Phase 06A has delivered a complete raw request body.

HTML forms usually send data as `application/x-www-form-urlencoded`. You will parse a small subset yourself.

## Prerequisite and Boundary

Complete [Phase 06A: Minimal Request Body Accumulation](06a-minimal-request-body-accumulation.md) first.

Phase 06B begins with bytes already available through `request.body()`. It must not read from `TcpStream`, search for HTTP header boundaries, or interpret `Content-Length`. Those are connection and HTTP-message concerns, not form-format concerns.

Keep the flow explicit:

```text
complete Request
  -> verify form Content-Type
  -> parse request.body()
  -> validate decoded fields
  -> handler makes an application decision
```

## What to Learn

- Form method and action
- `application/x-www-form-urlencoded`
- `key=value&key2=value2`
- Percent decoding
- `+` as space
- Duplicate-field policy
- Server-side validation
- The boundary between decoding and application decisions

## Where to Look

- MDN forms: https://developer.mozilla.org/en-US/docs/Learn/Forms
- MDN POST method: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST
- URL percent encoding: https://developer.mozilla.org/en-US/docs/Glossary/Percent-encoding

## Form Shape

Your page should contain a form conceptually like:

```html
<form method="post" action="/demo/form">
  <input name="name">
  <textarea name="message"></textarea>
  <button type="submit">Send</button>
</form>
```

## Step-by-Step Work

1. Add a `GET /demo/form` page with a form.
2. Submit it and confirm that the `POST /demo/form` handler receives the complete raw `request.body()` from Phase 06A.
3. Check that the request uses `application/x-www-form-urlencoded` before applying this decoder. For this phase, trim and compare the media-type portion before the first `;` case-insensitively; allow parameters to remain uninterpreted.
4. Give form decoding a small boundary separate from the route handler.
5. Split body pairs on `&`.
6. Split each pair on the first `=` so encoded or future values are not split unnecessarily.
7. Decode `+` into a space.
8. Decode each valid `%XX` sequence into one byte.
9. Decide how invalid percent escapes and invalid UTF-8 are reported.
10. Choose and document whether duplicate field names keep the first value, keep the last value, or preserve all values.
11. Validate required fields after decoding.
12. Return a useful client error for malformed encoding or invalid form data.

## Tiny Pseudocode Shape

```text
parse_form(body_bytes):
  for each pair separated by "&":
    split pair on the first "="
    decode raw "+" bytes as spaces
    percent-decode key bytes
    percent-decode value bytes
    apply the documented duplicate-field rule
  return decoded fields

handle_form(request):
  verify Content-Type
  parse request.body()
  validate required fields
  make the handler decision
```

Percent decoding should operate on bytes because `%XX` represents one encoded byte. Decide whether UTF-8 conversion happens after each field is decoded or at another explicit boundary.

Split the encoded body into pairs and key/value pieces before decoding delimiters. Within each piece, convert raw `+` bytes to spaces before percent decoding. This keeps `%2B` as a literal `+` instead of incorrectly converting it to a space a second time.

## Tests to Write

Keep most tests away from TCP. Phase 06A already tests fragmented request delivery; Phase 06B should focus on the pure form decoder and validation.

Useful cases:

- two normal fields
- empty value
- missing `=`
- raw `+` decoded as space while `%2B` remains `+`
- valid `%XX` sequences
- malformed `%` sequence
- percent-decoded multi-byte UTF-8 text
- empty key
- duplicate key according to your chosen rule
- required field missing
- required field present but empty
- non-form `Content-Type` rejected by the handler boundary
- case-insensitive form media type with a `charset` parameter accepted

## Common Problems

Problem: spaces appear as `+`.

Cause:

- URL-encoded forms use `+` to represent a space.

Problem: special characters look like `%3C`.

Cause:

- Percent encoding stores bytes as hexadecimal pairs.

Problem: malformed `%` input panics or silently changes data.

Cause:

- The decoder assumes two valid hexadecimal digits always follow `%`. Treat client-controlled encoding as fallible input.

Problem: duplicate field names behave unpredictably.

Cause:

- The representation has no explicit duplicate policy. Choose one and test it.

Problem: the form parser contains `TcpStream::read` calls.

Cause:

- Phase 06A and Phase 06B responsibilities were mixed. The form parser should receive only body bytes.

## Experiments

```bash
curl -i -X POST http://127.0.0.1:8080/demo/form \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "name=Rust&message=Learning%20HTTP"
```

Try spaces, Unicode, reserved characters, empty fields, duplicate keys, and malformed percent escapes. Compare the raw body with the decoded representation.

## Questions to Answer

- Why does the server validate input even if the HTML form says a field is required?
- Why must percent decoding work on bytes?
- What should happen if a required field is empty?
- Why should transport accumulation and form decoding remain separate?
- What duplicate-field behavior did you choose, and why?
- Which errors belong to malformed encoding and which belong to validation?

## Checkpoint

You are done when:

- The browser can submit a form through the Phase 06A request reader.
- The form decoder accepts raw body bytes rather than reading from the socket.
- `+` and valid `%XX` sequences decode according to the documented rules.
- Duplicate fields behave according to one tested policy.
- The server extracts and validates the required fields.
- Malformed encodings and invalid forms produce useful errors.

## Continue

After form decoding and validation work, continue with [Phase 07: In-Memory State](07-in-memory-state.md).
