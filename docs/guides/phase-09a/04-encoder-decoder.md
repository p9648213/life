# Encoder and Decoder

`Encoder` and `Decoder` are project-owned helpers, not built-in Rust types.
They belong in the generic codec layer, for example `src/storage/codec.rs`.

## Encoder

An encoder converts primitive Rust values into bytes using the documented
format. Conceptually it contains:

```text
output bytes
maximum permitted output size
```

It should provide primitive operations such as:

```text
write_u32
write_u64
write_string
```

Yes, `write_u32` should be an `Encoder` method. It should:

1. Check with `checked_add` that four more bytes fit the encoder limit.
2. Convert the value to the documented byte order.
3. Append all four bytes.
4. Leave the buffer unchanged if the size check fails.

`write_string` should:

1. Obtain the UTF-8 bytes.
2. Check the field-size limit.
3. Convert the byte length safely to the stored integer type.
4. Encode that length.
5. Check and append the exact bytes.

Use a checked conversion such as `u32::try_from(length)` when lengths are stored
as `u32`; an `as` cast can silently truncate an oversized value.

## Who Creates the Encoder?

The collection creates a short-lived record encoder, configured with the record
limit, then passes it to `T`:

```text
Collection<T>
  -> create Encoder(maximum record payload size)
  -> item.encode(&mut encoder)
  -> take completed record payload bytes
  -> add collection-level record framing
```

The record should not create the encoder because the record should not choose
the collection's limits.

## Decoder

A decoder conceptually contains:

```text
borrowed input byte slice
current cursor position
limits or remaining allocation budget
```

Primitive methods such as `read_u32` and `read_string` validate the remaining
input and advance the cursor once.

For a declared string length:

1. Read the fixed-width length.
2. Convert it to `usize` with a checked conversion.
3. Reject it if it exceeds the field limit.
4. Reject it if the record payload has too few remaining bytes.
5. Account for the decoded allocation.
6. Only then copy or allocate.
7. Validate UTF-8.

Do not repeatedly search from the beginning or remove bytes from the front of a
vector. One advancing cursor keeps decoding linear in the input size.

## First Tests

- A known `u32` produces the documented bytes.
- Zero and maximum integers round-trip.
- Truncated integer input fails.
- Writing exactly to the output limit succeeds.
- A write beyond the limit fails without partially changing the buffer.
- Empty, Unicode, newline, and delimiter-containing strings round-trip.
- Invalid UTF-8 and oversized declared lengths fail.

Next: [Storage limits](05-storage-limits.md).
