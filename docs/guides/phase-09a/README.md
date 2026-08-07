# Phase 09A Learning Guides

These guides explain the ideas behind
[Phase 09A](../../phases/09a-file-backed-storage-foundation.md). Each file
answers one small group of questions. Read only the topic you need, or follow
the implementation guide in order.

## Find Your Question

| Question | Guide |
| --- | --- |
| What are the layers, and who owns what? | [Mental model](01-mental-model.md) |
| What do big-endian and little-endian mean? | [File format and byte order](02-file-format-and-byte-order.md) |
| Where should `Encode` and `Decode` live? | [Codec traits](03-codec-traits.md) |
| What do `T: Encode + Decode` and `Sized` mean? | [Codec traits](03-codec-traits.md) |
| What is an `Encoder`, and where is it created? | [Encoder and Decoder](04-encoder-decoder.md) |
| Should `write_u32` be an `Encoder` method? | [Encoder and Decoder](04-encoder-decoder.md) |
| What are `max_size` and collection limits? | [Storage limits](05-storage-limits.md) |
| Can limits wait until later? | [Storage limits](05-storage-limits.md) |
| Why use `Collection<T>`? | [Store and Collection](06-store-and-collection.md) |
| What is `PhantomData<T>`? | [Store and Collection](06-store-and-collection.md) |
| How does `Store` construct `Collection<Resource>`? | [Store and Collection](06-store-and-collection.md) |
| How should collection files be read and rejected? | [Reading and errors](07-reading-and-errors.md) |
| How do stable IDs and safe saving work? | [Replacement and stable IDs](08-replacement-and-stable-ids.md) |
| What should I implement next? | [Step-by-step path](09-step-by-step.md) |

## Suggested Reading Order

1. [Mental model](01-mental-model.md)
2. [File format and byte order](02-file-format-and-byte-order.md)
3. [Codec traits](03-codec-traits.md)
4. [Encoder and Decoder](04-encoder-decoder.md)
5. [Storage limits](05-storage-limits.md)
6. [Store and Collection](06-store-and-collection.md)
7. [Reading and errors](07-reading-and-errors.md)
8. [Replacement and stable IDs](08-replacement-and-stable-ids.md)
9. [Step-by-step path](09-step-by-step.md)

The names and snippets describe responsibilities, not mandatory APIs. Choose
the concrete modules, types, method names, ownership, and APIs yourself.
