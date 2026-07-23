# Phase 10: Static Files and CSS

Goal: serve static assets with correct HTTP metadata and safe path handling.

Design the static-file boundary yourself.

## Expected Behavior

The browser can request a known CSS file from the server, receive its bytes with the correct content type, and use it in rendered HTML.

## Requirements

- Support `GET /static/app.css`.
- Return `Content-Type: text/css; charset=utf-8`.
- Return `404 Not Found` when the file is missing.
- Do not accept arbitrary filesystem paths in this phase.
- If generalized later, confine normalized paths to the static root and reject traversal, absolute paths, and platform-specific escape forms.
- Bound the size of files read into memory.
- Keep static-file handling separate from application state.

## Tests to Write

- the known CSS route returns exact bytes and content type;
- a missing asset returns `404`;
- unsupported methods do not serve the file;
- traversal-like paths cannot read outside the static root;
- oversized files follow the documented limit.

## Checkpoint

You are done when the page loads CSS from your server and user-controlled paths cannot escape the intended static-file boundary.

After this, continue with [Phase 11: Cookies](11-cookies.md).
