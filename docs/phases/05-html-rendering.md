# Phase 05: HTML Rendering

Goal: return real HTML pages from your server.

You are building server-rendered HTML manually. This teaches what template engines later automate.

## What to Learn

- HTML document structure
- Escaping user-controlled text
- Links
- Forms
- Reusable layout helpers

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

1. Make a helper that wraps page content in a full HTML document.
2. Make a home page.
3. Make a notes list page.
4. Make a new note form page.
5. Add links between pages.
6. Add an HTML escaping helper before displaying user text.

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

Create a note title like:

```html
<script>alert("xss")</script>
```

When displayed, it should appear as text, not run as code.

## Questions to Answer

- Why does server-generated HTML need escaping?
- What is the difference between HTML text and an HTML attribute?
- What does `Content-Type: text/html; charset=utf-8` tell the browser?

## Checkpoint

You are done when:

- Pages render in the browser.
- Pages link to each other.
- User text is escaped before display.

