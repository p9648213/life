# Reading Collections and Reporting Errors

Implement and test the read path before mutation.

## Read Sequence

```text
1. Resolve the validated collection path.
2. If it does not exist, return an empty collection.
3. Obtain metadata and reject an oversized file.
4. Read no more than the configured bound.
5. Validate magic bytes and format version.
6. Validate record count before reserving storage.
7. Validate each declared record payload length.
8. Give exactly that payload slice to T's decoder.
9. Require T's decoder to consume the entire payload.
10. Require the collection decoder to consume the entire file.
```

Do not use `create(true)` while listing. A read should not mutate the filesystem.
A missing file means empty data, while permission and other I/O failures remain
errors.

The collection format is binary, so do not use `read_to_string` for the complete
file. Only individually framed string fields require UTF-8 validation.

## Error Categories

Keep failures distinguishable enough to test and handle deliberately:

```text
invalid collection identifier
I/O operation and path
invalid magic
unsupported version
truncated input
declared length over limit
invalid UTF-8
trailing record or file bytes
ID exhaustion
replacement failure
```

Do not convert these errors to an empty vector. That makes corruption and
permission failures look like valid empty data.

## Linear Work

For a valid file of `F` bytes, decoding should take `O(F)` work. Use one
advancing cursor and avoid:

- rescanning from byte zero for every record;
- removing bytes from the front of a vector;
- repeatedly concatenating growing buffers;
- allocating before validating lengths;
- assuming individual limits automatically bound total allocation.

Add a maximum-size valid test. Prefer a deterministic cursor or byte-visit
invariant; at minimum, investigate unexpectedly slow scaling.

## Essential Tests

- Missing and valid empty files return empty records.
- Multiple records decode in order.
- Bad magic and unsupported versions fail.
- Truncated headers, fields, record frames, and payloads fail.
- Invalid UTF-8 and trailing bytes fail.
- Every declared limit is checked before allocation.
- Traversal and absolute collection identifiers fail.
- Maximum-size valid input has work proportional to its bytes.

Next: [Replacement and stable IDs](08-replacement-and-stable-ids.md).
