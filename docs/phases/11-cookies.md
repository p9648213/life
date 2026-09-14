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

## Cookie Input Behavior

Use this policy for the simple cookie subset in this phase:

| Input situation | Expected behavior |
|---|---|
| Missing `Cookie` header | No cookies. |
| `theme=dark` | One pair: `theme` with value `dark`. |
| `theme=dark; language=en` | Two pairs. |
| `token=abc==` | Split on the first `=`; preserve `abc==` as the value. |
| Spaces or tabs around a pair | Trim them before parsing the pair. |
| `theme=` | Accept an empty value. |
| Missing `=`, such as `theme` | Return a parsing error. |
| Empty name, such as `=dark` | Return a parsing error. |
| Duplicate name, such as `theme=dark; theme=light` | Return a parsing error. |
| Empty segment, such as a trailing `;` | Ignore it. |
| `Theme=dark; theme=light` | Treat the names as distinct; cookie names are case-sensitive. |
| Multiple `Cookie` header lines | Reject before header storage can overwrite an earlier line. |

Keep values literal: do not apply form decoding to `+` or percent escapes. Define and document the accepted name/value character subset and reject unsupported characters. These malformed-input and duplicate policies are project choices for this phase.

## Outgoing Cookie Attributes

These are attributes of `Set-Cookie`, not additional cookies. For example:

```http
Set-Cookie: theme=dark; Path=/; HttpOnly; SameSite=Lax
```

The browser later sends `Cookie: theme=dark`; it does not return the attributes. Your server constructs them; the browser enforces their behavior.

| Attribute | Meaning and example |
|---|---|
| `Path` | Limits eligible request paths. `/` covers all paths; `/account` matches `/account` and `/account/settings`, but not `/accounting`. If omitted, the browser derives a default from the setting request's path. This is not an authorization boundary. |
| `Domain` | Omit to restrict the cookie to the setting host. `Domain=example.com` also permits its subdomains; an unrelated domain is rejected. |
| `HttpOnly` | A flag without `=true`. Prevents JavaScript from reading the cookie through `document.cookie`; the browser can still attach it to requests initiated by JavaScript. |
| `Secure` | A flag restricting transmission to secure connections, normally HTTPS. It does not encrypt the stored value. |
| `Max-Age` | Lifetime in seconds: `Max-Age=3600` means one hour; zero or negative expires it immediately. |
| `Expires` | An absolute HTTP date. `Max-Age` takes precedence when both are present. |

These attribute semantics come from [RFC 6265](https://httpwg.org/specs/rfc6265.html#section-4.1.2).

### SameSite

`SameSite` controls sending cookies in cross-site contexts. Site is not the same as origin: the scheme and registrable domain matter, so two HTTPS subdomains of `example.com` can be same-site.

| Value | Behavior |
|---|---|
| `Strict` | Send only in same-site contexts; an initial navigation from another site does not include the cookie. |
| `Lax` | Also permits cross-site top-level navigations using safe methods, such as following a normal GET link. Cross-site POST forms, embedded images, and fetch requests do not get this exception. |
| `None` | Removes the SameSite restriction, but requires `Secure`. Other browser cookie restrictions still apply. |

Set `Lax` explicitly for this phase. SameSite provides partial CSRF protection; it is not a complete defense. See the [HTTP working group's cookie draft](https://httpwg.org/http-extensions/draft-ietf-httpbis-rfc6265bis.html#section-4.1.2.7) (work in progress).

### Construction Policy for This Phase

- Start with `Path=/`, `HttpOnly`, and `SameSite=Lax`; enable `Secure` for HTTPS. Use an explicit local HTTP configuration for the demo.
- Omit `Domain`, `Max-Age`, and `Expires` initially. Without expiration attributes, this is a browser-session cookie; browser session restoration can preserve it. This does not create server-side session state.
- Treat custom domains and absolute expiration dates as optional extensions. If you add expiration, start with `Max-Age`; deletion must target the original name, domain scope, and path. See [cookie storage and replacement rules](https://httpwg.org/http-extensions/draft-ietf-httpbis-rfc6265bis.html#section-4.1.2).
- Validate all fields before modifying the response. Reject CR/LF and unsupported characters, including semicolons that could inject another attribute. For this subset, require an explicit path to begin with `/` and reject unsupported SameSite values or `SameSite=None` without `Secure`.
- Emit enabled flags by name and omit disabled flags. Append one `Set-Cookie` header per cookie; do not combine cookies into a comma-separated header.
- Choose the Rust representation and constructor API yourself. Keep cookie formatting in the HTTP layer and application-specific values in handlers.

## Tests to Write

- one and multiple cookies parse correctly;
- values containing `=` follow the documented behavior;
- whitespace, malformed pairs, and duplicates follow policy;
- `Set-Cookie` serializes required attributes;
- enabled flags serialize without `=true`, and disabled flags are omitted;
- custom paths and supported SameSite modes serialize correctly;
- invalid attribute values and `SameSite=None` without `Secure` are rejected before changing the response;
- multiple outgoing cookies produce separate `Set-Cookie` headers;
- if expiration is implemented, `Max-Age` serializes seconds and deletion preserves the original scope;
- header injection is rejected.

Use browser developer tools to check path matching, HttpOnly visibility, and SameSite behavior. Unit tests verify serialization; browser checks verify when the browser stores and sends cookies.

## Checkpoint

You are done when the server can safely set and parse the documented cookie subset and you can explain its limitations.

After this, continue with [Phase 12: Sessions](12-sessions.md).
