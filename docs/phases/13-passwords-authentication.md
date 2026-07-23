# Phase 13: Passwords and Authentication

Goal: add registration and login without implementing unsafe cryptography.

Design the user model and authorization boundary yourself.

## Expected Behavior

A user can register, log in through a session, access a protected sample action, and log out. Failed authentication does not reveal whether a particular account exists.

## Requirements

- Use a proven password-hashing library such as Argon2; do not implement hashing yourself.
- Store only password hashes, never raw passwords.
- Use the library's salt generation and verification APIs.
- Validate and bound usernames and passwords before expensive work.
- Do not use plain SHA-256 or reversible encryption for password storage.
- Do not log passwords, hashes, or session secrets.
- Use generic login failure messages.
- Rotate the session ID after successful login.
- Keep authentication and application authorization decisions distinct.
- Document rate-limiting needs even if enforcement comes later.

## Tests to Write

- registration stores a verifiable hash rather than the password;
- correct credentials authenticate;
- incorrect credentials fail without creating authenticated state;
- protected actions reject unauthenticated sessions;
- login rotates the session ID;
- logout removes authenticated state;
- secrets do not appear in normal diagnostic output.

## Checkpoint

You are done when authentication uses a proven password-hashing implementation, protected actions require authorization, and raw credentials are never stored or logged.

After this, continue with [Phase 14: Error Handling](14-error-handling.md).
