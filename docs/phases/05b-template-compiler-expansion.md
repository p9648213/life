# Phase 05B: Template Compiler Expansion

Goal: extend the basic template compiler from Phase 05 without changing the request-time render model.

This phase continues the compiler work after the first generated templates are rendering real pages. Do not jump to a full framework. Improve the compiler in small steps while preserving the core performance shape:

```text
generated render function
  -> append literal chunks
  -> escape typed values
  -> render children or repeated sections
  -> write into caller-provided output buffer
```

## What to Learn

- Compiler pipeline boundaries
- Template language design
- Generated Rust readability
- Source locations and diagnostics
- Build-time generation versus checked-in generated files
- How loops, conditionals, attributes, and layouts affect escaping and allocation

## Step-by-Step Work

1. Separate the compiler into clear stages: source loading, tokenizing, parsing, code generation, and file output.
2. Add source locations to compiler errors.
3. Add field paths such as `{{ user.name }}` if the generated Rust can stay simple.
4. Add conditionals only after plain escaped variables are solid.
5. Add loops only after conditionals are solid.
6. Add layout or child-template support without making routing build HTML manually.
7. Decide whether generated Rust should be checked in, written under `OUT_DIR`, or both during learning.
8. Only after the standalone generator is understandable, consider a `build.rs` integration.

## Build Script Integration

The `build.rs` version should run the same compiler pipeline automatically during `cargo build`.

Target flow:

```text
templates/*.html
  -> build.rs
  -> compiler library
  -> OUT_DIR/templates.rs
  -> include generated render functions in the crate
```

Keep the compiler logic in a normal Rust module or helper crate. Let `build.rs` orchestrate file paths and Cargo rebuild rules; do not hide the parser and code generator inside the build script itself.

Useful `build.rs` responsibilities:

- Find template source files.
- Tell Cargo when to rerun the build script.
- Call the compiler pipeline.
- Write generated Rust under `OUT_DIR`.
- Fail the build with clear template diagnostics.

The request path should remain unchanged after this move. Handlers should still call generated render functions; the only difference is that generation now happens automatically before Rust compilation.

## Template Features To Add Slowly

Start with escaped variables:

```html
<h1>{{ title }}</h1>
```

Then consider field paths:

```html
<p>{{ user.display_name }}</p>
```

Then conditionals:

```html
{% if logged_in %}
  <a href="/account">Account</a>
{% else %}
  <a href="/login">Login</a>
{% endif %}
```

Then loops:

```html
<ul>
{% for item in items %}
  <li>{{ item.name }}</li>
{% endfor %}
</ul>
```

Each feature should compile into direct Rust control flow, not runtime interpretation.

## Generated Code Shape

Generated loops should look like ordinary Rust:

```rust
out.push_str("<ul>");
for item in ctx.items {
    out.push_str("<li>");
    crate::html::escape::text(out, item.name);
    out.push_str("</li>");
}
out.push_str("</ul>");
```

Generated conditionals should also be ordinary Rust:

```rust
if ctx.logged_in {
    out.push_str("<a href=\"/account\">Account</a>");
} else {
    out.push_str("<a href=\"/login\">Login</a>");
}
```

The compiler should make request-time rendering boring and predictable.

## Design Questions

- Does the compiler own context struct generation, or do you write context structs by hand?
- How will the compiler report `{{ missing_field }}` clearly?
- Are attributes escaped differently from text content?
- How will templates compose without building temporary strings for every child?
- Should the compiler merge adjacent literal chunks?
- Where should generated files live during development?
- What benchmark would prove that a compiler change actually improved rendering?

## Checkpoint

You are done when:

- The compiler has distinct stages instead of one large string-processing function.
- Errors point back to useful template source locations.
- At least one new template feature compiles into direct Rust.
- Generated code still writes into a caller-provided buffer.
- Request handling still does not parse template source.
- The render API from Phase 05 did not need a major redesign.
