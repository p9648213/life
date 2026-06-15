# Phase 19: Optional JSON API

Goal: compare server-rendered HTML with machine-readable API responses.

Do this after the HTML app works.

## What to Learn

- JSON
- API routes
- Status codes
- Validation
- Browser `fetch`

## Where to Look

- MDN JSON: https://developer.mozilla.org/en-US/docs/Learn/JavaScript/Objects/JSON
- MDN Fetch API: https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
- MDN Content-Type: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type

## Step-by-Step Work

1. Add `GET /api/notes`.
2. Return notes as JSON.
3. Set `Content-Type: application/json`.
4. Add `POST /api/notes`.
5. Validate request body.
6. Return useful status codes.
7. Keep HTML routes working.

## Dependency Rule

At first, build tiny JSON strings only for simple output so you see the format. Once the shape grows, using `serde_json` is reasonable.

## Questions to Answer

- How does an API route differ from an HTML route?
- Who consumes JSON?
- What error shape should an API return?

## Checkpoint

You are done when:

- HTML pages still work.
- API routes return JSON.
- Invalid API input returns a clear error.

