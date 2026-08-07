# Mental Model

Phase 09A teaches the backend to convert an application record into stable
bytes, store multiple records in one bounded file, and reconstruct them after a
restart.

It is not SQL, a query language, automatic struct serialization, or a general
database engine.

## Write Flow

```text
Resource value
  -> Resource's Encode implementation
  -> primitive Encoder
  -> record payload bytes
  -> Collection<Resource> framing
  -> complete snapshot bytes
  -> temporary file
  -> destination file
```

## Read Flow

```text
collection file
  -> validate header and collection limits
  -> isolate one framed record payload
  -> primitive Decoder
  -> Resource's Decode implementation
  -> Resource value
```

## Responsibility Boundary

| Layer | Responsibility |
| --- | --- |
| `Store` | Validate a collection identifier and select a path below the storage root |
| `Collection<T>` | Own file framing, collection limits, loading, and persistent replacement |
| `Encoder` / `Decoder` | Convert bounded primitive values to and from bytes |
| `Encode` / `Decode` for `T` | Define the stable field order for one record type |

The short version is:

```text
Store answers:       Which safe collection file?
Collection<T>:       How are records framed and persisted there?
T's codec:           Which fields form one record?
Encoder/Decoder:     How does each primitive become bytes?
```

Keep application validation outside the generic storage codec. The codec can
decide whether bytes represent a valid `Resource` shape; the application decides
whether that resource is allowed by domain rules.

Next: [File format and byte order](02-file-format-and-byte-order.md).
