# Phase 21: Configuration and Runtime Limits

Goal: make runtime behavior and resource limits explicit.

Design the configuration sources and type structure yourself.

## Expected Behavior

The server can change its bind address, storage paths, timeouts, and resource limits without source edits. Invalid configuration fails before accepting traffic.

## Requirements

- Give every configurable value a name, type, default, and validation rule.
- Include bind address, storage location, request limits, timeouts, concurrency limits, and retained-state limits where relevant.
- Define precedence when more than one configuration source is supported.
- Validate cross-field relationships and arithmetic at startup.
- Fail clearly on missing required or invalid values.
- Log the effective non-secret configuration.
- Never log secrets or credentials.
- Pass configuration explicitly to the components that use it.
- Re-evaluate algorithmic and memory cost whenever a limit increases.

## Tests to Write

- defaults produce a valid configuration;
- explicit values override defaults according to policy;
- malformed, out-of-range, and conflicting values fail;
- the server can bind a different configured address;
- configured limits reach the correct subsystem;
- secret values are redacted from diagnostics.

## Checkpoint

You are done when runtime values are centralized, validated before startup, and observable without exposing secrets.

After this, continue with [Phase 22: Observability and Diagnostics](22-observability-diagnostics.md).
