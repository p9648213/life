# Phase 09A Feedback

## Current Scope

The storage foundation supports manual codecs, persistence across reopening, listing, ID lookup, insertion, tombstoned deletion, and append-and-tombstone update through a generic collection. It uses the standard library without a serialization or database dependency.

You chose a simpler learning checkpoint: trust valid collection/index files and matching codecs, and assume every mutation completes its writes without interruption. Operations are sequential, including operations through different handles. This supersedes the earlier feedback requiring full corruption and index-consistency validation.

The [Phase 09A requirements](../phases/09a-file-backed-storage-foundation.md) describe the current contract. Corruption detection, interrupted-write handling, and recovery remain deferred. This review does not establish production readiness.

## Test Scope

The suite was reduced from 83 tests to 38 by deleting the agreed 45 corruption and interrupted-write tests. The retained coverage is:

- 22 normal-operation, codec, ID, path, and linear-decoding tests;
- 7 arithmetic and large-ID tests;
- 3 missing-file or invalid-path tests;
- 6 unsupported-version tests.

Arithmetic fixtures sometimes construct artificial metadata or sparse files to reach numeric boundaries cheaply. Those tests remain useful independently of general corruption detection.

The previous 81-test review and its external corruption diagnostics describe an earlier, stricter implementation. They are not evidence of guarantees provided by the simplified collection.

## Verification After Simplification

- Storage suite: 38 passed, 0 failed.
- `cargo test --locked --workspace --quiet`: 148 passed, 0 failed. Two TCP tests initially could not bind sockets in the sandbox; the rerun with local socket access passed.
- Formatting checked for the two edited Rust files; `git diff --check` passed.

## Implementation Feedback

The separation between store selection, collection framing, and application codecs remains useful. Stable IDs, explicit big-endian encoding, tombstones, live counts, and dead-byte accounting are preserved for completed operations.

The collection now trusts index offsets. It no longer checks decoded IDs against index entries, reconciles metadata with scanned frames, checks exact index length, or verifies every slot against a live-frame map. Record lengths are read from trusted frames without checking them against physical file size before allocation. Basic format/version recognition, codec errors, I/O propagation, missing/deleted-ID checks, and arithmetic protection remain.

Deletion reads the indexed frame length without decoding its payload. Update encodes the replacement and reads only the old frame length. These operations no longer need `Decode` or `HasId` bounds; lookup and listing require `Decode` only.

Keep cursor movement inside the active `BufReader`. Its logical position can differ from the underlying file because it reads ahead. Keep explicit seeks between operations on persistent handles.

## Mutation Boundary

The write order remains explicit:

- Insert updates live count and next ID, appends a frame, then updates the index count and appends its entry.
- Delete updates dead bytes, tombstones the frame, decrements live count, then clears its index offset.
- Update increases dead bytes, tombstones the old frame, appends the replacement, then redirects the index entry.

Errors still propagate, but partial changes are not rolled back. The current contract does not guarantee detection or repair of interrupted writes on reopening. There is no explicit synchronization to stable storage, so completed writes are not a power-loss durability guarantee.

## Performance And Next Work

Listing scans data frames once and skips tombstoned payloads. There is no live-offset map, final index scan, or index lookup per live record. For linear codecs, work is O(frame count + live payload bytes); memory contains decoded live records and one temporary payload. ID lookup reads one index slot and one payload.

The small functional suite is not a benchmark for millions of records. Explicit file, payload, record-count, and decoded-memory limits remain unfinished Phase 09B work. Corruption and interrupted-write support should be reintroduced deliberately with regression tests when revisiting that scope.

Continue to [Phase 10](../phases/10-static-files-css.md) and return to [Phase 09B](../phases/09b-file-storage-limits.md) later. Keep compaction and Phase 09C performance work driven by measurements.
