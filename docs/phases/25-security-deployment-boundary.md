# Phase 25: Security and Deployment Boundary

Goal: define what is safe, what remains unsafe, and how the application may be exposed.

Design the deployment topology and operational documents yourself.

## Expected Behavior

The project has an evidence-based go/no-go checklist for public exposure, including trust boundaries, secrets, TLS, resource controls, data recovery, and known unsupported behavior.

## Requirements

- Identify assets, entry points, trust boundaries, attackers, and abuse cases.
- Document what unauthenticated and authenticated users can do.
- Put public TLS behind a proven implementation; do not write cryptography yourself.
- Trust proxy headers only from a configured trusted proxy boundary.
- Define host, forwarded-address, and scheme handling.
- Keep secrets out of source, logs, responses, and backups shared insecurely.
- Document request, timeout, concurrency, session, and rate limits.
- Run with least privilege and define writable filesystem locations.
- Define database or file backup, restore, retention, and restore testing.
- Define startup, shutdown, health checking, rollback, and failure recovery.
- List unsupported HTTP behavior and remaining security risks.
- Do not claim production readiness without testing the complete deployed path.

## Tests to Write

- direct clients cannot spoof trusted proxy identity;
- TLS and security headers are verified at the deployed edge;
- secrets are absent from logs and error responses;
- resource limits work in the deployed topology;
- backup restoration is tested on an isolated destination;
- restart and rollback preserve required data;
- unauthenticated access matches the documented policy.

## Checkpoint

You are done when public-exposure risks are written down, proven tools own TLS and password hashing, recovery has been tested, and the deployment has a clear go/no-go decision.
