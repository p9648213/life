# Phase 21: Configuration and Runtime Limits

Goal: make runtime behavior explicit instead of hard-coded.

Configuration is not only convenience. It makes limits, addresses, storage paths, and debug behavior visible.

## What to Learn

- Environment variables
- Command-line arguments
- Default configuration
- Request and body limits
- Storage paths
- Debug versus release behavior

## Where to Look

- Rust environment variables: https://doc.rust-lang.org/std/env/
- Rust time durations: https://doc.rust-lang.org/std/time/struct.Duration.html
- `TcpStream` timeouts: https://doc.rust-lang.org/std/net/struct.TcpStream.html

## Step-by-Step Work

1. List every hard-coded runtime value.
2. Start with a small config struct.
3. Move bind address and port into config.
4. Move request header/body limits into config.
5. Move storage paths into config.
6. Validate config at startup.
7. Print the effective non-secret config at startup.

## Questions to Answer

- Which values should be configurable?
- Which values should stay compile-time decisions?
- What should happen when config is invalid?
- Which values are secrets and must not be logged?

## Checkpoint

You are done when:

- The server can run on a different port without editing source code.
- Limits are named in one place.
- Invalid config fails during startup.
