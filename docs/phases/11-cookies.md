# Phase 11: Cookies

Goal: understand browser-stored state carried through HTTP headers.

Design the cookie representation and parsing API yourself.

## Expected Behavior

The server can send a `Set-Cookie` header, the browser returns the cookie in a later `Cookie` request header, and a handler can read a simple cookie value.

## Requirements

- Support simple `name=value` pairs separated by semicolons.
- Trim optional whitespace around pairs.
- Split each pair on the first `=`.
- Define behavior for malformed and duplicate cookie names.
- Set appropriate attributes such as `HttpOnly` and `SameSite=Lax`.
- Add `Secure` when cookies travel over HTTPS.
- Reject CR/LF in response header values.
- Bound parser work by the existing header limits.
- Do not treat a client-provided cookie as proof of identity.

## Tests to Write

- one and multiple cookies parse correctly;
- values containing `=` follow the documented behavior;
- whitespace, malformed pairs, and duplicates follow policy;
- `Set-Cookie` serializes required attributes;
- header injection is rejected.

## Checkpoint

You are done when the server can safely set and parse the documented cookie subset and you can explain its limitations.

After this, continue with [Phase 12: Sessions](12-sessions.md).
