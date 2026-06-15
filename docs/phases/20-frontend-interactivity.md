# Phase 20: Optional Frontend Interactivity

Goal: add a small amount of browser-side behavior without replacing the app.

The server-rendered app should continue to work without JavaScript.

## What to Learn

- DOM events
- `fetch`
- Progressive enhancement
- Client-side state
- Server-side state

## Where to Look

- MDN DOM: https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model
- MDN events: https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Building_blocks/Events
- MDN Fetch API: https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API

## Step-by-Step Work

1. Serve a JavaScript file from `/static/app.js`.
2. Add a tiny enhancement to the note form.
3. Submit with `fetch`.
4. Show success or validation errors.
5. Keep normal form submission working if JavaScript fails.

## Scope Rule

Do not add a frontend framework for this phase. The goal is to understand the browser APIs directly.

## Questions to Answer

- What still works with JavaScript disabled?
- Which state is stored in the browser?
- Which state is stored on the server?
- What changes when using `fetch` instead of normal form submission?

## Checkpoint

You are done when:

- One workflow is enhanced with JavaScript.
- The app still works without JavaScript.

