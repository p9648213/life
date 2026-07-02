# Phase 19: JSON API

Goal: support machine-readable API responses as a first-class backend adapter.

Do this after routing, body parsing, validation, and error mapping are understandable.

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

1. Add `GET /api/resources`.
2. Return sample records as JSON.
3. Set `Content-Type: application/json`.
4. Add `POST /api/resources`.
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
