# Phase 15: Concurrency

Goal: handle more than one client at a time.

Start with the simplest model: one thread per connection. Later, build a small thread pool.

## What to Learn

- Threads
- Shared ownership with `Arc`
- Interior mutability with `Mutex`
- Race conditions
- Lock scope

## Where to Look

- Rust threads: https://doc.rust-lang.org/book/ch16-01-threads.html
- Shared state concurrency: https://doc.rust-lang.org/book/ch16-03-shared-state.html
- `Arc`: https://doc.rust-lang.org/std/sync/struct.Arc.html
- `Mutex`: https://doc.rust-lang.org/std/sync/struct.Mutex.html

## Step-by-Step Work

1. Spawn a thread for each accepted connection.
2. Move the stream into that thread.
3. Wrap shared app state in `Arc<Mutex<_>>`.
4. Clone the `Arc` for each thread.
5. Lock state only while reading or modifying it.
6. Avoid holding the lock while writing to the TCP stream.
7. Test multiple simultaneous requests.

## Important Mental Model

```text
Arc
  lets multiple threads own a pointer to the same state

Mutex
  makes only one thread access the protected state at a time
```

## Questions to Answer

- Why does Rust reject shared mutable state without synchronization?
- What data actually needs a mutex?
- Why should locks be held for a short time?
- What happens if a handler panics while holding a lock?

## Checkpoint

You are done when:

- Multiple clients can connect.
- Shared note state remains consistent.
- You understand where locking happens.

