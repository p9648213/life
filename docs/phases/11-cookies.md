# Phase 11: Cookies

Goal: understand browser-stored state sent by HTTP headers.

Cookies are not magic. The server sends `Set-Cookie`; the browser later sends `Cookie`.

## What to Learn

- `Set-Cookie` response header
- `Cookie` request header
- Cookie attributes
- `HttpOnly`
- `SameSite`
- Expiration

## Where to Look

- MDN cookies: https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies
- MDN Set-Cookie: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
- MDN Cookie: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cookie

## Step-by-Step Work

1. Add a response header:

```text
Set-Cookie: visited=yes; HttpOnly; SameSite=Lax
```

2. Reload the page in a browser.
3. Inspect request headers and find `Cookie`.
4. Add cookie parsing for simple `name=value` pairs.
5. Show different content when `visited=yes` exists.

## Cookie Parser Scope

Support simple cookies:

```text
Cookie: visited=yes; theme=light
```

You can split on `;`, trim spaces, then split each pair on the first `=`.

Document limitations.

## Questions to Answer

- What is the difference between `Set-Cookie` and `Cookie`?
- Where is the cookie stored?
- Why is `HttpOnly` useful?
- Why does a cookie not prove identity by itself?

## Checkpoint

You are done when:

- Server can set a cookie.
- Browser sends it on later requests.
- Server can parse it.

