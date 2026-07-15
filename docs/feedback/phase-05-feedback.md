# Phase 05 Feedback

## Overall

You completed the Phase 05 goal: template source is compiled into Rust before request handling, and the generated render functions append directly into a caller-provided output buffer.

The implementation now has a useful small compiler boundary:

```text
templates/
  -> build.rs discovers template files
  -> htmlc tokenizes and validates template syntax
  -> htmlc generates Rust render code in OUT_DIR/templates.rs
  -> src/lib.rs includes the generated module
  -> tests call the generated renderer with runtime values
```

I verified that:

- `cargo test --manifest-path crates/htmlc/Cargo.toml` passes with 18 tests.
- `cargo test --test templates` passes.
- `cargo check` passes.
- `cargo fmt --check` passes.
- `git diff --check` passes.
- Nested template paths compile into generated functions and view structs.
- Literal HTML, quoted attributes, Unicode, backslashes, and newlines are emitted as valid Rust string literals.
- Repeated variables share one generated view field.
- `{name}` renders raw text and `{name:escape}` writes HTML-escaped text directly into the final output buffer.
- Empty variables, unmatched braces, invalid names, and invalid operations return compiler errors.

## What You Did Well

- You kept `build.rs` focused on orchestration. The compiler lives in the separate `htmlc` crate instead of being hidden inside the build script.
- You compile templates at build time and include the generated Rust from `OUT_DIR`, so request-time rendering does not scan template text or replace placeholders.
- You chose an explicit per-occurrence escape operation. That makes raw versus escaped output visible in the template and lets the same value be rendered differently where needed.
- Your `escape_html` helper appends directly into the caller-owned output buffer. This avoids allocating a temporary escaped `String` for every escaped variable.
- You added focused compiler tests for successful generation, literal escaping, repeated variables, malformed delimiters, invalid names, and invalid operations.
- You added an integration test that compiles a real nested template and verifies raw and escaped runtime output together.
- You documented the template syntax, build-time flow, output-buffer boundary, and template-path naming convention in the phase guide.
- You kept dependencies minimal and did not introduce a web framework or runtime template interpreter.

## Things To Improve Later

- The compiler errors identify the error kind but do not yet carry a template filename, byte offset, or line and column. Improve diagnostics when templates become larger.
- The build script currently uses `unwrap` around file access and compiler results. That is fine for this phase, but later attach the template path to build errors before reporting them.
- The template-path rule intentionally relies on Rust compilation for some edge cases, such as a path component starting with a digit or generated-name collisions. If this becomes frustrating, move those checks into the build script and report the conflicting paths directly.
- Raw `{name}` output is powerful but must only be used for trusted HTML. Keep untrusted user-controlled values on the explicit `{name:escape}` path.
- Escaping currently targets HTML text content. Attribute, URL, JavaScript, CSS, and other contexts need separate rules before they are supported.
- Generated view fields are currently borrowed `&str` values. Future phases may need typed fields, nested paths, conditionals, loops, layouts, includes, and source-mapped compiler diagnostics.
- The phase verifies rendering through a focused integration test. Wiring generated templates into real application handlers can remain separate until the application domain is chosen.

## Ready For Phase 06

You are ready to move on to Phase 06: Form Parsing.

Carry forward the same separation of work: parse and validate request input at a clear boundary, represent valid data explicitly, and keep handlers focused on application decisions rather than low-level string processing.
