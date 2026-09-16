# Phase 11 Feedback

## Overall

Phase 11 is complete for the [agreed cookie subset](../phases/11-cookies.md). You are ready for [Phase 12: Sessions](../phases/12-sessions.md). This is a learning checkpoint, not a production-readiness assessment.

## What Works Well

- `Cookie` represents one outgoing cookie. Name/value, domain, and path setters validate before mutation and return errors without changing existing state.
- `parse_cookie()` returns borrowed name/value pairs and preserves duplicates. `Request::extract_cookie()` exposes parsing errors without coupling the parser to handlers or TCP.
- Parsing splits on the first `=`, accepts empty values, preserves literal data, and rejects malformed pairs and empty segments.
- Both header extraction and cookie parsing trim only ASCII spaces/tabs. Review caught broad `.trim()` removing unsupported whitespace before validation; the new regressions cover that fix.
- Serialization emits separate `Set-Cookie` headers, explicit default attributes, and flags without `=true`. `Max-Age` distinguishes unset, zero, and positive seconds while preserving scope.
- The implementation uses the standard library without new dependencies.

## Agreed Policies

- Duplicate names within one header are preserved in received order. That order does not establish which value a consumer should use.
- For repeated `Cookie` header lines, the last header value wins case-insensitively; earlier values are discarded rather than merged.
- Uninitialized cookies may serialize. Callers must set a valid name/value before sending; setters and incoming parsing still reject empty names.
- `SameSite=None` preserves the caller's `Secure` choice. Successful serialization does not guarantee browser acceptance.

These are deliberate project policies. Tests requiring duplicate rejection, repeated-header rejection, empty-segment tolerance, or uninitialized-cookie rejection were removed or revised accordingly.

## Verification

- The final code review ran `cargo test`: all tests passed, including 19 cookie tests. The TCP tests needed execution outside the sandbox to bind local sockets.
- Whitespace regressions failed before the trimming fix and passed afterward through both the standalone parser and request integration.
- `git diff --check` passed during the final review.
- You reported the browser checkpoint passed. This confirmation came from your manual test; the reviewer did not independently observe the browser.

## Resource Limits And Next Steps

Cookie parsing takes O(n) time and O(k) extra memory for k pairs. Each byte is examined a bounded number of times; duplicate preservation requires no repeated searches. Strings borrow the input, but the vector still allocates storage for pairs.

A review probe with 1,048,574 bytes and 349,525 pairs took about 71 ms in a debug build. Vector capacity used 16 MiB on the review machine, excluding the input and other request allocations. This is a single observation, not a benchmark guarantee. Revisit total per-request memory when changing the 1 MiB request limit or adding concurrency; no quadratic parsing path was found.

For Phase 12, decide how session lookup handles multiple values for the session cookie name. Keep that decision in the session boundary. Use a proven cryptographically secure randomness source for opaque IDs, bound retained session state, and define expiration and logout behavior.

The temporary test adapter can be simplified once the public APIs settle. Further allocation tuning should follow measurement rather than delaying the next phase.
