# File Format and Byte Order

Treat a collection file like a small protocol. Future versions of the program
must interpret previously written bytes in the same way.

## Example Layout

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
name                   UTF-8 bytes
number                 fixed-width integer
```

This is an example, not a mandatory layout. Document your chosen magic bytes,
version, integer widths, field order, and record framing before implementing it.

Do not persist raw struct memory. Rust struct layout can contain padding and can
change across compilers, targets, or source changes.

## Big-Endian and Little-Endian

For the `u32` value `0x12345678`, the four component bytes are `12 34 56 78`.

Big-endian stores the most significant byte first:

```text
12 34 56 78
```

Little-endian stores the least significant byte first:

```text
78 56 34 12
```

Rust exposes both representations:

```text
to_be_bytes / from_be_bytes    big-endian
to_le_bytes / from_le_bytes    little-endian
```

Either choice works for Phase 09A. Choose one, document it, and use it for every
stored multi-byte integer. Avoid native-endian storage because it can depend on
the machine.

Endianness does not reverse decimal digits and does not affect `u8`, which is
only one byte.

## Length-Prefixed Values

Store a string as:

```text
UTF-8 byte length + exact UTF-8 bytes
```

This allows empty strings, Unicode, newlines, commas, and delimiter characters
without escaping. The declared length must be validated before allocation or
consumption.

Next: [Codec traits](03-codec-traits.md).
