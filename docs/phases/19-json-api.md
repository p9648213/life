# Phase 19: JSON API

Goal: add a machine-readable adapter alongside the HTML application.

Design the API representation and error envelope yourself.

## Expected Behavior

API routes can list and create resources as JSON while existing HTML routes continue to work.

## Requirements

- Support `GET /api/resources` and `POST /api/resources`.
- Return `Content-Type: application/json`.
- Validate the request media type before decoding.
- Distinguish unsupported media type, malformed JSON, and application-invalid data.
- Return valid JSON for both success and error responses.
- Escape and serialize strings correctly; use `serde_json` once shapes are non-trivial.
- Set useful status codes for creation, validation, missing resources, and internal failures.
- Bound request bodies, collection sizes, and response sizes.
- Reuse application behavior rather than duplicating rules in the API adapter.

## Tests to Write

- list and create responses are valid JSON;
- content type is correct;
- malformed JSON and unsupported media types remain distinct;
- application validation matches the HTML path;
- strings with quotes, control characters, and Unicode serialize correctly;
- HTML routes remain unchanged;
- size limits are enforced.

## Checkpoint

You are done when HTML and JSON adapters share application rules, all API responses are valid JSON, and invalid input receives a stable error shape.

After this, continue with [Phase 20: Frontend Interactivity Adapter](20-frontend-interactivity.md).
