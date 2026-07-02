# Phase 24: Backend Core API Boundary

Goal: make real application code sit on top of the backend core cleanly.

This is where the project stops being a sequence of demos and becomes a reusable backend foundation for your own app.

## What to Learn

- Public versus private APIs
- Handler signatures
- App context
- Dependency direction
- Stable module boundaries
- Avoiding framework-style hidden control flow

## Where to Look

- Rust visibility: https://doc.rust-lang.org/reference/visibility-and-privacy.html
- Rust modules: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
- Trait objects, later if needed: https://doc.rust-lang.org/book/ch17-02-trait-objects.html

## Step-by-Step Work

1. Draw the current request flow.
2. Mark which modules are backend core.
3. Mark which modules are app-specific.
4. Decide what a handler receives.
5. Decide what a handler returns.
6. Keep the router independent of `TcpStream`.
7. Keep app storage choices out of the parser and response builder.
8. Write a small example app layer using the public boundary.

## Boundary Test

Ask this before adding any public API:

```text
Does the backend core need this, or does one app want this?
```

If only one app wants it, keep it in app code.

## Questions to Answer

- Which types should be public?
- Which fields should stay private behind accessors?
- Can the app add routes without changing connection code?
- Can storage change without changing the parser?

## Checkpoint

You are done when:

- The backend core has a small public surface.
- App-specific code is easy to find.
- Replacing the sample app does not require rewriting TCP, parsing, or response code.
