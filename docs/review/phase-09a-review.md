# Phase 09A Review

Review date: 2026-09-04

## Result

Phase 09A is not complete yet. The current format uses explicit big-endian integers, persistent IDs, live/tombstoned record frames, a direct ID-offset index, and header metadata for live records and dead bytes. Deletion now tombstones its frame instead of physically removing it, and update appends a replacement before tombstoning the old frame.

## Findings

| Priority | Finding | Regression tests |
| --- | --- | --- |
| P1 | An uncached `record_count()` reads through the following `dead_bytes` field and returns a slice-conversion error. | `uncached_record_count_reads_only_the_record_count_header_field` |
| P1 | `list()` accepts an unknown frame-state byte instead of rejecting malformed storage. | `invalid_frame_flag_is_rejected` |
| P1 | A tombstoned frame whose declared payload extends beyond EOF is accepted because seeking can move beyond the physical file end. | `truncated_tombstoned_frame_is_rejected` |
| P1 | Update tombstones the replaced frame but does not add its full frame length to `dead_bytes`. | `update_adds_the_replaced_frame_to_dead_bytes` |

The callable update behavior now preserves the record ID and unaffected records across reopening. Storage size and work limits remain in [Phase 09B](../phases/09b-file-storage-limits.md). Reuse or compaction of accumulated tombstoned space remains a measured [Phase 09C](../phases/09c-file-storage-performance-optimization.md) concern.

## Tests

The Phase 09A suite is in [`tests/storage.rs`](../../tests/storage.rs).

```text
32 tests total
28 passed
4 failed against current production behavior
0 ignored
```

Run it with:

```sh
cargo test --test storage -- --test-threads=1
```

Current static verification:

```text
rustfmt --edition 2021 --check tests/storage.rs passed
git diff --check                                passed
```
