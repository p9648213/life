# Phase 16: Better HTTP Behavior

Goal: harden request framing and connection behavior without attempting all of HTTP.

Design the reader state and connection policy yourself.

## Expected Behavior

Fragmented valid requests within configured limits still work. Oversized, ambiguous, stalled, or unsupported framing is rejected explicitly without excessive reads, rescanning, allocation, or panics.

## Requirements

- Enforce separate header, body, and total-request limits.
- Reject an oversized body from `Content-Length` before deliberately reading more body bytes.
- Once total length is known, bound each read by the remaining request bytes.
- Append only bytes actually returned by `read`.
- Choose one duplicate `Content-Length` policy; rejecting all duplicates is acceptable.
- Make the framing reader and `Request::parse` enforce the same policy.
- Reject every unsupported `Transfer-Encoding`.
- Reject requests containing both `Transfer-Encoding` and `Content-Length`.
- Define read timeouts for slow or stalled clients.
- Choose and document one connection policy:
  - close after one response, advertise `Connection: close`, and deliberately discard surplus; or
  - preserve surplus bytes and process the next request.
- Do not mistake bytes after the first request boundary for part of its body.
- Ensure request accumulation and parsing examine each permitted byte only a bounded number of times.

## Tests to Write

- fragmented requests within limits succeed;
- oversized headers and bodies fail promptly;
- a short final read succeeds;
- the final bounded read leaves later bytes unread when possible;
- surplus already received before the boundary was known follows the connection policy;
- duplicate or conflicting framing headers are rejected;
- unsupported transfer coding is rejected;
- stalled-client behavior follows the timeout policy;
- maximum-size deterministic tests complete in time consistent with linear work.

## Checkpoint

You are done when framing rules, limits, timeouts, and surplus-byte behavior are explicit, consistent across layers, and covered by boundary tests.

After this, continue with [Phase 17: Testing](17-testing.md).
