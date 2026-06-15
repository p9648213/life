# Phase 12: Sessions

Goal: connect a browser cookie to server-side state.

A session cookie usually stores only a random session ID. The actual session data lives on the server.

## What to Learn

- Session ID
- Server-side session map
- Expiration
- Login state
- Logout

## Where to Look

- OWASP session management: https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html
- Rust collections: https://doc.rust-lang.org/std/collections/

## Step-by-Step Work

1. Create a session store in app state.
2. Generate a random session ID when needed.
3. Store session data under that ID.
4. Send the ID in a cookie.
5. On each request, parse the cookie.
6. Look up the session in the store.
7. Add a logout route that deletes the session.
8. Expire the cookie during logout.

## Randomness Rule

Do not use counters or timestamps for session IDs.

For learning only, you can start by using a placeholder to understand the flow, but before calling it done, use a proper randomness source.

## Questions to Answer

- Why not store all session data in the cookie?
- What happens if a session ID is predictable?
- What does logout need to remove?

## Checkpoint

You are done when:

- A browser gets a session ID.
- Server remembers data for that session.
- Logout removes the session.

