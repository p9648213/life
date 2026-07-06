# Phase 05: HTML Rendering

Goal: return real HTML pages from your server through a small compiled-template rendering engine, without making HTML the core app architecture.

You are building the first version of a server-rendered HTML engine. The long-term direction is a compiled template engine, similar in spirit to Sailfish: template structure is translated before request handling, and rendering should mostly append known literals and typed values into an output buffer.

For this phase, "compiled template engine" means the render path must not scan template text, find placeholders, or perform string replacement while handling a request. A tiny first version may use hand-written Rust render functions or structs that represent the code a future compiler would generate. Later phases can replace that manual step with real parsing, code generation, proc macros, caching, or build-time compilation.

Treat these pages as an adapter for browser workflows, not as the definition of the backend core.

## What to Learn

- HTML document structure
- Escaping user-controlled text
- Links
- Forms
- A minimal compiled-template render API
- Reusable layout templates and helpers
- The difference between compile-time template structure and request-time values

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

1. Define a small rendering boundary, such as a template object or function that writes into a caller-provided output buffer.
2. Make a compiled layout template that wraps page content in a full HTML document.
3. Make a compiled home page template.
4. Make a compiled health or debug page template.
5. Make one temporary compiled form page template for exercising form parsing.
6. Add links between pages.
7. Add an HTML escaping helper before displaying user text.
8. Keep routing responsible for choosing handlers/templates, not for building HTML strings inline.

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

For now, it is acceptable to manually write the Rust functions or structs that a future template compiler would generate. The important design habit is that template structure is known before request handling, while request-time rendering only supplies data and appends output.

Keep the public boundary small. Prefer an API shape that can evolve toward generated templates later, such as rendering into a provided buffer instead of always returning newly allocated strings.

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
- What work happens before request handling in a compiled template engine?
- What work still happens at request time?
- Why is string-based placeholder lookup slower and less type-safe than typed template data?

## Checkpoint

You are done when:

- Pages render in the browser.
- Pages link to each other.
- User text is escaped before display.
- HTML rendering does not parse template source or replace placeholders during request handling.
- Templates render into an explicit output buffer or equivalent response body builder.
- HTML helpers are separate from routing and TCP code.
