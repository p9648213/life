# Phase 09B: File-Storage Limits

Goal: place explicit size and work limits around the Phase 09A file-storage operations.

The limits, configuration shape, ownership, and API names are your design choices. Keep the phase focused on bounding existing storage behavior rather than changing the persistent model or optimizing an unmeasured bottleneck.

## Expected Behavior

Valid collections within the chosen limits retain the Phase 09A behavior. Excessive files, record counts, record payloads, and fields are rejected explicitly before excessive allocation, iteration, disk growth, or decoded-state growth occurs.

The same limits apply consistently when writing, reopening, listing, and mutating a collection.

## Requirements

- Define maximum collection-file size, record count, record-payload size, and field size.
- Bound total decoded allocation either directly or as a documented consequence of the other limits.
- Check declared record and field lengths before allocating or consuming them.
- Reject an insertion or update that would cross a limit before reporting success.
- Use checked arithmetic when calculating projected file size, counts, lengths, and allocation.
- Keep traversal linear in the permitted stored byte count; examine each input byte only a bounded number of times.
- Preserve the Phase 09A file format and direct-mutation behavior unless a deliberate version change is documented.
- Keep application-domain validation separate from generic storage safety limits.

## Tests to Write

- just-below, exact-limit, and just-above cases for every configured limit;
- excessive declared record and field lengths are rejected before large allocation;
- excessive record count is rejected before large iteration or decoded collection allocation;
- an oversized existing collection is rejected explicitly on reopen;
- an insertion or update that would cross a limit fails without being reported as successful;
- valid data at the configured boundaries still round-trips and survives reopening;
- a maximum-size valid collection completes with work proportional to its byte count.

Prefer a deterministic cursor, byte-visit, allocation, or work-count invariant for the final runtime test. At minimum, investigate a maximum-size test that is unexpectedly slow.

## Checkpoint

You are done when every storage limit has explicit boundary tests, excessive declarations fail before expensive work, valid boundary-sized collections retain Phase 09A behavior, and maximum permitted work scales linearly with the configured limits.

After this, continue with [Phase 10: Static Files and CSS](10-static-files-css.md). Revisit [Phase 09C: File-Storage Performance Optimization](09c-file-storage-performance-optimization.md) only after measurement identifies a storage bottleneck.
