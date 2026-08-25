# Phase 09A Review

Review date: 2026-08-25

## Result

Phase 09A is not complete yet. The storage structure is a good foundation: responsibilities are separated, integers use explicit big-endian encoding, records are framed, IDs persist, and deletion physically removes record bytes.

## Findings

| Priority | Finding | Regression tests |
| --- | --- | --- |
| P1 | ID `0` and exhausted IDs can panic because arithmetic is unchecked. | `deleting_id_zero_returns_error_instead_of_panicking`, `exhausted_next_id_returns_error_instead_of_panicking_or_reusing_zero` |
| P1 | Truncated primitive and field reads can panic instead of returning a storage error. | `truncated_primitive_returns_error_instead_of_panicking`, `field_length_larger_than_its_payload_returns_error_instead_of_panicking` |
| P1 | `list()` ignores `record_count` and accepts partial prefixes, missing frames, extra frames, and trailing payload bytes. | `partial_frame_length_prefix_is_rejected`, `record_count_larger_than_available_frames_is_rejected`, `frames_beyond_the_declared_record_count_are_rejected`, `trailing_bytes_inside_a_framed_record_are_rejected` |
| P1 | Deletions around an existing index hole can make a surviving ID unreachable. | `deleting_records_around_an_existing_index_hole_preserves_later_ids` |
| P1 | Failed or interrupted store/index mutations can leave an inconsistency that reopening does not reject. | `index_write_failure_is_reported_and_inconsistent_reopen_is_rejected`, `corrupt_index_header_is_rejected_before_mutation`, `interrupted_insert_header_without_frame_is_rejected_on_reopen` |
| P2 | Traversal and absolute collection identifiers can escape the storage root. | `traversal_collection_identifier_is_rejected`, `absolute_collection_identifier_is_rejected` |
| P2 | Missing collections do not read as empty, and cached record counts become stale. | `missing_collection_reads_as_empty`, `cached_record_count_tracks_completed_deletion` |
| P1 phase gap | The collection has no callable update operation. | `update_preserves_id_and_every_unaffected_record` remains ignored |

Here, “callable update operation” means real storage behavior callable by a test, not an HTTP API. The ignored test does not force a method name or ownership design. Storage size and work limits are now deferred to [Phase 09B](../phases/09b-file-storage-limits.md) and are not Phase 09A completion requirements.

## Tests

The Phase 09A suite is in [`tests/storage.rs`](../../tests/storage.rs).

```text
29 tests total
12 passed
16 failed against current production behavior
1 ignored pending update behavior
```

Run it with:

```sh
cargo test --test storage -- --test-threads=1
```

Compilation and static verification:

```text
cargo test --workspace --no-run                          passed
cargo clippy --all-targets --all-features -- -D warnings passed
rustfmt --edition 2024 --check tests/storage.rs           passed
git diff --check                                          passed
```
