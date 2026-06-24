# Phase 03 Feedback

## Overall

You completed the main goal of Phase 03: raw request bytes are now parsed into a structured `Request` containing a request line, headers, optional content length, and body. Malformed requests return an error and receive a `400 Bad Request` response instead of panicking the server.

I verified that:

- `cargo fmt --check` passes.
- All 17 focused request parser tests pass.
- Valid `GET` and `POST` requests parse correctly.
- `PUT`, `PATCH`, and `DELETE` are explicitly included in the documented subset.
- Header names can be retrieved case-insensitively.
- Header values containing `:` are preserved.
- `Content-Length` is parsed case-insensitively and checked against the exact body byte count.
- Bytes after the headers are rejected when no `Content-Length` is present.
- Malformed request lines, headers, versions, body lengths, UTF-8, and short inputs return errors.
- `handle_client` converts parsing failures into `400 Bad Request` responses.

## What You Did Well

- You used bounds-checked slice access while searching for `\r\n` and `\r\n\r\n`, avoiding out-of-bounds panics on short input.
- You separated `RequestLine`, `HttpMethod`, and `ContentLength` into meaningful types instead of leaving the parsed request as unrelated strings.
- You changed `Request::parse` to return `Result<Request, AppError>` and removed parsing `unwrap` calls.
- You validated that the request line contains exactly a method, target, and version.
- You require the exact supported version token, `HTTP/1.1`.
- You used `split_once(':')`, so colons inside header values are not discarded.
- You normalize header names and provide `get_header`, giving callers case-insensitive lookup behavior.
- You use `Content-Length` to validate the body byte count and reject both incomplete and surplus bodies.
- You reject unexpected trailing bytes when `Content-Length` is absent, keeping request boundaries unambiguous while pipelining is unsupported.
- You added a project error type and implemented `Display` without adding an error-handling dependency.
- You added focused success and failure tests instead of testing only the happy path.

## Things To Improve Later

- `handle_client` still performs one read into a 256-byte buffer. A valid request can arrive partially or exceed the buffer and be treated as malformed. This is an intentional limitation until Phase 16.
- The parser currently uses an empty header slice to detect several malformed conditions. Later, separating “separator not found,” “headers incomplete,” and “header invalid” would produce more precise errors.
- Duplicate `Content-Length` headers are not rejected. If different components choose different values, request boundaries can become ambiguous. Rejecting duplicates is useful future hardening.
- A `HashMap` keeps only one value for duplicate header names. Some headers may legally appear more than once, so a later parser may need a different representation.
- HTTP/1.1 normally requires a `Host` header. The current parser stores headers but does not validate that requirement, which is acceptable for this learning subset.
- Some parsed fields and status variants are not used outside tests yet. Routing and later phases will begin consuming them.
- `AppError::RequestHeaderInvalid` currently covers several different failures. More specific variants can be introduced when callers need to react differently.

## Answer Review

Your final answers capture the important Phase 03 concepts:

- The first line is the request line, or start-line, and contains the method, request target, and HTTP version.
- A blank line encoded as `\r\n\r\n` separates the headers from the body.
- Malformed syntax should produce `400 Bad Request` because the server cannot safely interpret the request.
- One `read` is insufficient in a real server because TCP is a byte stream and one request may arrive in multiple pieces.

## Ready For Phase 04

You are ready to move on to routing. The main idea to carry forward is that routing should consume the structured method and target produced by the parser rather than inspect raw TCP bytes again.
