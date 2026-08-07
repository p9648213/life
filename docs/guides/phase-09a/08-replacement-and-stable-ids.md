# Snapshot Replacement and Stable IDs

## Stable IDs

Choose whether persistent identity lives directly on the application record or
in a stored-record wrapper. Make the boundary explicit.

After reopening, derive a safe next ID from the maximum stored ID:

```text
next ID = checked(maximum stored ID + 1)
```

Do not use record count plus one. Deletion could then reuse an existing ID.
Define the empty-collection starting ID and return an explicit error at maximum
ID exhaustion.

Tests should cover restart, deletion, unsorted stored records, and exhaustion.

## Encode a Complete Snapshot

```text
write header
for every record:
  encode a bounded record payload
  check payload size
  write payload length
  write payload
check complete file size while growing output
```

Check growth during encoding, not only after constructing an oversized buffer.

## Replace Instead of Overwrite

Mutation uses this sequence:

```text
1. Build and validate the complete replacement snapshot.
2. Create a unique temporary file in the destination directory.
3. Write the complete snapshot.
4. Flush and handle the result.
5. Close the file.
6. Rename it over the destination.
7. Only now report success.
```

The temporary file must be on the same filesystem as the destination so rename
has the intended atomic replacement boundary. Failure before rename must leave
the previous valid destination usable.

Atomic replacement means readers see the old complete snapshot or the new
complete snapshot. It does not make changes across multiple files atomic and is
not, by itself, a complete power-loss durability guarantee.

## Typed Mutation Flow

An insertion conceptually performs:

```text
load records
derive and assign stable ID
apply in-memory change
encode complete replacement
persist replacement
return success only after rename
```

A `Result` already represents success or failure; add a separate boolean only
if it communicates another real outcome.

Tests must prove that replacement failure is returned, the old snapshot remains
usable when failure happens before rename, and success is never reported early.

Next: [Step-by-step path](09-step-by-step.md).
