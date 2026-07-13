# Phase 05 Review

Review date: 2026-07-13

## Overall Result

Phase 05 is not ready to mark complete yet.

The static compiled-template pipeline works:

```text
template file
  -> build.rs
  -> htmlc
  -> OUT_DIR/templates.rs
  -> include! in the library
  -> handler renders into a caller-owned String
```

The request path does not read or scan template files, and routing, rendering,
response generation, and TCP handling remain separate. However, the dynamic
template path is currently unusable and unsafe. The only wired template was
made static, which prevents the normal build from exposing the dynamic-path
problems described below.

This review is intentionally limited to the compiler foundation. Multiple
pages, reusable layouts, forms, navigation, styling, and a complete browser
flow are deferred until after the compiler is solid.

## Findings

### 1. Blocker: runtime values are not HTML-escaped

Location: `crates/htmlc/src/compiler.rs:60-63`

Generated variables use raw output shaped like:

```text
out.push_str(view.field)
```

Therefore a runtime value such as `<script>alert("xss")</script>` would be
inserted as executable markup instead of displayed as text. This violates the
escaping requirement in `docs/phases/05-html-rendering.md:219-241`.

The current tests whose names contain `escapes` only verify that literal HTML
is represented safely inside generated Rust string literals. They do not test
HTML escaping of runtime data.

### 2. Blocker: handlers cannot construct generated dynamic views

Locations:

- `crates/htmlc/src/compiler.rs:37-40`
- `src/lib.rs:4-6`
- `src/main.rs:10-13`

The generated struct is public, but its fields are private and it has no
constructor. The application binary imports templates through the `life`
library boundary, so constructing a dynamic view would fail with Rust error
`E0451` for private fields.

Choose one small public boundary:

- public generated fields;
- a generated constructor; or
- typed parameters accepted directly by the render function.

Then compile and call one real dynamic template from a handler to prove the
boundary works end to end.

### 3. High: malformed and repeated variables generate invalid Rust

Locations:

- `crates/htmlc/src/compiler.rs:38-40`
- `crates/htmlc/src/compiler.rs:60-63`
- `crates/htmlc/src/compiler.rs:71-74`

One generated field is created for every variable occurrence. Reusing the same
variable, for example `{title} ... {title}`, therefore creates duplicate struct
fields.

Variable names are also not trimmed or validated. Empty names, whitespace,
hyphens, punctuation, and Rust keywords can be copied into generated source.
`generate_code` always returns a `String`, so malformed templates cannot return
a useful compiler error.

For Phase 05, add a small compile-error type and cover the required basic
errors. Detailed source spans and advanced diagnostics can wait for Phase 05B.

### 4. Medium: template discovery panics on ordinary directory contents

Location: `build.rs:30-54`

Current discovery behavior has several fragile cases:

- `path.extension().unwrap()` panics for an extensionless file.
- A non-HTML regular file enters the recursive branch and is opened as a
  directory.
- A root-level template such as `templates/home.html` has no `/` for
  `split_once` and panics.
- Paths deeper than two components can generate invalid Rust names.
- Path parsing uses a hard-coded `/`, which is not portable.
- Discovered files are not sorted, so generated-source order is not
  deterministic.

Use `Path` operations, recurse only into directories, skip unrelated files,
validate generated identifiers, and sort the discovered templates.

### 5. Medium: tests do not validate generated behavior

Location: `crates/htmlc/tests/htmlc.rs:3-7`

`remove_whitespace` strips whitespace everywhere, including inside generated
Rust string literals. A meaningful HTML change such as `Hello world` becoming
`Helloworld` can therefore be hidden by the assertion helper.

The tests compare generated source strings but never compile and execute a
dynamic renderer. That is why private fields, duplicate fields, invalid names,
and raw HTML insertion are not caught.

At minimum, add one build-integrated dynamic template test that:

- constructs its generated context;
- invokes the generated function;
- checks the rendered HTML exactly;
- checks `&`, `<`, `>`, `"`, and `'` escaping;
- checks a `<script>` payload; and
- covers malformed and repeated variables.

The root project is not currently a Cargo workspace, so root `cargo test` does
not run the eight `htmlc` integration tests. Keep the separate compiler test
command documented, or later make workspace membership an explicit project
decision.

### 6. Low: formatting is not clean

`cargo fmt --all -- --check` currently reports formatting differences in:

- `crates/htmlc/tests/htmlc.rs`; and
- `src/lib.rs` because of its trailing blank line.

## What Is Already Good

- `htmlc` is a small build dependency with no unnecessary libraries.
- `build.rs` writes generated code under Cargo's `OUT_DIR`.
- `src/lib.rs` includes the generated Rust as normal compiled code.
- The compiler and its tests consistently use the intentional `{name}`
  variable syntax documented for Phase 05.
- Request handling does not scan templates or replace placeholders.
- The generated static renderer appends a literal chunk directly into a
  caller-owned output buffer.
- The router chooses the handler and does not construct page HTML.
- `Response::html` supplies `Content-Type: text/html; charset=utf-8` and owns
  the copied body bytes.
- Literal HTML is represented safely as a Rust string literal.

## Verification Performed

- `cargo check --all-targets`: passed.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `cargo clippy --manifest-path crates/htmlc/Cargo.toml --all-targets -- -D warnings`:
  passed.
- Root request, response, and routing tests: 34 passed.
- `cargo test --test server`, rerun with local socket permission: 2 passed.
- Total current root tests: 36 passed.
- `cargo test --manifest-path crates/htmlc/Cargo.toml --all-targets`: 8 passed.
- `git diff --check`: passed.
- `cargo fmt --all -- --check`: failed for the formatting differences listed
  above.

The passing checks only exercise the wired static template. They do not prove
that the dynamic rendering path works.

## Recommended Fix Order

- [ ] Return basic errors for unmatched, empty, or invalid variables.
- [ ] Decide how repeated variables share one context field.
- [ ] Add the text escaping helper and generate calls to it for dynamic values.
- [ ] Make generated contexts constructible across the library/binary boundary.
- [ ] Compile and render one dynamic template end to end.
- [ ] Replace whitespace-insensitive source snapshots with behavior-focused
      assertions.
- [ ] Add escaping and invalid-template regression tests.
- [ ] Harden and sort template discovery.
- [ ] Run formatting and the full validation command set again.

## Work That Can Wait

- Avoid optimizing the current `String` to `Vec<u8>` copy in `Response::html`
  until profiling shows it matters.
- Rich source locations, field paths, conditions, loops, inheritance, and
  advanced diagnostics belong in Phase 05B or later.
- Multiple pages, reusable layouts, forms, navigation, styling, and the full
  browser flow are deferred until after the compiler foundation.
- Static-file handling for the stylesheet belongs in the later static-files
  phase.

No implementation files were changed as part of this review.
