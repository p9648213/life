# Encode and Decode Traits

Define the codec contracts in the generic storage layer, for example in:

```text
src/storage/codec.rs
```

Implement those contracts near each application record:

```text
src/storage/codec.rs          defines Encode and Decode
src/app/resource/model.rs     Resource implements them
```

This keeps the dependency direction correct:

```text
storage defines a capability
application records implement it
storage does not depend on Resource
```

## What the Traits Mean

Conceptually:

```text
Encode: write my fields into a primitive Encoder
Decode: construct me by reading fields from a primitive Decoder
```

The record owns field order. For a `Resource`, that might be:

```text
encode: id -> name -> number
decode: id -> name -> number
```

The concrete signatures are your design decision.

## `where T: Encode + Decode`

This is a trait bound. It means `T` can be any type for which Rust can prove:

```text
T implements Encode
and
T implements Decode
```

The `+` means “and.” The bound is checked at compile time, not with a runtime
condition.

It permits generic collection code to perform operations conceptually like:

```text
item.encode(encoder)
T::decode(decoder)
```

Bounds can be precise per operation:

```text
list:        T needs Decode
save:        T needs Encode
insert:      T needs Decode + Encode when it reloads the old snapshot
```

## `trait Decode: Sized`

`Sized` means the direct size of a value is known at compile time. `Decode`
constructs and returns `Self` by value, so Rust needs that guarantee.

`Resource`, `String`, and `Vec<T>` are sized. A `String` owns variable-length
heap data, but the `String` handle itself has a fixed direct size.

Directly unsized types include:

```text
str
[T]
dyn SomeTrait
```

They are normally used behind sized pointers such as `&str`, `&[T]`, or
`Box<dyn SomeTrait>`.

Phase 09A decodes concrete owned records, so a trait-level `Sized` requirement
is appropriate. A method-level `where Self: Sized` is an alternative when the
trait itself must also support unsized implementors; this phase does not need
that flexibility.

Next: [Encoder and Decoder](04-encoder-decoder.md).
