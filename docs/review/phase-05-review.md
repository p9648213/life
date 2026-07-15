# Phase 05 Review Follow-Up

This is a follow-up list from the first Phase 05 code review. Re-run the checks after completing the remaining items, then ask for another review.

## 1. Root-Level Template Paths

Status: addressed.

`templates/index.html` originally caused `build.rs` to panic because the name-generation code expected every template to have a directory component. The current `if let Some(...)` branch supports root-level templates.

Check again after later changes: `cargo check` must work with both `templates/index.html` and a nested template such as `templates/card/index.html`.

## 2. Generated Escape Import

Status: still open.

Dynamic templates generate `use life::util::escape_html;`. That path cannot refer to the `life` crate while `life` itself is being compiled. Generate a crate-local path that works from the generated `templates` module, then verify a template containing `{title}` builds successfully.

## 3. Generated-Code Snapshot Tests

Status: addressed.

The `htmlc` snapshots now match the public generated view fields, the `escape_html` import, and `escape_html(...)` calls.

Check again: `cargo test --manifest-path crates/htmlc/Cargo.toml`.

## 4. Malformed Template Errors

Status: tests added; implementation still open.

The compiler now returns `Result<String, TemplateError>`, and tests cover:

- Empty variable: `{}`.
- Unclosed variable: `{title`.
- Stray closing brace: `title}`.
- Invalid variable name: `{display name}`.

The parser must return `Err(...)` for every case. Do not leave `panic!` for the empty-variable case. Add the needed `TemplateError` variants and make all malformed-template tests pass.

## 5. Deeply Nested Template Names

Status: still open.

`templates/card/index.html` works. A dynamic template at `templates/admin/card/index.html` currently generates an invalid Rust name like `AdminCard/indexView`, because the second slash remains in the struct name.

Decide the supported template-path depth. If deeper nesting is supported, convert every path component into valid Rust identifier components. Also consider filenames containing characters Rust identifiers do not allow, such as `user-card.html`.

## 6. Escaping Without a Temporary String

Status: future performance refinement.

`escape_html(text)` returns a newly allocated `String`, and generated code then copies that result into the caller-provided page buffer. Later, change the escaping boundary so it writes escaped text directly into the existing output buffer.

This is not a correctness blocker. Keep the current behavior until the compiler and error handling are correct, then make the allocation improvement and benchmark it.

## Re-Review Commands

```sh
cargo test --manifest-path crates/htmlc/Cargo.toml
cargo check
```
