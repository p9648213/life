# Storage Limits

Limits are correctness and availability rules. They are not performance
optimizations and should not be postponed until after collection file I/O.

## Required Boundaries

One limits configuration can begin with:

```text
maximum collection file size
maximum record count
maximum record payload size
maximum variable-sized field size
maximum total decoded allocation
```

The values are policies you choose and document. They are not supplied by Rust.
Start with centralized constants or one small configuration type; runtime
configuration can wait until Phase 21.

## What `max_size` Means

An encoder's `max_size` is the maximum number of bytes that particular encoder
may produce. It is not the vector's current capacity.

When encoding one record:

```text
record Encoder max_size = maximum record payload size
```

When building the complete file:

```text
snapshot output limit = maximum collection file size
```

Document whether the record-size limit covers only the payload or also its
length prefix. A simple policy is:

```text
record limit covers payload
file limit covers header + every length prefix + every payload
```

The decoder must use the same definition.

## Nested Limits

```text
collection file                         file-size limit
├── header
├── record length
└── record payload                      record-size limit
    ├── id
    ├── name length
    └── name UTF-8 bytes                field-size limit
```

Many valid fields can still exceed one record, and many valid records can still
exceed one file. Check every boundary independently.

## Can Limits Wait?

Only during a tiny fixed-width experiment. Writing and reading one `u32` is
always four bytes, so you can first learn the byte-order methods in isolation.

Before adding strings, add field and encoder-output limits. Before reading a
collection file, add all collection and decoded-allocation limits.

Without them, corrupt input can claim a huge string or record count and trigger
an excessive allocation before the program discovers the file is invalid.

## Check Order

For every declared variable length:

```text
read length
-> checked conversion
-> relevant configured-limit check
-> remaining-input check
-> total-allocation accounting
-> allocation or copy
```

Tests must prove an over-limit declaration is rejected before a large
allocation is attempted.

Next: [Store and Collection](06-store-and-collection.md).
