# Fullstack From Scratch in Rust

This documentation is a learning path for building a small fullstack web app from low-level pieces. It intentionally avoids full code solutions. Each phase tells you what to learn, where to look, what to try, and how to check your understanding.

Recommended app: a small notes app.

Final target:

- HTTP/1.1 server using `std::net`
- Manual request parsing
- Manual response generation
- Basic router
- Server-rendered HTML
- Browser forms
- In-memory state
- File-backed storage
- Cookies and sessions
- Basic authentication
- Small concurrency model
- Tests for the parts you build yourself

## How to Use These Docs

For each phase:

1. Read the phase file.
2. Open the linked Rust documentation.
3. Write a tiny experiment before integrating it into the app.
4. Implement the smallest useful version.
5. Test manually with `curl` or the browser.
6. Write down what you learned before moving on.

Do not copy large finished implementations from tutorials. If you use outside material, use it to answer one narrow question, then return to your own code.

## Reading Sources

Useful official references:

- Rust Book: https://doc.rust-lang.org/book/
- Rust standard library: https://doc.rust-lang.org/std/
- `std::net`: https://doc.rust-lang.org/std/net/
- `TcpListener`: https://doc.rust-lang.org/std/net/struct.TcpListener.html
- `TcpStream`: https://doc.rust-lang.org/std/net/struct.TcpStream.html
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- MDN HTTP overview: https://developer.mozilla.org/en-US/docs/Web/HTTP/Overview
- MDN HTTP messages: https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages
- MDN forms: https://developer.mozilla.org/en-US/docs/Learn/Forms
- MDN cookies: https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies

## Phase Index

1. [Project Setup](phases/00-project-setup.md)
2. [TCP Server](phases/01-tcp-server.md)
3. [HTTP Response Builder](phases/02-http-response-builder.md)
4. [HTTP Request Parser](phases/03-http-request-parser.md)
5. [Routing](phases/04-routing.md)
6. [HTML Rendering](phases/05-html-rendering.md)
7. [Form Parsing](phases/06-form-parsing.md)
8. [In-Memory State](phases/07-in-memory-state.md)
9. [Redirects](phases/08-redirects.md)
10. [File-Backed Storage](phases/09-file-backed-storage.md)
11. [Static Files and CSS](phases/10-static-files-css.md)
12. [Cookies](phases/11-cookies.md)
13. [Sessions](phases/12-sessions.md)
14. [Passwords and Authentication](phases/13-passwords-authentication.md)
15. [Error Handling](phases/14-error-handling.md)
16. [Concurrency](phases/15-concurrency.md)
17. [Better HTTP Behavior](phases/16-better-http-behavior.md)
18. [Testing](phases/17-testing.md)
19. [Optional Database Layer](phases/18-database-layer.md)
20. [Optional JSON API](phases/19-json-api.md)
21. [Optional Frontend Interactivity](phases/20-frontend-interactivity.md)

## Manual Test Commands

You will use these often:

```bash
cargo run
cargo test
curl -v http://127.0.0.1:8080/
curl -i http://127.0.0.1:8080/
curl -i -X POST http://127.0.0.1:8080/notes -d "title=Hello&body=World"
```

