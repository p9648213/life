# Phase 25: Security and Deployment Boundary

Goal: document what is safe, what is not safe yet, and how the backend should be exposed.

Serious code is not automatically production-safe. This phase separates engineering quality from internet-hardening.

## What to Learn

- Threat modeling
- Reverse proxies
- TLS termination
- Header trust boundaries
- Secrets
- Rate limiting
- Backup and restore
- Safe error messages

## Where to Look

- OWASP Top 10: https://owasp.org/www-project-top-ten/
- OWASP Cheat Sheet Series: https://cheatsheetseries.owasp.org/
- Mozilla TLS guidance: https://wiki.mozilla.org/Security/Server_Side_TLS

## Step-by-Step Work

1. List what the server accepts from users.
2. List what data must be protected.
3. List what happens if the process crashes.
4. Decide whether a reverse proxy terminates TLS.
5. Document which headers are trusted only from the proxy.
6. Document backup and restore for persistent data.
7. Document known unsupported HTTP behavior.
8. Write a "not safe for public internet until" checklist.

## Do Not Skip

- Do not expose raw password handling without proven password hashing.
- Do not run public TLS with homemade crypto.
- Do not trust proxy headers from arbitrary clients.
- Do not log secrets or passwords.
- Do not claim production readiness without a rollback and restore path.

## Questions to Answer

- What can an unauthenticated user do?
- What happens if a request is huge or slow?
- Where are secrets stored?
- What must be backed up?
- What is handled by a reverse proxy instead of this Rust process?

## Checkpoint

You are done when:

- Remaining public-deployment risks are written down.
- TLS and password hashing are delegated to proven tools.
- Persistent data has a backup and restore plan.
- The app has a clear go/no-go checklist before public exposure.
