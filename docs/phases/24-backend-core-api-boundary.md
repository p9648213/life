# Phase 24: Backend Core API Boundary

Goal: make application code depend on a small, explicit backend-core API.

Design the final module and public API structure yourself.

## Expected Behavior

A small application can register routes, receive requests and application context, and return responses without changing TCP, parsing, or serialization code.

## Requirements

- Make dependency direction point from application code toward backend core.
- Keep `TcpStream` out of router and application handlers.
- Keep application records, storage choices, and business rules out of HTTP parsing and response modules.
- Expose only types and operations required by application code.
- Keep fields private when invariants require controlled mutation.
- Preserve explicit request flow; avoid framework-style hidden global registration or implicit extraction.
- Keep error boundaries and ownership understandable from function signatures.
- Do not generalize an API until at least one real application need justifies it.
- Remove or keep private obsolete phase-demo surfaces.

## Tests to Write

- a small example application can use only the public boundary;
- replacing the sample application does not alter TCP or parsing code;
- replacing storage does not alter request parsing or response serialization;
- handlers can be tested without sockets;
- private invariants cannot be bypassed through public fields;
- core modules do not import application modules.

## Checkpoint

You are done when the backend core has a small public surface, application-specific code is easy to identify, and control flow remains explicit.

After this, continue with [Phase 25: Security and Deployment Boundary](25-security-deployment-boundary.md).
