# Custom Backend From Scratch in Rust

This documentation is a learning and engineering path for building a custom Rust HTTP library from low-level pieces. It intentionally avoids full code solutions. Each phase tells you what to learn, where to look, what to try, and how to check your understanding.

The old roadmap used a small app as the center of the work. The new direction is different: build a backend core that can support whatever fullstack application you decide to build separately. Temporary sample routes are allowed only to exercise the backend.

Current status:

- Phases 00 through 03 are complete.
- The next implementation phase is Phase 04: routing.
- The current code has a TCP accept loop, a response builder, a request parser, and focused parser/response tests.
- The next design move is to split routing and handler behavior away from TCP I/O.

Main target:

- HTTP/1.1 server starting with `std::net`, with async I/O deferred to Phase 12B
- Manual request parsing
- Manual response generation
- Router and handler boundary
- Application state boundary
- HTML, form, and JSON adapters when needed
- Cookie parsing and serialization
- File-backed storage and database-backed storage
- Explicit error mapping
- Request limits and protocol hardening
- Configuration, diagnostics, and observability
- Concurrency model you understand
- Tests for the parts you build yourself
- Benchmarking and profiling before performance rewrites

Non-goals:

- Sessions, authentication, and application error policy belong to applications using the library.
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

Phase 05B is an optional, repeatable continuation. It does not block Phase 06A; return to it only when a concrete template requires another compiler capability.

After Phase 11, continue with Phase 12A.

Phase 12A establishes bounded thread-based concurrency. Phase 12B adds async I/O later and does not block Phase 13 or subsequent phases.

1. [Project Setup](phases/00-project-setup.md)
2. [TCP Server](phases/01-tcp-server.md)
3. [HTTP Response Builder](phases/02-http-response-builder.md)
4. [HTTP Request Parser](phases/03-http-request-parser.md)
5. [Routing](phases/04-routing.md)
6. [HTML Rendering and Template Compiler Foundation](phases/05-html-rendering.md)
7. [Need-Driven Template Compiler Expansion (Optional)](phases/05b-template-compiler-expansion.md)
8. [Minimal Request Body Accumulation](phases/06a-minimal-request-body-accumulation.md)
9. [Form Parsing](phases/06b-form-parsing.md)
10. [In-Memory State](phases/07-in-memory-state.md)
11. [Redirects](phases/08-redirects.md)
12. [File-Backed Storage Foundation](phases/09a-file-backed-storage-foundation.md)
    - [Phase 09A Learning Guides](guides/phase-09a/README.md)
    - [File-Storage Limits](phases/09b-file-storage-limits.md)
    - [File-Storage Performance Optimization (Deferred)](phases/09c-file-storage-performance-optimization.md)
13. [Static Files and CSS](phases/10-static-files-css.md)
14. [Cookies](phases/11-cookies.md)
15. [Phase 12A: Thread-Based Concurrency](phases/12a-thread-based-concurrency.md)
    - [Phase 12B: Async I/O (Deferred)](phases/12b-async-io.md)
16. [Better HTTP Behavior](phases/13-better-http-behavior.md)
17. [Testing](phases/14-testing.md)
18. [Database Layer](phases/15-database-layer.md)
19. [JSON API](phases/16-json-api.md)
20. [Frontend Interactivity Adapter](phases/17-frontend-interactivity.md)
21. [Configuration and Runtime Limits](phases/18-configuration-runtime-limits.md)
22. [Observability and Diagnostics](phases/19-observability-diagnostics.md)
23. [Benchmarking and Profiling](phases/20-benchmarking-profiling.md)
24. [Backend Core API Boundary](phases/21-backend-core-api-boundary.md)
25. [Security and Deployment Boundary](phases/22-security-deployment-boundary.md)

## Feedback Index

- [Current Backend Feedback](feedback/current-backend-feedback.md)
- [Phase 01 Feedback](feedback/phase-01-feedback.md)
- [Phase 02 Feedback](feedback/phase-02-feedback.md)
- [Phase 03 Feedback](feedback/phase-03-feedback.md)
- [Phase 09A Feedback](feedback/phase-09a-feedback.md)
- [Phase 10 Feedback](feedback/phase-10-feedback.md)
- [Phase 11 Feedback](feedback/phase-11-feedback.md)

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
