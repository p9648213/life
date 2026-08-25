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

## Expected Behavior

A configured collection can encode, persist, reopen, and decode its record type. The file format is stable and self-identifying. Missing collection files represent empty data, while malformed, truncated, or unsupported data produces an explicit error.

Mutations modify the configured collection file directly. Success is reported only after the required writes complete successfully. This phase does not promise atomic recovery from interruption during a mutation; reopening must report any resulting malformed or truncated file explicitly rather than silently accepting partial data.

## Requirements

- Define a file header containing magic bytes and a format version.
- Choose and document one integer byte order.
- Frame variable-sized fields and records with checked lengths.
- Reject a declared length that does not fit within the bytes physically remaining in its frame or file before allocating or consuming it.
- Require each record decoder to consume exactly its framed payload.
- Preserve stable IDs and derive a safe next ID after reopening.
- Validate collection identifiers so they cannot escape the storage root.
- Distinguish a missing file from permission, corruption, version, and other I/O errors.
- Modify one collection file directly for each mutation.
- Document the order of direct file changes and which interrupted states may remain readable, malformed, or truncated.
- Keep domain validation outside the generic storage codec.
- Do not use raw struct memory, pointers, padding bytes, or platform-dependent layout as the persistent format.

## Tests to Write

- primitive integers round-trip with the documented byte order;
- empty, Unicode, delimiter-containing, and newline-containing strings round-trip;
- one concrete record type round-trips through its manual codec;
- multiple records survive constructing a new storage instance;
- missing collection files read as empty data;
- invalid magic bytes and unsupported versions are rejected;
- malformed lengths, invalid UTF-8, truncated fields, truncated records, and trailing record bytes are rejected;
- collection traversal and absolute-path attempts are rejected;
- IDs remain valid after reopening;
- failed persistence is not reported as success;
- direct insertion, update, and deletion preserve every unaffected record when the writes complete successfully;
- malformed or truncated states left by an interrupted direct mutation are rejected on reopening;

## Checkpoint

You are done when at least one application record type is encoded and decoded through the generic storage boundary, valid records survive reopening, corrupt input fails explicitly, direct insertion, update, and deletion preserve unaffected records, and a failed direct mutation cannot be reported as successful.

After this, continue with [Phase 09B: File-Storage Limits](09b-file-storage-limits.md). Revisit [Phase 09C: File-Storage Performance Optimization](09c-file-storage-performance-optimization.md) only after measurement identifies a storage bottleneck.
