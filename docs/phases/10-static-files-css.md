# Phase 10: Static Files and CSS

Goal: serve a small CSS file yourself.

This teaches file serving, MIME types, and path safety.

## What to Learn

- Static assets
- MIME types
- Path traversal
- Cache headers later

## Where to Look

- MDN MIME types: https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types
- Rust paths: https://doc.rust-lang.org/std/path/
- Rust fs: https://doc.rust-lang.org/std/fs/

## Step-by-Step Work

1. Create `static/app.css`.
2. Add a route for `GET /static/app.css`.
3. Read the file bytes.
4. Return `Content-Type: text/css; charset=utf-8`.
5. Link the stylesheet in your HTML layout.
6. Return 404 if the file is missing.

## Safety Rule

Do not allow arbitrary paths yet.

Avoid supporting this early:

```text
GET /static/anything-the-user-types
```

If you later support multiple files:

- Reject `..`.
- Normalize paths carefully.
- Only serve files from the static directory.

## Questions to Answer

- Why does the browser need the CSS content type?
- What is path traversal?
- Why is serving one known CSS file safer than serving arbitrary paths?

## Checkpoint

You are done when:

- Browser loads your CSS from your server.
- Missing static files return 404.

