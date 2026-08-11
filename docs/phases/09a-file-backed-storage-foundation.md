# Phase 09A: File-Backed Storage Foundation

Goal: persist generic application record types through an explicit, bounded, and type-safe file-storage structure without an external serialization library.

The names below describe responsibilities, not mandatory APIs. Choose the concrete types, modules, method names, and ownership yourself.

## Design Structure

Keep these responsibilities separate:

```text
Store
  -> selects a validated collection under the storage root
Collection<T>
  -> owns file framing, limits, and persistent replacement
Encoder / Decoder
  -> encode and decode bounded primitive values
Encode / Decode for T
  -> define the stable field order for one record type
```

The application record type controls its field representation. The collection controls how records are separated and persisted. The store controls where collections may exist. Do not make an arbitrary `T` persistable by copying its in-memory representation.

Implement record codecs manually in this phase so the field order, byte representation, bounds checks, and error paths remain explicit while the storage format is being learned.

## Expected Behavior

A configured collection can encode, persist, reopen, and decode its record type. The file format is stable and self-identifying. Missing collection files represent empty data, while malformed, oversized, truncated, or unsupported data produces an explicit error.

Mutations use snapshot replacement: write the complete replacement to a temporary file on the same filesystem, flush and close it, then rename it over the destination. Success is reported only after replacement succeeds.

## Requirements

- Define a file header containing magic bytes and a format version.
- Choose and document one integer byte order.
- Frame variable-sized fields and records with checked lengths.
- Check configured limits before allocating or consuming a declared length.
- Require each record decoder to consume exactly its framed payload.
- Preserve stable IDs and derive a safe next ID after reopening.
- Validate collection identifiers so they cannot escape the storage root.
- Distinguish a missing file from permission, corruption, version, and other I/O errors.
- Keep decoding linear in the stored byte count; advance one cursor and do not repeatedly rescan growing input.
- Bound file size, record count, record size, field size, and decoded allocation.
- Use temporary-file replacement on the same filesystem for mutations.
- Document the atomicity boundary if one operation can affect multiple files.
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
- declared lengths over every configured limit are rejected before large allocation;
- collection traversal and absolute-path attempts are rejected;
- IDs remain valid after reopening;
- failed persistence is not reported as success;
- a maximum-size valid file completes with work proportional to its byte count.

Prefer a deterministic cursor or byte-visit invariant for the final runtime test. At minimum, investigate a maximum-size test that is unexpectedly slow instead of accepting it only because it finishes.

## Checkpoint

You are done when at least one application record type is encoded and decoded through the generic storage boundary, valid records survive reopening, corrupt or excessive input fails explicitly, parsing work is linear within configured limits, and a failed replacement cannot be reported as a successful mutation.

After this, continue with [Phase 10: Static Files and CSS](10-static-files-css.md). Revisit [Phase 09B: File-Storage Performance Optimization](09b-file-storage-performance-optimization.md) only after measurement identifies a storage bottleneck.
