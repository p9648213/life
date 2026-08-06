# Phase 09A Learning Guide: Building File-Backed Storage

Use this guide together with
[Phase 09A: File-Backed Storage Foundation](../phases/09a-file-backed-storage-foundation.md).
The phase file defines the contract. This guide explains the mental model and a
small implementation sequence.

This is intentionally not a finished implementation. The type names and
pseudocode illustrate responsibilities; choose the concrete modules, types,
method names, ownership, and APIs yourself.

## 1. The Goal in Plain Language

Phase 09A teaches the backend to convert an application record into stable
bytes, store multiple records in one bounded file, and reconstruct the same
records after restarting.

For one record, the flow is:

```text
Resource value
  -> Resource encoder
  -> primitive encoder
  -> record payload bytes
  -> collection framing
  -> complete file bytes
  -> temporary file
  -> destination file
```

Reading performs the reverse flow:

```text
collection file
  -> validate file header and limits
  -> isolate one framed record payload
  -> primitive decoder
  -> Resource decoder
  -> Resource value
```

This phase is not SQL, a query language, automatic struct serialization, or a
general database engine.

## 2. Responsibility Boundaries

Keep four decisions separate:

| Layer | Knows | Does not know |
| --- | --- | --- |
| `Store` | storage root and valid collection identifiers | fields inside a record |
| `Collection<T>` | file header, record framing, limits, and replacement | application validation rules |
| encoder/decoder | integer byte order and bounded primitive values | what a `Resource` means |
| codec for `T` | stable field order for that record type | destination paths or file replacement |

A generic method such as `list<T>` cannot invent values of `T`. It needs a
contract saying that `T` knows how to decode itself from the primitive decoder.
Likewise, insertion needs `T` to know how to encode itself.

Conceptually, those capabilities could look like:

```text
Encode:
    encode self into a primitive encoder, or return an error

Decode:
    construct Self from a primitive decoder, or return an error
```

These are responsibility sketches, not required trait signatures.

## 3. Understand the File as a Protocol

Treat the collection file like a small network protocol. Once data has been
written, future versions of the program must interpret the bytes the same way.
That means every value needs an explicit representation.

One possible format is:

```text
FILE HEADER
  magic bytes          fixed width
  format version       fixed-width integer
  record count         fixed-width integer

REPEATED RECORD FRAME
  payload length       fixed-width integer
  payload              exactly payload-length bytes
```

A possible `Resource` payload is:

```text
id                     fixed-width integer
name byte length       fixed-width integer
name                    UTF-8 bytes
number                 fixed-width integer
```

This is an example, not a mandatory layout. Whichever layout you choose, write
it down before implementing it.

Length-prefixing a string makes empty strings, Unicode, newlines, commas, and
other delimiter characters ordinary data. No delimiter escaping is needed.

### Format decisions to record

Before writing codec code, answer these questions in a comment or small design
note:

- What are the magic bytes?
- What is the first supported format version?
- Are integers little-endian or big-endian?
- How wide is each stored integer?
- Is the number of records stored in the header or detected at end-of-file?
- How is each record payload framed?
- In what order does each concrete record encode its fields?
- Are collection filenames given an extension?

Do not write raw Rust struct memory. Struct layout can include padding and can
change across compilers, targets, or source changes.

## 4. Choose Limits Before Reading Data

Every length read from a file is untrusted. A corrupt file can claim that its
next field is several gigabytes long.

Choose explicit bounds for at least:

```text
maximum collection file size
maximum record count
maximum encoded record size
maximum encoded field size
maximum total decoded allocation
```

The exact numbers belong to your application and can change later. What matters
now is that each limit exists and is checked at the correct boundary.

For a declared string length, the safe order is:

```text
1. Read the fixed-width length.
2. Convert it to the platform index type with a checked conversion.
3. Reject it if it exceeds the field limit.
4. Reject it if it exceeds the remaining record payload.
5. Account for it in the total allocation budget.
6. Only then copy or allocate.
7. Validate the bytes as UTF-8.
```

Never allocate based only on a length found in the file.

## 5. Decode With One Advancing Cursor

A decoder can be understood as:

```text
input: borrowed byte slice
position: current offset
limits or remaining allocation budget
```

Reading a value checks the bytes beginning at `position`, then advances
`position` once. Record decoding should not repeatedly search the entire input,
remove bytes from the front of a vector, or recreate progressively smaller
owned buffers.

With one cursor, decoding a file of `F` bytes should take `O(F)` work. Each byte
is inspected a bounded number of times. Memory remains bounded by the configured
file and decoded-allocation limits.

Record framing creates an important boundary:

```text
collection decoder
  -> reads record length
  -> gives exactly that payload slice to T's decoder
  -> requires T's decoder to finish at the end of the slice
```

If the concrete decoder leaves bytes behind, either the field layout is wrong
or the file contains unsupported data. Reject it instead of silently ignoring
those bytes.

## 6. Step-by-Step Implementation Path

Complete and test one step before moving to the next.

### Step 1: Specify the format and limits

Write the decisions from Sections 3 and 4. At this point there should be no
file I/O and no generic collection code.

Checkpoint:

- You can draw every byte needed for an empty file and one concrete record.
- Every variable-sized region has a length frame.
- Every relevant size has a configured maximum.

### Step 2: Expand the storage error vocabulary

Represent failures that callers may need to distinguish. Think in categories
such as:

```text
invalid collection identifier
I/O operation and path
invalid magic
unsupported version
truncated input
declared length over limit
invalid UTF-8
invalid record payload or trailing bytes
ID exhaustion
replacement failure
```

Do not flatten all failures into a connection or read string. Missing data is
special only when loading a collection: a missing collection file means an
empty collection. Permission errors and other I/O failures remain errors.

Checkpoint:

- A test can distinguish corrupt data, unsupported data, and an I/O failure.

### Step 3: Encode and decode fixed-width integers

Start with the integer types your format actually needs. Use the standard
library byte-order conversion methods corresponding to your documented byte
order.

Tests:

- zero round-trips;
- a nontrivial value round-trips;
- the maximum value round-trips;
- too few bytes produces a truncation error;
- the encoded bytes prove the selected byte order.

Checkpoint:

- Encoding and decoding primitives does not depend on `Resource` or files.

### Step 4: Encode and decode bounded strings

Encode the UTF-8 byte length followed by the exact UTF-8 bytes. Decode by
following the safe length-check order from Section 4.

Tests:

- empty string;
- ASCII text;
- Unicode text;
- newline and delimiter-containing text;
- invalid UTF-8;
- truncated string bytes;
- declared length over the field limit, rejected before allocation.

Checkpoint:

- All string cases operate only on in-memory byte buffers.

### Step 5: Give one record type a manual codec

Choose one application record, such as the current `Resource`. Decide its
persistent fields and stable field order. Encode each field through the
primitive encoder and decode them in exactly the same order.

Keep domain validation outside this codec. The codec answers “are these valid
bytes for the stored representation?” Application code answers questions such
as “is this resource name allowed?”

Tests:

- one record round-trips;
- empty and Unicode names round-trip;
- truncation at each field boundary fails;
- extra bytes after the expected fields fail.

Checkpoint:

- `decode(encode(value))` produces the original value.
- The record codec has no knowledge of file paths.

### Step 6: Validate collection identifiers

Make `Store` turn an identifier into a path under its configured root. Define a
narrow identifier grammar rather than trying to clean up arbitrary paths.

At minimum, reject:

```text
empty identifiers
absolute paths
parent-directory components
path separators
identifiers outside the chosen character or length rules
```

Tests:

- one ordinary identifier is accepted;
- `../secret` is rejected;
- nested traversal is rejected;
- absolute paths are rejected;
- boundary-length identifiers behave as documented.

Checkpoint:

- An accepted collection can never resolve outside the storage root.

### Step 7: Decode a complete collection

Implement the read path before mutation:

```text
1. Resolve the validated collection path.
2. If the path does not exist, return an empty collection.
3. Obtain file metadata and reject an oversized file.
4. Read no more than the configured bound.
5. Validate magic bytes and version.
6. Validate record count before reserving storage.
7. For each record, validate its declared payload length.
8. Give exactly that payload to the concrete record decoder.
9. Require exact payload consumption.
10. Require exact file consumption.
```

Do not open a missing collection with `create(true)` during a read. That mixes
observation with mutation and makes missing-file behavior harder to reason
about.

Tests:

- missing file returns an empty vector;
- a valid empty file returns an empty vector;
- several records decode in order;
- bad magic and unsupported version fail;
- truncated header, frame, and payload fail;
- extra bytes at the end of the file fail;
- every declared length limit is enforced before allocation.

Checkpoint:

- A fresh storage instance can reopen a valid file created by a test fixture.

### Step 8: Define stable ID behavior

Choose where persistent identity belongs: directly on the application record or
in a stored-record wrapper. Both designs can work; make the boundary explicit.

After reopening, derive a safe next ID from stored IDs. Do not use record count
plus one because deletion can make that reuse an existing ID.

Conceptually:

```text
next ID = checked(maximum stored ID + 1)
```

Also decide how an empty collection begins and what happens at the maximum ID.

Tests:

- IDs survive reopening;
- deletion does not cause an existing ID to be reused;
- an unsorted file still produces a safe next ID;
- maximum-ID exhaustion returns an error.

Checkpoint:

- Restarting the process cannot change the identity of an existing record.

### Step 9: Encode a complete collection snapshot

Construct the complete replacement bytes in the documented format:

```text
header
for every record:
    encode record into a bounded payload
    check payload size
    write payload length
    write payload
check total encoded file size
```

Check growth while encoding, not only after creating an oversized buffer.

Tests:

- zero, one, and several records produce decodable snapshots;
- a record over its limit is rejected;
- a collection over its count or file-size limit is rejected;
- encoded snapshots have the documented header and byte order.

Checkpoint:

- The collection encoder and decoder round-trip completely in memory.

### Step 10: Persist by temporary-file replacement

Mutation uses snapshot replacement:

```text
1. Build and validate the complete replacement snapshot.
2. Create a uniquely named temporary file in the destination directory.
3. Write the complete snapshot.
4. Flush the file and handle the result.
5. Close the file and handle relevant failures.
6. Rename it over the destination.
7. Only after rename succeeds, report mutation success.
```

The temporary file must be on the same filesystem as the destination so the
rename has the intended atomic replacement boundary. Clean up a leftover
temporary file when practical, but never remove the valid destination merely
because replacement failed.

Atomic replacement means readers see the old complete snapshot or the new
complete snapshot. It does not make changes across multiple collection files
atomic, and it is not by itself a complete power-loss durability guarantee.

Tests:

- saved records survive constructing a new storage instance;
- replacement failure is returned as an error;
- the previous valid destination remains usable when failure occurs before
  rename;
- success is not returned before replacement completes.

Checkpoint:

- A failed write cannot turn an in-memory intention into reported success.

### Step 11: Add typed collection operations

Now connect the pieces into the smallest operations the application needs, for
example listing and inserting records. Prefer typed operations over parsing
SQL-like strings.

An insertion conceptually performs:

```text
load existing records
derive and assign a stable ID
apply the in-memory change
encode the replacement snapshot
persist the replacement snapshot
return success only after replacement
```

Returning `Result` already communicates success or failure; consider whether a
separate success boolean would convey any additional state.

Checkpoint:

- The generic boundary explains why a type is encodable and decodable.
- The application does not manually manipulate collection-file bytes.

### Step 12: Integrate without hiding errors

Only after the storage boundary works in isolated tests should handlers call it.
Map storage failures deliberately. Do not convert every error to an empty list
or ignore insertion results.

The temporary resource endpoints are scaffolding for exercising persistence;
they do not define the backend's application domain.

Checkpoint:

- A handler cannot return a successful mutation response after persistence
  failed.
- Corruption is observable rather than displayed as an empty collection.

### Step 13: Audit worst-case work

For a valid file of `F` bytes and `R` records, write down the cost of the read
path. The target is:

```text
time: O(F)
memory: bounded by configured file and decoded-allocation limits
```

Look specifically for:

- rescanning from byte zero for every record;
- removing bytes from the front of a vector;
- repeatedly concatenating growing buffers;
- cloning complete record payloads more than required;
- allocating before validating declared lengths;
- `record_count * maximum_record_size` exceeding the total allocation bound.

Add a maximum-size valid fixture. Prefer a deterministic cursor or byte-visit
invariant that proves forward progress. At minimum, compare scaled fixtures and
investigate surprising growth instead of accepting a slow test merely because
it eventually passes.

Checkpoint:

- Every input byte is examined a bounded number of times.
- Raising any limit triggers a new time and memory cost review.

## 7. Mapping This Guide to the Current Prototype

The current storage prototype already has the outer idea of a storage root and
generic `list`/`insert_one` operations. The next work is not to fill those
methods immediately. Their inner dependencies should be built and tested first.

Current behavior to replace deliberately:

- `read_to_string` assumes the entire file is UTF-8, but the collection format
  is binary;
- an unbounded read trusts the file size;
- joining an unchecked collection name permits path escape attempts;
- creating a file while listing mixes read and mutation behavior;
- unconstrained `T` provides no way to encode or decode a record;
- ignoring the inserted value and returning success violates the persistence
  contract;
- defaulting a storage error to an empty list hides corruption and permissions
  failures.

Build the codec and collection tests below the HTTP layer first. The handler
integration should become a small final step rather than the place where binary
format bugs are debugged.

## 8. Phase Completion Checklist

- [ ] The binary format and byte order are documented.
- [ ] File, record, field, count, and decoded-allocation limits exist.
- [ ] Primitive integer and string codecs pass focused tests.
- [ ] One concrete application record has a manual codec.
- [ ] A decoder uses one advancing cursor.
- [ ] Record and file payloads must be consumed exactly.
- [ ] Collection identifiers cannot escape the storage root.
- [ ] Missing collection files mean empty data.
- [ ] Corrupt, unsupported, excessive, and inaccessible files return explicit
      errors.
- [ ] Stable IDs survive reopening and cannot overflow silently.
- [ ] Mutations use same-filesystem temporary-file replacement.
- [ ] Failed persistence is never reported as success.
- [ ] Multiple valid records survive a fresh storage instance.
- [ ] Maximum-size parsing has work proportional to stored bytes.

When every item is demonstrated by a focused test, return to the Phase 09A
checkpoint before continuing to Phase 10.
