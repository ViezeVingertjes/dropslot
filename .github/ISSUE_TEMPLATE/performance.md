---
name: Performance Issue
about: Report performance problems or suggest optimizations
title: '[PERFORMANCE] '
labels: performance
assignees: ''

---

## Performance Issue Description
A clear and concise description of the performance problem.

## Current Performance
**Measurements:**
- Latency: [e.g., 500ms]
- Throughput: [e.g., 1000 ops/sec]
- Memory usage: [e.g., 100MB]
- CPU usage: [e.g., 80%]

## Expected Performance
**What performance do you expect?**
- Target latency: [e.g., <100ms]
- Target throughput: [e.g., 10k ops/sec]
- Target memory usage: [e.g., <50MB]

## Benchmark Code
```rust
// Code used to measure performance
use dropslot::Bus;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    
    // Your benchmark code here
    
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
```

## Environment
- **OS**: [e.g., Windows 11, Ubuntu 22.04, macOS 13]
- **CPU**: [e.g., Intel i7-12700K, AMD Ryzen 7 5800X]
- **RAM**: [e.g., 32GB DDR4-3200]
- **Rust Version**: [e.g., 1.70.0]
- **DropSlot Version**: [e.g., 0.1.0]
- **Build Profile**: [e.g., release, debug]

## Profiling Data
If you have profiling data (flamegraph, perf, etc.), please attach it or provide relevant excerpts.

## Potential Optimizations
If you have ideas for optimizations, please describe them here.

## Comparison
How does this compare to similar libraries or previous versions?

## Additional Context
- Scale of operation (number of topics, subscribers, etc.)
- Concurrent usage patterns
- Hardware constraints
- Real-world vs synthetic benchmarks

## Checklist
- [ ] I have measured the performance issue with release builds
- [ ] I have provided specific metrics and expectations
- [ ] I have included reproducible benchmark code
- [ ] I have specified my hardware and environment details
