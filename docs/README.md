# Custom Backend From Scratch in Rust

This documentation is a learning and engineering path for building a custom Rust web backend from low-level pieces. It intentionally avoids full code solutions. Each phase tells you what to learn, where to look, what to try, and how to check your understanding.

The old roadmap used a small app as the center of the work. The new direction is different: build a backend core that can support whatever fullstack application you decide to build separately. Temporary sample routes are allowed only to exercise the backend.

Current status:

- Phases 00 through 03 are complete.
- The next implementation phase is Phase 04: routing.
- The current code has a TCP accept loop, a response builder, a request parser, and focused parser/response tests.
- The next design move is to split routing and handler behavior away from TCP I/O.

Main target:

- HTTP/1.1 server using `std::net`
- Manual request parsing
- Manual response generation
- Router and handler boundary
- Application state boundary
- HTML, form, and JSON adapters when needed
- Cookies and sessions
- File-backed storage and database-backed storage
- Explicit error mapping
- Request limits and protocol hardening
- Configuration, diagnostics, and observability
- Concurrency model you understand
- Authentication using proven password hashing
- Tests for the parts you build yourself
- Benchmarking and profiling before performance rewrites

Non-goals:

- Do not clone every feature of Axum, Actix, Hyper, or a browser-facing reverse proxy.
- Do not write homemade cryptography, password hashing, or TLS.
- Do not optimize by guessing. First make behavior correct and measurable.
- Do not treat a local learning server as internet-safe until the security and deployment boundary phase says what is still missing.

For the deeper plan and current implementation feedback, read [backend-engine-plan.md](backend-engine-plan.md).

## How to Use These Docs

For each phase:

1. Read the phase file.
2. Open the linked Rust documentation.
3. Write a tiny experiment before integrating it into the backend core.
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
6. [HTML Rendering and Template Compiler Foundation](phases/05-html-rendering.md)
7. [Template Compiler Expansion](phases/05b-template-compiler-expansion.md)
8. [Form Parsing](phases/06-form-parsing.md)
9. [In-Memory State](phases/07-in-memory-state.md)
10. [Redirects](phases/08-redirects.md)
11. [File-Backed Storage](phases/09-file-backed-storage.md)
12. [Static Files and CSS](phases/10-static-files-css.md)
13. [Cookies](phases/11-cookies.md)
14. [Sessions](phases/12-sessions.md)
15. [Passwords and Authentication](phases/13-passwords-authentication.md)
16. [Error Handling](phases/14-error-handling.md)
17. [Concurrency](phases/15-concurrency.md)
18. [Better HTTP Behavior](phases/16-better-http-behavior.md)
19. [Testing](phases/17-testing.md)
20. [Database Layer](phases/18-database-layer.md)
21. [JSON API](phases/19-json-api.md)
22. [Frontend Interactivity Adapter](phases/20-frontend-interactivity.md)
23. [Configuration and Runtime Limits](phases/21-configuration-runtime-limits.md)
24. [Observability and Diagnostics](phases/22-observability-diagnostics.md)
25. [Benchmarking and Profiling](phases/23-benchmarking-profiling.md)
26. [Backend Core API Boundary](phases/24-backend-core-api-boundary.md)
27. [Security and Deployment Boundary](phases/25-security-deployment-boundary.md)

## Feedback Index

- [Current Backend Feedback](feedback/current-backend-feedback.md)
- [Phase 01 Feedback](feedback/phase-01-feedback.md)
- [Phase 02 Feedback](feedback/phase-02-feedback.md)
- [Phase 03 Feedback](feedback/phase-03-feedback.md)

## Manual Test Commands

You will use these often:

```bash
cargo run
cargo fmt --check
cargo test
curl -v http://127.0.0.1:8080/
curl -i http://127.0.0.1:8080/
curl -i http://127.0.0.1:8080/health
curl -i http://127.0.0.1:8080/not-real
curl -i -X POST http://127.0.0.1:8080/demo/form -d "name=Rust&message=Hello"
```
