# Phase 09A: File-Backed Storage Foundation

Goal: persist generic application record types through an explicit and type-safe file-storage structure without an external serialization library.

The names below describe responsibilities, not mandatory APIs. Choose the concrete types, modules, method names, and ownership yourself.

## Design Structure

Keep these responsibilities separate:

```text
Store
  -> selects a validated collection under the storage root
Collection<T>
  -> owns file framing, validation, and persistent mutation
Encoder / Decoder
  -> encode and decode primitive values
Encode / Decode for T
  -> define the stable field order for one record type
```

The application record type controls its field representation. The collection controls how records are separated and persisted. The store controls where collections may exist. Do not make an arbitrary `T` persistable by copying its in-memory representation.

Implement record codecs manually in this phase so the field order, byte representation, length checks, and error paths remain explicit while the storage format is being learned.

The current collection format uses big-endian multi-byte integers:

```text
collection header
[magic][version][next_id: u32][live_record_count: u32][dead_bytes: u64]

record frame
[state: u8][payload_length: u32][payload]
```

A state byte of `1` marks a live frame and `0` marks a tombstoned frame. `live_record_count` counts only live frames. `dead_bytes` is the total byte length occupied by tombstoned frames, including each frame's state byte, length prefix, and payload.

## Expected Behavior

A configured collection can encode, persist, reopen, list, and find records by ID. Its format is stable and self-identifying. Missing collection files return an error; creating a collection is explicit.

For this learning checkpoint, assume both the collection and index files contain valid data produced by this storage implementation, codecs match the stored record layout, and every mutation completes all required writes without interruption. Operations through multiple handles are sequential; concurrent mutation is outside this contract.

Manual file corruption, invalid index offsets, mismatched metadata, partial writes, and crash recovery are deferred. Reopening and listing do not promise to detect these conditions. Basic magic/version checks and ordinary I/O and codec errors remain, but they are not comprehensive file validation. I/O errors still propagate rather than being reported as success; an error after a write can leave partial changes, with no rollback or recovery guarantee. Completed writes do not imply power-loss durability.

Insertion appends a live frame. Deletion clears the record's index location and marks its frame as tombstoned without shrinking the collection file. Update preserves the ID by directing its index entry to an appended replacement frame and tombstoning the previous frame. Deletion and update add the full replaced frame length to `dead_bytes`. Reclaiming or reusing tombstoned space is deferred to Phase 09C and requires measurement.

## Requirements

- Define the collection header, index slots, and record framing explicitly; encode multi-byte integers in big-endian order.
- Encode and decode records manually, with checked field lengths and ordinary codec error propagation.
- Preserve stable IDs, live counts, and dead-byte accounting across completed insertions, updates, deletions, and reopening.
- Reject missing or deleted IDs, including index offset zero, in targeted operations.
- Keep checked ID/count/dead-byte arithmetic and wide index-position calculations.
- Validate collection identifiers so they cannot escape the storage root.
- Retain basic format/version recognition and propagate file-opening and I/O errors.
- Keep cursor movement explicit; seek through the active buffered reader when scanning.
- Keep domain validation outside the generic storage codec.
- Do not use raw struct memory, pointers, padding bytes, or platform-dependent layout as the persistent format.

## Tests

The simplified storage suite retains 38 tests:

- 22 tests for normal operations, codecs, stable IDs, collection paths, and linear decoding;
- 7 tests for arithmetic overflow and large index positions;
- 3 tests for missing files and invalid file paths;
- 6 tests for unsupported file versions.

The 45 corruption and interrupted-write tests were deleted for this checkpoint. Some retained arithmetic tests still construct artificial header values or sparse files to exercise numeric boundaries without creating billions of records; this does not establish general corruption handling.

Keep normal deleted-ID lookup coverage. An index deliberately redirected to another record is outside the current contract.

## Performance And Deferred Work

Listing makes one pass over data frames, skips tombstoned payloads, and decodes each live payload once. It does not build a live-offset map, perform per-record index lookups, or rescan the index. With linear codecs, work is O(frame count + live payload bytes); memory holds the decoded live records and one temporary payload. A targeted lookup reads one index slot and one payload, with O(payload size) work and memory for a linear codec.

Explicit file, payload, record-count, and decoded-memory limits remain unfinished Phase 09B work. Corruption validation and interrupted-write handling are separate deferred work to revisit later; they are not completed by adopting these assumptions. Phase 09C optimizations still require measurement.

## Checkpoint

You are done with this simplified checkpoint when valid records survive reopening, normal lookup and completed mutations preserve IDs and unaffected records, metadata is updated correctly, and the retained tests pass. This does not establish production readiness or safe handling of damaged files.

Continue to [Phase 10: Static Files and CSS](10-static-files-css.md) as the chosen learning order. Return to [Phase 09B: File-Storage Limits](09b-file-storage-limits.md) later, and revisit [Phase 09C: File-Storage Performance Optimization](09c-file-storage-performance-optimization.md) after measurement identifies a bottleneck.
