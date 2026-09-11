# Phase 10: Static Files

Goal: serve static assets with correct HTTP metadata and safe path handling.

Design the static-file boundary yourself.

## Expected Behavior

Clients can request files under `/static/`, including nested paths, and receive their exact bytes with the correct HTTP metadata. CSS, JavaScript, images, fonts, and other static files share the same serving behavior; CSS is one example.

## Requirements

- Serve regular files beneath a designated static root through `GET /static/<path>` without adding a route for each file.
- Preserve file bytes, including binary data; do not require UTF-8 or render files as templates.
- Set `Content-Length` to the body length in bytes. Define a small extension-to-content-type mapping for CSS, JavaScript, common images, and fonts; use `application/octet-stream` for unknown extensions. Only add a charset for appropriate text types.
- Use the URL path for file lookup, excluding the query string, so `/static/app.css?v=1` serves the same file as `/static/app.css`.
- Return `404 Not Found` for missing files and directories; do not list directories or select index files automatically. Unsupported methods must not serve file contents.
- Confine file access to the static root. Define URL-decoding and normalization rules, and reject traversal, absolute paths, malformed encodings, and platform-specific escape forms. Encoded paths and symbolic links must not allow access outside the root.
- Set an explicit file-size limit and document the error response for oversized files and other I/O failures. Enforce the limit while reading, even if a file grows after its metadata is checked.
- Keep path processing linear in path length and file handling linear in bytes served, with memory bounded by the configured limits. Avoid repeated rescanning or copying of growing buffers.
- Keep static-file handling separate from application state.

Caching, conditional requests, range requests, compression, and streaming large files are outside this phase.

## Tests to Write

- CSS, JavaScript, an image, and a font return exact bytes, expected content types, and correct byte lengths;
- binary data containing invalid UTF-8 and zero bytes is preserved;
- an unknown extension returns `application/octet-stream`;
- nested paths work, and query strings do not change file lookup;
- missing assets and directories return `404`;
- unsupported methods do not serve the file;
- plain and encoded traversal, absolute paths, malformed encodings, and symlink escapes cannot read outside the static root;
- empty files, files exactly at the size limit, and oversized files follow the documented behavior;
- maximum permitted paths and files complete with bounded work and memory; investigate unexpectedly slow boundary tests.

## Checkpoint

You are done when a page loads CSS, JavaScript, and an image from your server, binary-file tests pass, and user-controlled paths cannot escape the static root. Adding another file beneath that root must not require a new route.

After this, continue with [Phase 11: Cookies](11-cookies.md).
