# Phase 20: Frontend Interactivity Adapter

Goal: progressively enhance one browser workflow without replacing the server-rendered application.

Design the JavaScript organization and UI behavior yourself.

## Expected Behavior

One form can submit through `fetch` and display success or validation feedback. The same workflow still works through normal browser submission when JavaScript is unavailable.

## Requirements

- Serve a small JavaScript file from a static route with the correct content type.
- Use browser APIs directly; do not add a frontend framework for this phase.
- Keep server-side validation authoritative.
- Preserve normal form action and method behavior.
- Handle network, non-JSON, validation, and server failures visibly.
- Avoid rendering untrusted strings through unsafe HTML insertion.
- Prevent accidental duplicate submissions while a request is active.
- Keep browser state separate from persistent server state.

## Tests to Write

- the script is served with the correct content type;
- normal form submission still works without JavaScript;
- enhanced submission handles success and validation errors;
- network and unexpected server responses remain recoverable;
- user-controlled text is inserted safely;
- one user action does not create duplicate requests.

## Checkpoint

You are done when one workflow is enhanced without making JavaScript mandatory or moving application authority into the browser.

After this, continue with [Phase 21: Configuration and Runtime Limits](21-configuration-runtime-limits.md).
