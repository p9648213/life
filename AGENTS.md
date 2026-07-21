# Project Guidance

This project is for building a serious custom Rust web backend from low-level pieces.

The goal is to understand and own the full backend flow: TCP, HTTP parsing, response generation, routing, handler boundaries, state, persistence, cookies, sessions, concurrency, observability, testing, and performance. The backend should stay hackable and explicit, not become an opinionated framework like Axum or Actix.

The application domain is separate from the backend core. Do not assume a to-do app, notes app, or any fixed product unless the user chooses one for a specific exercise.

Important constraints for assistance:

- Do not provide full code solution unless the user explicitly asks to break this rule. Complete test code is allowed when the user asks for tests.
- Prefer explanations, small snippets, pseudocode, diagrams, tests, and debugging hints.
- Guide the user step by step so they implement the important parts themselves.
- Avoid adding web frameworks or large libraries.
- Keep dependencies minimal and justify every dependency.
- Treat the codebase as a serious engineering project, but do not claim it is production-ready until security, deployment, operational, and protocol-hardening work has been done.
- Prefer explicit code flow and clear module boundaries over hidden framework-style magic.
- When giving feedback, cover correctness, API boundaries, testability, performance implications, and what should wait until measurement.
- For loops over network or other client-controlled input, explicitly analyze worst-case time and memory usage up to the configured limits. Look for growing buffers that are rescanned, nested loops whose work is amplified by partial reads, and repeated copying or allocation.
- Treat pathological complexity on permitted or adversarial input as a correctness and availability issue, not as optional optimization. Do not defer an obvious quadratic request-reading or parsing path merely because functional tests pass or a smaller current limit hides its cost.
- Evaluate boundary tests for both result and runtime. Prefer deterministic invariants or work-count checks when practical; at minimum, investigate max-size tests that are unexpectedly slow instead of accepting them as green only because they eventually pass.
- Re-evaluate algorithmic cost whenever request, header, body, buffer, or collection limits change. Record how the work scales with the limit, and require request framing and parsing to examine each input byte only a bounded number of times unless a different complexity is explicitly justified.
- Cryptography, password hashing, and TLS are exceptions: recommend proven libraries or external tools instead of teaching unsafe homemade implementations.

The detailed guide has been moved into separate files under `docs/`.

Start here:

- [docs/README.md](docs/README.md)
- [docs/backend-engine-plan.md](docs/backend-engine-plan.md)
- [docs/feedback/](docs/feedback/)

Phase files:

- [Phase 00: Project Setup](docs/phases/00-project-setup.md)
- [Phase 01: TCP Server](docs/phases/01-tcp-server.md)
- [Phase 02: HTTP Response Builder](docs/phases/02-http-response-builder.md)
- [Phase 03: HTTP Request Parser](docs/phases/03-http-request-parser.md)
- [Phase 04: Routing](docs/phases/04-routing.md)
- [Phase 05: HTML Rendering](docs/phases/05-html-rendering.md)
- [Phase 05B (Optional): Need-Driven Template Compiler Expansion](docs/phases/05b-template-compiler-expansion.md)
- [Phase 06A: Minimal Request Body Accumulation](docs/phases/06a-minimal-request-body-accumulation.md)
- [Phase 06B: Form Parsing](docs/phases/06b-form-parsing.md)
- [Phase 07: In-Memory State](docs/phases/07-in-memory-state.md)
- [Phase 08: Redirects](docs/phases/08-redirects.md)
- [Phase 09: File-Backed Storage](docs/phases/09-file-backed-storage.md)
- [Phase 10: Static Files and CSS](docs/phases/10-static-files-css.md)
- [Phase 11: Cookies](docs/phases/11-cookies.md)
- [Phase 12: Sessions](docs/phases/12-sessions.md)
- [Phase 13: Passwords and Authentication](docs/phases/13-passwords-authentication.md)
- [Phase 14: Error Handling](docs/phases/14-error-handling.md)
- [Phase 15: Concurrency](docs/phases/15-concurrency.md)
- [Phase 16: Better HTTP Behavior](docs/phases/16-better-http-behavior.md)
- [Phase 17: Testing](docs/phases/17-testing.md)
- [Phase 18: Database Layer](docs/phases/18-database-layer.md)
- [Phase 19: JSON API](docs/phases/19-json-api.md)
- [Phase 20: Frontend Interactivity Adapter](docs/phases/20-frontend-interactivity.md)
- [Phase 21: Configuration and Runtime Limits](docs/phases/21-configuration-runtime-limits.md)
- [Phase 22: Observability and Diagnostics](docs/phases/22-observability-diagnostics.md)
- [Phase 23: Benchmarking and Profiling](docs/phases/23-benchmarking-profiling.md)
- [Phase 24: Backend Core API Boundary](docs/phases/24-backend-core-api-boundary.md)
- [Phase 25: Security and Deployment Boundary](docs/phases/25-security-deployment-boundary.md)
