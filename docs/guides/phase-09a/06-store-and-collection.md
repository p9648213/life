# Store and Collection

The relationship is:

```text
Store creates a validated typed collection handle.
Collection<T> performs typed record operations on that selected file.
```

## Why `Collection<T>`?

With only a generic method and a string, callers could ask for two incompatible
types from the same file:

```text
list<Resource>("resources")
list<User>("resources")
```

A typed handle chooses the relationship once:

```text
store.collection<Resource>("resources")
    -> Collection<Resource>
    -> list() returns Resource records
    -> insert() accepts a Resource
```

This makes accidental type mixing harder. `Collection<T>` remains one possible
API, not a phase requirement.

`T` supplies the record codec type. It does not automatically supply a filename,
collection name, field names, or runtime schema.

## `PhantomData<T>`

A collection can be logically associated with `T` without storing a value of
`T`. Its runtime fields may only be:

```text
validated path
storage limits
```

Rust rejects an unused generic type parameter. `PhantomData<T>` is a zero-sized
compile-time marker that says the collection is logically associated with `T`.
It stores no `Resource`, performs no allocation, and writes nothing to disk.

You do not need `PhantomData` when only an individual method is generic. You
need it when the collection struct itself carries the type relationship.

## How Store Constructs a Collection

The caller selects the record type and runtime collection identifier:

```text
store.collection<Resource>("resources")
```

The construction flow is:

```text
untrusted "resources"
  -> Store validates the identifier
  -> Store resolves a path beneath its root
  -> restricted Collection constructor receives that trusted path and limits
  -> Collection<Resource>
```

`Store` does not construct a `Resource` value and does not import `Resource`.
The generic caller supplies the compile-time type.

Prefer an owned resolved path in the first design. Then the collection does not
borrow `Store`, avoiding an extra lifetime relationship. Copying or cloning a
small limits value is acceptable for this phase.

## Constructor Boundary

Keep the direct collection constructor private to the storage layer. Public
application code should go through `Store`, establishing this invariant:

> If a collection handle exists, its path has already passed Store validation.

Use a narrow collection-identifier grammar. Reject empty identifiers, absolute
paths, separators, parent components, excessive length, and characters outside
the documented set. A missing destination file cannot safely be used as a
reason to skip lexical validation.

## Suggested Module Boundary

```text
src/storage/
├── mod.rs
├── codec.rs          traits and primitive Encoder/Decoder
├── collection.rs     Collection<T>, framing, limits, persistence
├── store.rs          root and validated collection selection
└── error.rs          storage error vocabulary
```

Create `collection.rs` when primitive and concrete record codecs are ready and
you begin record framing. Do not put application validation, HTTP behavior,
`CARGO_MANIFEST_DIR`, or `Resource` field ordering there.

Next: [Reading and errors](07-reading-and-errors.md).
