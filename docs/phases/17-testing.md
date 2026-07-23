# Phase 17: Testing

Goal: make the backend safe to change through deterministic automated tests.

Design the test layout and helpers yourself.

## Expected Behavior

`cargo test` verifies the important HTTP, application, persistence, and concurrency contracts without requiring every test to use a real socket or browser.

## Requirements

- Prefer pure tests for parsing, serialization, routing, escaping, and storage conversion.
- Cover valid, invalid, boundary, and adversarial inputs.
- Add a regression test for every fixed bug.
- Test both results and state invariants after failures.
- Test persistence with isolated temporary locations.
- Keep a small number of end-to-end socket tests for integration boundaries.
- Make tests deterministic; avoid timing guesses when synchronization or work-count invariants are possible.
- Treat unexpectedly slow maximum-size tests as possible complexity bugs.
- Keep fixtures bounded and understandable.

## Tests to Write

- request and response framing round-trips where applicable;
- parser rejection cases do not panic;
- routing distinguishes path and method outcomes;
- form decoding and HTML escaping cover boundary inputs;
- storage round-trips and rejects corruption;
- session and authentication invariants hold;
- concurrent mutations preserve state;
- configured limits work at just-below, exact, and just-above boundaries.

## Checkpoint

You are done when the critical contracts have deterministic regression coverage and failures identify the responsible boundary clearly.

After this, continue with [Phase 18: Database Layer](18-database-layer.md).
