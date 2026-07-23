# Phase 14: Error Handling

Goal: turn expected failures into explicit, safe HTTP responses.

Design the error types and conversion boundaries yourself.

## Expected Behavior

Invalid client input produces an appropriate client-error response, internal failures produce a server-error response, and expected failures do not panic or get reported as success.

## Requirements

- Keep parsing, routing, form, storage, session, and application errors distinguishable where their meaning differs.
- Map malformed requests and supported-format validation failures to `400 Bad Request`.
- Map missing resources to `404 Not Found` and known routes with unsupported methods to `405 Method Not Allowed`.
- Map oversized bodies to `413 Payload Too Large`.
- Map missing or unsupported body media types to `415 Unsupported Media Type`.
- Keep malformed data in a supported media type as `400 Bad Request`.
- Map unexpected storage and internal failures to `500 Internal Server Error`.
- Log useful internal context server-side while returning short, safe client messages.
- Do not leak paths, credentials, session IDs, query secrets, or debug representations.

## Tests to Write

- invalid input does not panic;
- each documented failure maps to the correct status;
- `415` unsupported media type remains distinct from `400` malformed supported data;
- storage failures are not reported as success;
- wrong-method and missing-route behavior remain distinct;
- internal details do not appear in response bodies.

## Checkpoint

You are done when expected errors have consistent status mappings, internal failures are diagnosable server-side, and client responses do not expose sensitive details.

After this, continue with [Phase 15: Concurrency](15-concurrency.md).
