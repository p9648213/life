# Phase 23: Benchmarking and Profiling

Goal: make performance decisions from repeatable measurements.

Design the benchmark harness and result format yourself.

## Expected Behavior

You can reproduce release-mode measurements for a simple route, a parsed-body route, and a state or storage route, then compare one controlled change.

## Requirements

- State the question and suspected bottleneck before measuring.
- Use release builds and record the exact revision.
- Record hardware, operating conditions, command, workload, concurrency, limits, and dataset.
- Include warm-up and multiple samples.
- Measure throughput, latency distribution, error rate, CPU, and memory when relevant.
- Separate client/load-generator limits from server limits.
- Preserve correctness checks during load.
- Change one relevant variable at a time.
- Profile before large rewrites.
- Treat pathological scaling at configured limits as a correctness problem.

## Tests to Write

- the benchmark workload returns expected responses;
- failures are counted rather than discarded;
- repeated baseline runs show an understood noise range;
- increasing input toward configured limits has expected time and memory scaling;
- before-and-after runs use the same conditions.

## Checkpoint

You are done when one performance decision is supported by saved, repeatable evidence and no correctness invariant was traded away silently.

After this, continue with [Phase 24: Backend Core API Boundary](24-backend-core-api-boundary.md).
