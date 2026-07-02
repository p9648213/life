# Phase 06: Form Parsing

Goal: accept browser form submissions.

HTML forms usually send data as `application/x-www-form-urlencoded`. You will parse a small subset yourself.

## What to Learn

- Form method and action
- `application/x-www-form-urlencoded`
- `key=value&key2=value2`
- Percent decoding
- `+` as space
- Server-side validation

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
2. Submit the form and print the raw request body.
3. Confirm the content type.
4. Split body pairs on `&`.
5. Split each pair on the first `=`.
6. Decode `+` into a space.
7. Decode `%XX` hex bytes.
8. Validate required fields.
9. Return an error response for invalid form data.

## Tiny Pseudocode Shape

```text
parse_form(body):
  for each pair separated by "&":
    split pair into key and value
    decode key
    decode value
    store key/value
```

## Common Problems

Problem: spaces appear as `+`.

Cause:

- URL-encoded forms use `+` to represent a space.

Problem: special characters look like `%3C`.

Cause:

- Percent encoding stores bytes as hex.

Problem: duplicate field names.

Decision for now:

- Either keep the first value or the last value.
- Document your choice.

## Experiments

```bash
curl -i -X POST http://127.0.0.1:8080/demo/form \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "name=Rust&message=Learning%20HTTP"
```

## Questions to Answer

- Why does the server validate input even if the HTML form says a field is required?
- Why must percent decoding work on bytes?
- What should happen if a required field is empty?

## Checkpoint

You are done when:

- The browser can submit a form.
- Your server extracts fields from the form body.
- Invalid forms produce a useful error.
