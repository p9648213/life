# Phase 10 Feedback

## Overall

Phase 10 is complete for the agreed learning scope: serve assets from a trusted static directory whose files and parent directories are not replaced while requests are being served. You are ready to continue with [Phase 11: Cookies](../phases/11-cookies.md).

The static-file boundary remains explicit:

```text
Request path, excluding query
  -> Router selects the static mount and checks the method
  -> validate the asset path
  -> resolve and check root confinement and regular-file metadata
  -> read with a byte limit
  -> Response serializes headers and exact body bytes
```

## What Works Well

- One `/static/` mount serves nested assets without registering each file.
- Static serving takes a root path and an asset path, without depending on application state or TCP sockets.
- Files are read as bytes, preserving invalid UTF-8, zero bytes, and template-like text.
- CSS, JavaScript, PNG, JPEG, SVG, and WOFF2 have explicit content types. Extension matching ignores case; unknown or absent extensions use `application/octet-stream`.
- The response serializer calculates `Content-Length` from the body bytes.
- Canonical paths are compared using path components, so a sibling such as `static-private` is not mistaken for a child of `static`.
- The standard library is sufficient for the current implementation; no dependencies were added.

## Current Policies And Review Fixes

Paths reject leading slashes, backslashes, colons, NUL bytes, and `.` or `..` segments. All percent signs are rejected, including valid encodings: this implementation deliberately does not URL-decode asset paths. Invalid paths return `400`.

Missing assets and directories return `404`, with no directory listing or automatic index selection. Review fixed two additional cases: `/static/` now returns `404`, and looking beneath a regular file, such as `/static/app.css/child.css`, maps `NotADirectory` to `404`.

POST requests to the mount return `405` without asset contents. Other currently unsupported methods are rejected by the request parser before routing.

`MAX_ASSET_SIZE` is 10 KiB. Empty files and files exactly at the limit are served. Oversized files return `500` with `Limit exceed`; other I/O failures return `500` with `Internal Server Error` and are logged. These are the current error policies, not a requirement to use the same wording in future phases.

## Verification

After the two mapping fixes:

- Phase 10 tests: 22 passed, 0 failed.
- `cargo test --workspace --no-fail-fast --quiet`: 175 passed, 0 failed. Two existing TCP tests required a rerun outside the sandbox to bind local sockets.
- Formatting checks for the changed Rust files and `git diff --check` passed.

The two new status regressions failed before the fixes and passed afterward. Coverage includes exact bytes and metadata, nested paths and queries, files added after mounting, method rejection, traversal and encoding rejection, Unix symlink behavior, size boundaries, and allocation checks.

The resource page references the extracted CSS files. A browser checkpoint loading CSS, JavaScript, and an image was not verified during this review; automated asset tests cover their serving behavior.

## Resource Limits And Deferred Work

Explicit path validation takes O(path length) time with no segment collection. Path construction and filesystem resolution also have costs; the review does not establish a platform-independent bound for filesystem internals. Reading and response serialization take O(bytes read or served), with asset-buffer memory O(size limit). Serialization copies the body, but does not introduce quadratic growth.

`take(MAX_ASSET_SIZE + 1)` enforces the byte cap during reading, even if a file grows. The extra byte distinguishes an exact-limit file from an oversized one. The sparse-file test checks the largest allocation, not total peak memory or exact I/O work. The 1 MiB request-path test exercises late traversal rejection and bounded allocation; it is not proof that every valid filesystem path completes within a particular runtime. No unexpectedly slow boundary test was observed.

Concurrent filesystem replacement is explicitly deferred. Canonicalization, metadata lookup, and opening are separate operations. Another program with write access could replace a checked file or parent directory with a symlink before opening, causing the server to open an outside file it can read. A remote request that only reads assets cannot perform that replacement by itself. Existing tests cover stable symlinks, not this race or concurrent file growth.

Keep the static tree and its parent directories trusted for this checkpoint. Revisit confinement during opening when introducing writable asset trees, uploads, or deployment hardening. Caching, conditional requests, compression, ranges, and large-file streaming remain outside Phase 10; performance changes should follow measurements.
