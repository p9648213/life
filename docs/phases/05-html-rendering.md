# Phase 05: HTML Rendering and Template Compiler Foundation

Goal: build the first usable version of a small compiled-template engine that turns template source into Rust render code before request handling.

You are building the foundation for a real template compiler. The long-term direction is similar in spirit to Sailfish: template structure is translated before request handling, and rendering should mostly append known literals and typed values into an output buffer.

For this phase, "compiled template engine" means the request-time render path must not scan template text, find placeholders, or perform string replacement. The basic version should compile a tiny template format into Rust code that writes directly into a caller-provided output buffer.

Keep the compiler small. It does not need loops, conditionals, inheritance, includes, macros, or a complete HTML parser yet. The foundation matters more than features: the generated render code should already have the shape you want to keep later.

This phase focuses on the compiler and generated render boundary. Multiple application pages, reusable layouts, forms, navigation, styling, and a complete browser flow can be added after the compiler foundation is solid.

## What to Learn

- Choosing explicitly between raw and escaped runtime text
- A minimal template compiler pipeline
- A generated render API
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
<h1>{title:escape}</h1>
```

Can become:

```text
Literal("<h1>")
Variable("title", Escape)
Literal("</h1>")
```

Code generation then turns those tokens into Rust code:

```rust
out.push_str("<h1>");
crate::util::escape_html(ctx.title, out);
out.push_str("</h1>");
```

For this phase, you do not need parser generators, formal grammars, LLVM, bytecode, optimization passes, proc macro internals, or a full HTML parser. A simple scanner that reads until `{`, reads a variable name and optional colon-separated operations until `}`, and repeats is enough to begin. The only supported operation is `escape`. Repeating it, as in `{title:escape:escape}`, is valid and still selects one escape call; an empty or unknown operation is an error.

The important early errors are practical:

- Opened `{` but never found `}`.
- Empty variable name.
- Invalid variable name.
- Empty or unsupported variable operation.
- Template generated invalid Rust.

## Where to Look

- OWASP XSS overview: https://owasp.org/www-community/attacks/xss/

## Scope Boundary

One small template is enough to prove this phase. It should contain literal HTML, a raw runtime variable, and an escaped runtime variable. A focused test may call the generated renderer directly; wiring it into a real application handler is outside this checkpoint.

## Step-by-Step Work

1. Define a tiny template source format. For this phase, `{title}` emits a raw value, `{title:escape}` emits an HTML-escaped value, and repeated `:escape` operations still escape once.
2. Write a compiler path that turns template source into Rust code.
3. Generate render functions that write into a caller-provided output buffer.
4. Add an HTML escaping helper that writes directly into the existing output buffer.
5. Compile and call one dynamic template from a focused test so generated Rust, raw output, escaped output, and the runtime data boundary are checked together.
6. Keep compiler and rendering logic separate from routing and TCP code.
7. Keep the generated render path allocation-conscious from the beginning.

## Compiled Template Requirement

The hot render path should look conceptually like:

```text
append literal HTML
append a raw or escaped typed value according to the compiled operation
append literal HTML
```

It should not look like:

```text
read template source
find "{name}"
replace "{name}" with a value
repeat for every placeholder
```

For the first compiler, a simple token stream is enough:

```text
Literal("<h1>")
EscapedExpr("title")
Literal("</h1><p>")
RawExpr("trusted_html")
Literal("</p>")
```

That can generate Rust shaped like:

```rust
pub fn render_home(ctx: &HomeTemplate<'_>, out: &mut String) {
    out.push_str("<h1>");
    crate::util::escape_html(ctx.title, out);
    out.push_str("</h1><p>");
    out.push_str(ctx.trusted_html);
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
  -> tokenize literals, {raw_variables}, and {escaped_variables:escape}
  -> generate Rust render function
  -> write generated file
  -> include generated render code and call it from tests or handlers
```

The first tokenizer does not need to understand all HTML. It can treat everything outside `{...}` as literal text. That is acceptable because this compiler runs before request handling. Later phases can decide whether to add stricter HTML parsing.

Keep generated code boring. Boring generated code is easier to inspect, test, profile, and replace.

## Template Paths and Generated Names

Template paths become generated Rust function and view names. Each path component must start with a lowercase ASCII letter or `_`; the remaining characters may be lowercase ASCII letters, digits, or `_`.

Valid:

```text
templates/admin/card_2/page_2.html
```

This can generate names shaped like:

```text
render_admin_card_2_page_2
AdminCard2Page2View
```

Invalid:

```text
templates/admin/card-2/page_2.html
templates/admin/2card/page_2.html
```

The `-` is not valid in a Rust identifier, and a component cannot start with a digit. The build script rejects unsupported characters; if a path still produces invalid generated Rust, let the Rust compiler report its normal diagnostic.

The current view-name conversion removes underscores. Avoid placing both `page2.html` and `page_2.html` in the same template directory, because they can generate the same view struct name and Rust will report the duplicate definition during compilation.

## When the Compiler Runs

The target model is build-time compilation:

```text
cargo build
  -> build.rs runs
  -> template compiler reads templates
  -> generated Rust is written to OUT_DIR
  -> rustc compiles generated render functions
  -> tests or request handlers call compiled Rust functions
```

At request time, the server should not read template files or compile templates. Application code may create context data, allocate or reuse an output buffer, and call the already-generated render function. Phase 05 verifies this render boundary in tests; application wiring can happen separately.

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
    crate::util::escape_html(ctx.title, out);
    out.push_str("</h1></main>");
}
```

Avoid generated code shaped like:

```rust
let html = template_source.replace("{title}", ctx.title);
```

The first version should:

- Render into one caller-owned `String`.
- Append large literal chunks when possible.
- Append raw dynamic text directly into the output buffer when the template requests raw output.
- Escape marked dynamic text directly into the output buffer without a temporary `String`.
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

Escaping is selected per variable occurrence in the template:

```text
{name}                 appends name as raw text
{name:escape}          HTML-escapes name into the output buffer
{name:escape:escape}   is valid and produces the same single escape call
```

Repeated `:escape` operations are duplicate-tolerant syntax; they do not escape the already-escaped output a second time. Empty operations such as `{name:}` and unknown operations return a compiler error.

Use raw variables only for content that is already trusted and intended to contain HTML. Any user-controlled or otherwise untrusted value must use the `:escape` operation.

At minimum, escape these characters in text content:

```text
&  becomes &amp;
<  becomes &lt;
>  becomes &gt;
"  becomes &quot;
'  becomes &#x27;
```

Do not pass raw user input through an unescaped variable.

## Experiments

Render a template containing `{message:escape}` with sample text like:

```html
<script>alert("xss")</script>
```

The escaped occurrence should be emitted as text. A separate `{trusted_html}` occurrence should remain raw so both operations are verified.

## Questions to Answer

- Why does server-generated HTML need escaping?
- When is it valid to select raw output instead of `:escape`?
- What is the difference between HTML text and an HTML attribute?
- What work happens before request handling in your compiler?
- What generated code still runs at request time?
- Why is string-based placeholder lookup slower and less type-safe than typed template data?
- Why should the generated render function write into a caller-provided buffer?
- What would force you to redesign the render API later?

## Checkpoint

You are done when:

- The compiler accepts literal HTML, raw `{name}` variables, escaped `{name:escape}` variables, and repeated `:escape` operations that generate one escape call.
- Unmatched, empty, and invalid variables, plus empty or unsupported operations, return useful compiler errors.
- At least one dynamic template compiles into valid Rust and is called from a focused test with runtime data.
- The generated renderer appends raw variables directly and escapes `:escape` variables into the same output buffer.
- Template source is compiled before request handling.
- Generated render code does not parse template source or replace placeholders during request handling.
- Generated templates render into an explicit output buffer or equivalent response body builder.
- Compiler and rendering helpers are separate from routing and TCP code.
- The basic compiler shape can be extended in the continuation phase without changing the handler/render boundary.

## Continue

After this phase, continue with [Phase 06: Form Parsing](06-form-parsing.md).
