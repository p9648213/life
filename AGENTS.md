# Project Guidance

This project is for building a serious custom Rust web backend from low-level pieces.

The goal is to understand and own the full backend flow: TCP, HTTP parsing, response generation, routing, handler boundaries, state, persistence, cookies, sessions, concurrency, observability, testing, and performance. The backend should stay hackable and explicit, not become an opinionated framework like Axum or Actix.

The application domain is separate from the backend core. Do not assume a to-do app, notes app, or any fixed product unless the user chooses one for a specific exercise.

Important constraints for assistance:

- Do not provide full code solutions unless the user explicitly asks to break this rule.
- Prefer explanations, small snippets, pseudocode, diagrams, tests, and debugging hints.
- Guide the user step by step so they implement the important parts themselves.
- Avoid adding web frameworks or large libraries.
- Keep dependencies minimal and justify every dependency.
- Treat the codebase as a serious engineering project, but do not claim it is production-ready until security, deployment, operational, and protocol-hardening work has been done.
- Prefer explicit code flow and clear module boundaries over hidden framework-style magic.
- When giving feedback, cover correctness, API boundaries, testability, performance implications, and what should wait until measurement.
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
- [Phase 06: Form Parsing](docs/phases/06-form-parsing.md)
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
