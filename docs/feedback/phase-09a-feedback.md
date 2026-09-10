# Phase 09A Feedback

## Overall

You completed the main file-storage foundation: application records can be manually encoded, persisted, reopened, listed, updated, and deleted through a generic collection boundary. The implementation uses the standard library without a serialization or database dependency.

The review covered the storage layer, its codecs, and its tests. It does not establish production readiness or validate the entire backend.

## Verified Results

- `cargo test --locked --test storage`: 81 passed, 0 failed; the latest run finished in 0.02 seconds after compilation.
- `git diff --check`: passed.
- Separate diagnostic tests for a wrong ID in a cleared index slot and maximum index-count overflow passed after their fixes. These two diagnostics were run outside the repository test suite; they are not included in the 81 tests.
- Regression coverage includes persistence across reopening, stable IDs, update/delete preservation, malformed framing, payload bounds, metadata inconsistencies, and several interrupted-write states.
- Index-offset regressions reject a deleted ID pointing beyond EOF, inside a live payload, or to another ID's live frame.

The short suite runtime is functional-test evidence, not a benchmark for millions of records. Full-workspace tests, formatting, and Clippy are not claimed as part of this storage review.

## What You Did Well

- `Store` selects validated collection names under a configured root; collection framing remains separate from application record encoding.
- Primitive codecs use explicit big-endian bytes and checked string lengths. Live record decoding must consume exactly its framed payload.
- Frame lengths are checked against physically remaining file bytes before payload allocation or skipping, including tombstoned frames.
- Insertions append records; updates preserve IDs through replacement frames; deletions retain tombstones. Live count and dead-byte accounting are checked against the scanned frames.
- Mutations check target IDs and offsets, and arithmetic validation now precedes the relevant writes in the reviewed overflow cases.
- Index length is checked exactly with checked `u64` arithmetic. Index-count increment overflow returns an error rather than panicking.
- The final index scan checks both slot IDs and offsets against observed live frames. Cleared entries require no live frame; nonzero entries require a matching live frame.
- Tests construct isolated files and introduce specific corruptions. Byte-preservation assertions distinguish a rejected operation from one that reports an error after modifying files.

## Important Lessons From The Review

An offset inside the file is not necessarily a frame boundary. Recording actual live-frame positions and checking index entries against them closes that gap without decoding payloads twice.

A cleared index entry is different from a missing or malformed entry. Returning offset zero from the lookup lets each caller apply the appropriate rule: listing may accept a deleted record, while update and delete reject an already-cleared entry.

Keep cursor movement inside the active `BufReader`. Its logical position can differ from the underlying file position because it reads ahead. Seeking through the underlying file while buffered reads continue caused valid files to be reported as truncated.

Validation errors should be discovered before the first write whenever the necessary information is already available. A checked addition is insufficient if another file has already been modified before that check runs.

## Mutation And Recovery Boundary

The current write order is explicit but not atomic:

- Insert updates the collection's live count and next ID, appends the frame, then updates the index count and appends its entry.
- Delete updates dead bytes, tombstones the frame, decrements the live count, then clears the index offset.
- Update increases dead bytes, tombstones the old frame, appends the replacement, then redirects the index entry.

Interruption can leave mismatched metadata, incomplete frames, or stale index entries. The tests cover several such states being rejected when reopening and listing. Opening a handle alone does not perform the full scan. Automatic repair and atomic rollback are not provided, and completed writes are not a power-loss durability guarantee; there is no explicit synchronization to stable storage.

## Performance And Deferred Work

Listing scans collection frames and then index entries. For linear record codecs, expected work scales with stored bytes and entry count; these are consecutive loops, not a nested scan. Memory includes all decoded live records, a live-offset map, and a temporary payload buffer. Tombstoned payloads are bounds-checked and skipped.

The implementation still performs a direct index lookup for each live frame before the final buffered index scan. Measure those repeated seeks and reads alongside map allocation, elapsed time, and peak memory before changing the algorithm. If removing those lookups later, preserve duplicate-live-ID detection and the existing consistency guarantees.

Phase 09B remains unfinished: explicit file, record-count, payload, field, and decoded-memory limits still need boundary tests. Large physically valid files can therefore consume substantial memory and work. The passing suite does not demonstrate bounded behavior at a chosen maximum collection size.

Keep compaction, scratch-buffer reuse, and other Phase 09C optimizations driven by measured workloads. Full index validation remains part of the design; the previously considered corruption exception was withdrawn.

## Next Step

You chose to continue to [Phase 10: Static Files and CSS](../phases/10-static-files-css.md) and return to [Phase 09B: File-Storage Limits](../phases/09b-file-storage-limits.md) later. This is a deliberate change in learning order, not completion of 09B. Carry the storage limits forward as unfinished work while keeping the Phase 10 exercise focused on static-file behavior.
