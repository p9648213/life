# Phase 14: Error Handling

Goal: make failure explicit.

Early code often uses `unwrap` or `expect`. That is fine for exploration, but your app should eventually convert failures into clear HTTP responses.

## What to Learn

- `Result`
- Custom error enums
- Client errors versus server errors
- Logging
- Error-to-response conversion

## Where to Look

- Rust error handling: https://doc.rust-lang.org/book/ch09-00-error-handling.html
- `Result`: https://doc.rust-lang.org/std/result/
- MDN HTTP status codes: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status

## Step-by-Step Work

1. List the failure cases in your app.
2. Group them by area:
   - parsing
   - routing
   - forms
   - storage
   - sessions
3. Decide which ones are client errors.
4. Decide which ones are server errors.
5. Convert errors into HTTP responses.
6. Log internal details server-side.
7. Show safe messages to users.
8. Keep unsupported body formats distinct from malformed data in a supported format. For example, a form endpoint should map a missing or unsupported `Content-Type` to `415 Unsupported Media Type`, while malformed `application/x-www-form-urlencoded` data remains a `400 Bad Request`.

## Suggested Mapping

```text
Malformed request       -> 400
Invalid form            -> 400
Unsupported media type  -> 415
Missing page            -> 404
Wrong method            -> 405
Body too large          -> 413
Storage failure         -> 500
Unexpected bug          -> 500
```

## Questions to Answer

- Which errors are caused by bad client input?
- Which errors are caused by your server?
- What information should never be shown in the browser?

## Checkpoint

You are done when:

- Invalid input does not panic.
- Storage failures are not reported as success.
- Error responses use reasonable status codes.
- Tests distinguish an unsupported request `Content-Type` (`415`) from malformed data in a supported format (`400`).
