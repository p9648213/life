# Phase 17: Testing

Goal: make your low-level code safer to change.

The best parts to test are pure functions: parsers, serializers, route matching, escaping, and storage format conversion.

## What to Learn

- Unit tests
- Integration tests
- Test fixtures
- Edge cases
- Round-trip tests

## Where to Look

- Rust testing: https://doc.rust-lang.org/book/ch11-00-testing.html
- Cargo tests: https://doc.rust-lang.org/cargo/commands/cargo-test.html

## What to Test First

Prioritize:

- Response serialization
- Request parsing
- Form parsing
- HTML escaping
- Route matching
- Storage load/save round trips

## Step-by-Step Work

1. Add tests beside the functions they test.
2. Test one valid case.
3. Test one invalid case.
4. Test boundary cases.
5. Add regression tests when you fix bugs.

## Example Test Ideas

No full code, but test names may look like:

```text
parses_get_request_line
rejects_missing_http_version
escapes_script_tag_as_text
decodes_plus_as_space
serializes_content_length_as_body_bytes
storage_round_trips_record
```

## Questions to Answer

- Which code is hard to test because it mixes too many responsibilities?
- Which bugs did tests catch?
- Which behavior still requires browser testing?

## Checkpoint

You are done when:

- `cargo test` runs.
- Core parsers have valid and invalid tests.
- HTML escaping has tests.
