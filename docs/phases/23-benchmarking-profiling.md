# Phase 23: Benchmarking and Profiling

Goal: measure before optimizing.

Performance work should answer a specific question. Do not rewrite large parts of the server because something might be faster.

## What to Learn

- Throughput
- Latency percentiles
- Warm-up
- Release builds
- Allocation pressure
- Lock contention
- Benchmark noise

## Where to Look

- Cargo release builds: https://doc.rust-lang.org/cargo/reference/profiles.html
- Linux `perf`, if available: https://perf.wiki.kernel.org/
- `hyperfine`, if installed: https://github.com/sharkdp/hyperfine

## Step-by-Step Work

1. Build with `cargo build --release`.
2. Benchmark one simple route.
3. Benchmark one route that parses a body.
4. Benchmark one route that touches state.
5. Record the command, machine, build mode, and result.
6. Change one thing.
7. Re-run the same benchmark.

## Things To Measure

- Requests per second
- Median latency
- Tail latency
- CPU usage
- Memory growth
- Error rate under load

## Questions to Answer

- What exact bottleneck are you testing?
- Did the change improve release-mode performance?
- Did correctness or readability get worse?
- Is the bottleneck in parsing, routing, storage, locking, or writing?

## Checkpoint

You are done when:

- You have a repeatable benchmark command.
- Results are saved with enough context to compare later.
- One optimization decision is based on measurement.
