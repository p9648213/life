# Phase 11: Cookies

Goal: understand browser-stored state carried through HTTP headers.

Design the cookie representation and parsing API yourself.

## Expected Behavior

The server sends a `Set-Cookie` header. The browser returns name/value pairs in a later `Cookie` request header, and a handler can read them.

## Requirements

- Split semicolon-separated pairs on the first `=`. Preserve later `=`, literal `+`, and percent escapes.
- Trim only ASCII spaces/tabs around names and values, including during header extraction.
- Allow empty values; reject empty names, missing `=`, empty segments, and unsupported characters.
- Keep names case-sensitive and preserve duplicate pairs in received order; order does not determine precedence.
- A missing `Cookie` header means no cookies. For repeated header lines, the last value wins case-insensitively; earlier values are discarded, not merged.
- Keep parsing O(n), with memory bounded by request limits. Do not treat cookies as proof of identity.
- Default to `Path=/`, `HttpOnly`, and `SameSite=Lax`; enable `Secure` for HTTPS. Omit `Domain` and expiration by default.
- Validate setter input before mutation, including names, values, domains, and paths. Reject CR/LF and attribute injection; paths must start with `/`.
- Emit separate `Set-Cookie` headers. Enabled flags appear by name; disabled flags are omitted.
- Support `SameSite=Lax`, `Strict`, and `None`. Preserve the caller's `Secure` choice even with `None`; browser acceptance requires `Secure` for `None`.
- Serialize `Max-Age` in seconds; zero expires the cookie. Deletion must match its name, path, and domain scope. Absolute `Expires` support is optional.
- Uninitialized cookies may serialize without error; setting a valid name/value is the caller's responsibility. Setters and incoming parsing still reject empty names.

## Tests to Write

- missing, single, and multiple cookies;
- empty values, case-sensitive names, literal values, and preserved duplicates;
- allowed spaces/tabs and rejected unsupported whitespace through both parsing layers;
- malformed pairs and empty segments return errors;
- invalid setter input leaves the cookie unchanged;
- separate headers, default attributes, optional flags, and all SameSite modes serialize correctly;
- expiration distinguishes unset, zero, and positive seconds while preserving scope.

## Checkpoint

Tests pass for the agreed subset. Verify in a browser that cookies are stored, returned, and expired; check path matching, HttpOnly, and SameSite behavior. Explain the distinction between server serialization and browser acceptance.

After this, continue with [Phase 12: Sessions](12-sessions.md).
