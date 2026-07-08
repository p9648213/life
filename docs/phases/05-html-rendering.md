# Phase 05: HTML Rendering and Template Compiler Foundation

Goal: return real HTML pages from your server through the first version of a small compiled-template rendering engine, without making HTML the core app architecture.

You are building the foundation for a real template compiler. The long-term direction is similar in spirit to Sailfish: template structure is translated before request handling, and rendering should mostly append known literals and typed values into an output buffer.

For this phase, "compiled template engine" means the request-time render path must not scan template text, find placeholders, or perform string replacement. The basic version should compile a tiny template format into Rust code that writes directly into a caller-provided output buffer.

Keep the compiler small. It does not need loops, conditionals, inheritance, includes, macros, or a complete HTML parser yet. The foundation matters more than features: the generated render code should already have the shape you want to keep later.

Treat these pages as an adapter for browser workflows, not as the definition of the backend core.

## What to Learn

- HTML document structure
- Escaping user-controlled text
- Links
- Forms
- A minimal template compiler pipeline
- A generated render API
- Reusable layout templates and helpers
- The difference between compile-time template structure and request-time values
- Why generated render code can be faster than runtime template interpretation

## Minimum Compiler Knowledge

You only need a small amount of compiler knowledge to start this phase.

Focus on this pipeline:

```text
template text
  -> tokenizer
  -> small token list
  -> Rust code string
  -> generated .rs file
```

A tokenizer turns template text into meaningful pieces. For example:

```html
<h1>{{ title }}</h1>
```

Can become:

```text
Literal("<h1>")
Variable("title")
Literal("</h1>")
```

Code generation then turns those tokens into Rust code:

```rust
out.push_str("<h1>");
crate::html::escape::text(out, ctx.title);
out.push_str("</h1>");
```

For this phase, you do not need parser generators, formal grammars, LLVM, bytecode, optimization passes, proc macro internals, or a full HTML parser. A simple scanner that reads until `{{`, reads a variable name until `}}`, and repeats is enough to begin.

The important early errors are practical:

- Opened `{{` but never found `}}`.
- Empty variable name.
- Invalid variable name.
- Template generated invalid Rust.

## Where to Look

- MDN HTML basics: https://developer.mozilla.org/en-US/docs/Learn/Getting_started_with_the_web/HTML_basics
- MDN forms: https://developer.mozilla.org/en-US/docs/Learn/Forms
- OWASP XSS overview: https://owasp.org/www-community/attacks/xss/

## Minimum Page Shape

Every page should eventually produce:

```html
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>...</title>
</head>
<body>
  ...
</body>
</html>
```

## Step-by-Step Work

1. Define a tiny template source format. Start with literal HTML plus escaped variables such as `{{ title }}`.
2. Write a compiler path that turns template source into Rust code.
3. Generate render functions that write into a caller-provided output buffer.
4. Make generated templates for a layout, a home page, a health or debug page, and one temporary form page.
5. Add an HTML escaping helper before displaying user text.
6. Keep routing responsible for choosing handlers/templates, not for building HTML strings inline.
7. Add links between pages.
8. Keep the generated render path allocation-conscious from the beginning.

## Compiled Template Requirement

The hot render path should look conceptually like:

```text
append literal HTML
append escaped typed value
append literal HTML
append already-rendered child content
```

It should not look like:

```text
read template source
find "{{ name }}"
replace "{{ name }}" with a value
repeat for every placeholder
```

For the first compiler, a simple token stream is enough:

```text
Literal("<h1>")
EscapedExpr("title")
Literal("</h1><p>")
EscapedExpr("message")
Literal("</p>")
```

That can generate Rust shaped like:

```rust
pub fn render_home(ctx: &HomeTemplate<'_>, out: &mut String) {
    out.push_str("<h1>");
    crate::html::escape::text(out, ctx.title);
    out.push_str("</h1><p>");
    crate::html::escape::text(out, ctx.message);
    out.push_str("</p>");
}
```

This example is intentionally small. The important design habit is that template structure is known before request handling, while request-time rendering only supplies data and appends output.

Keep the public boundary small. Prefer an API shape that can evolve toward generated templates later, such as rendering into a provided buffer instead of always returning newly allocated strings.

## Compiler Shape

Start with a standalone compiler or generator before wiring it into `build.rs`. It is easier to debug generated Rust when you can open the file directly.

A simple pipeline is:

```text
templates/home.html
  -> read source
  -> tokenize literals and {{ variables }}
  -> generate Rust render function
  -> write generated file
  -> include or call generated render code from handlers
```

The first tokenizer does not need to understand all HTML. It can treat everything outside `{{ ... }}` as literal text. That is acceptable because this compiler runs before request handling. Later phases can decide whether to add stricter HTML parsing.

Keep generated code boring. Boring generated code is easier to inspect, test, profile, and replace.

## When the Compiler Runs

The target model is build-time compilation:

```text
cargo build
  -> build.rs runs
  -> template compiler reads templates
  -> generated Rust is written to OUT_DIR
  -> rustc compiles generated render functions
  -> request handlers call compiled Rust functions
```

At request time, the server should not read template files or compile templates. It should only create context data, allocate or reuse an output buffer, and call the generated render function.

For the first version, it is still reasonable to use a manual generator:

```text
cargo run --bin templatec
  -> writes generated Rust you can inspect
cargo build
  -> compiles that generated Rust
```

Once the generated code shape is stable, move the same compiler path behind `build.rs`. A common shape is:

```rust
// generated file lives under OUT_DIR
include!(concat!(env!("OUT_DIR"), "/templates.rs"));
```

Using `include!` here is only a build integration detail. The template engine itself still does not need proc macros or runtime template interpretation.

## Performance Foundation

Build the basic version so you do not have to rewrite the render model later.

Prefer:

```rust
pub fn render(ctx: &Page<'_>, out: &mut String) {
    out.push_str("<main><h1>");
    escape_text(out, ctx.title);
    out.push_str("</h1></main>");
}
```

Avoid generated code shaped like:

```rust
let html = template_source.replace("{{ title }}", ctx.title);
```

The first version should:

- Render into one caller-owned `String`.
- Append large literal chunks when possible.
- Escape dynamic text directly into the output buffer.
- Avoid building an intermediate DOM tree.
- Avoid allocating a new `String` for every element.
- Allow rough capacity planning at the handler or generated-template boundary.

For example:

```rust
let mut body = String::with_capacity(1024 + ctx.title.len() + ctx.message.len());
render_home(&ctx, &mut body);
```

The capacity estimate does not need to be exact. Its job is to reduce reallocations while keeping the render API simple.

## Escaping

At minimum, escape these characters in text content:

```text
&  becomes &amp;
<  becomes &lt;
>  becomes &gt;
"  becomes &quot;
'  becomes &#39;
```

Do not display raw user input as HTML.

## Experiments

Submit sample text like:

```html
<script>alert("xss")</script>
```

When displayed, it should appear as text, not run as code.

## Questions to Answer

- Why does server-generated HTML need escaping?
- What is the difference between HTML text and an HTML attribute?
- What does `Content-Type: text/html; charset=utf-8` tell the browser?
- What work happens before request handling in your compiler?
- What generated code still runs at request time?
- Why is string-based placeholder lookup slower and less type-safe than typed template data?
- Why should the generated render function write into a caller-provided buffer?
- What would force you to redesign the render API later?

## Checkpoint

You are done when:

- Pages render in the browser.
- Pages link to each other.
- User text is escaped before display.
- Template source is compiled before request handling.
- Generated render code does not parse template source or replace placeholders during request handling.
- Generated templates render into an explicit output buffer or equivalent response body builder.
- HTML helpers are separate from routing and TCP code.
- The basic compiler shape can be extended in the continuation phase without changing the handler/render boundary.

## Continue

After this phase, continue with [Phase 05B: Template Compiler Expansion](05b-template-compiler-expansion.md).
