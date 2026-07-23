# Phase 12: Sessions

Goal: connect a browser cookie to server-side session data.

Design the session types, storage layout, and handler API yourself.

## Expected Behavior

A browser receives an opaque session ID cookie. Later requests use that ID to retrieve server-side session data. Logout removes the server-side session and expires the cookie.

## Requirements

- Store only an opaque session ID in the cookie.
- Generate IDs with a cryptographically secure randomness source.
- Do not use counters, timestamps, or homemade randomness.
- Make IDs long enough to resist guessing.
- Handle missing, malformed, unknown, and expired IDs safely.
- Set appropriate cookie attributes.
- Bound session count and retained session data.
- Define expiration and cleanup behavior.
- Rotate the session ID after authentication.
- Logout must invalidate both server-side state and the browser cookie.

## Tests to Write

- a new session can be created and retrieved;
- different clients receive different IDs;
- unknown and expired IDs do not expose session data;
- logout removes the session and expires the cookie;
- session limits are enforced;
- authentication rotates the prior session ID.

## Checkpoint

You are done when session IDs are unpredictable, session data remains server-side, expiration is defined, and logout invalidates the session completely.

After this, continue with [Phase 13: Passwords and Authentication](13-passwords-authentication.md).
