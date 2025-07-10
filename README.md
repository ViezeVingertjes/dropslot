# DropSlot 🏗️

[![Crates.io](https://img.shields.io/crates/v/dropslot.svg)](https://crates.io/crates/dropslot)
[![Documentation](https://docs.rs/dropslot/badge.svg)](https://docs.rs/dropslot)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

A high-performance publish-subscribe library with **latest-only delivery semantics** for Rust. Built on top of Tokio with zero-copy operations and optimized for both high throughput and low latency scenarios.

## ✨ Key Features

- **Latest-only delivery**: Subscribers receive only the most recent message, perfect for real-time applications
- **Zero-copy operations**: Optimized for `bytes::Bytes` and other efficient data types
- **String-keyed topics**: Simple and intuitive topic naming system
- **High performance**: Optimized data structures, memory layout, and CPU cache utilization
- **Async/sync APIs**: Both `async` and non-blocking synchronous operations
- **Thread-safe**: Built with concurrent access in mind using lock-free data structures
- **Memory efficient**: Weak references prevent memory leaks with manual cleanup available

## 🚀 Quick Start

Add dropslot to your `Cargo.toml`:

```toml
[dependencies]
dropslot = "0.1"
```

### Basic Usage

```rust
use dropslot::Bus;
use bytes::Bytes;

#[tokio::main]
async fn main() {
    let bus = Bus::<Bytes>::new();
    
    // Create a topic and subscriber
    let topic = bus.topic("events");
    let mut subscriber = topic.subscribe();
    
    // Publish messages
    topic.publish(Bytes::from("Hello, World!"));
    
    // Receive the latest message
    if let Some(message) = subscriber.wait_for_message().await {
        println!("Received: {:?}", message);
    }
}
```

### Performance Configurations

```rust
use dropslot::Bus;
use bytes::Bytes;

// High throughput: optimized for many messages
let ht_bus = Bus::<Bytes>::high_throughput();

// Low latency: optimized for speed
let ll_bus = Bus::<Bytes>::low_latency();

// Custom capacity
let custom_bus = Bus::<Bytes>::with_capacity(128);
```

## 📊 Performance

DropSlot is designed for high-performance scenarios and delivers exceptional performance:

### Key Performance Metrics

| Operation | Latency | Throughput | Notes |
|-----------|---------|------------|-------|
| **Topic Creation** | ~136 ns | ~7.4M ops/sec | Ultra-fast topic instantiation |
| **Message Publishing** | ~467 ns | ~2.1M ops/sec | Direct publish to topic |
| **Subscriber Creation** | ~510 ns | ~2.0M ops/sec | Fast subscriber setup |
| **Message Retrieval** | ~477 ns | ~2.1M ops/sec | Non-blocking message access |
| **Topic Lookup** | ~40 ns | ~25M ops/sec | Optimized topic resolution |
| **Error Handling** | ~330 ps | ~3.0B ops/sec | Near-zero overhead |

### Scalability Performance

| Scenario | Performance | Details |
|----------|-------------|----------|
| **10 Topics** | ~3.6 μs | Excellent small-scale performance |
| **100 Topics** | ~43 μs | Linear scaling maintained |
| **1000 Topics** | ~458 μs | Consistent performance at scale |
| **High Frequency** | ~11 μs/batch | 1000 message batches |
| **Concurrent (16 threads)** | ~3.3 ms | Excellent multi-threaded performance |

### Memory & Concurrency

- **Memory cleanup**: ~3.3 μs for unused topic cleanup
- **Concurrent publishing**: Linear scaling up to 16 threads
- **Zero-copy operations**: ~612 ns for `bytes::Bytes` handling
- **Topic management**: ~529 ns for topic removal operations

### Architecture Optimizations

- **Lock-free concurrent access** using `DashMap`
- **Optimized hashing** with `AHash`
- **CPU cache-friendly** memory layout and prefetching
- **Zero-copy operations** for byte data
- **Efficient memory management** with `Arc` and weak references

*Benchmarks run on CI environment with optimized builds. Your mileage may vary based on hardware and workload.*

## 🔧 Advanced Usage

### Custom Message Types

```rust
use dropslot::Bus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Event {
    id: u64,
    data: String,
}

let bus = Bus::<Event>::new();
let topic = bus.topic("events");

let event = Event {
    id: 1,
    data: "Hello".to_string(),
};

topic.publish(event);
```

### Error Handling

```rust
use dropslot::Bus;

let bus = Bus::<String>::new();
let topic = bus.topic("events");
let mut subscriber = topic.subscribe();

match subscriber.try_get_message() {
    Ok(Some(msg)) => println!("Received: {}", msg),
    Ok(None) => println!("No new message"),
    Err(e) if e.is_empty() => println!("No message available"),
    Err(e) if e.is_disconnected() => println!("Topic disconnected"),
    Err(e) => println!("Error: {}", e),
}
```

### Multiple Subscribers

```rust
use dropslot::Bus;

let bus = Bus::<String>::new();
let topic = bus.topic("notifications");

// Multiple subscribers to the same topic
let mut email_sub = topic.subscribe();
let mut sms_sub = topic.subscribe();
let mut push_sub = topic.subscribe();

// All subscribers receive the same (latest) message
topic.publish("Important update!".to_string());
```

### Topic Management

```rust
use dropslot::Bus;

let bus = Bus::<String>::new();

// Check topic count
println!("Active topics: {}", bus.topic_count());

// Get all topic names
let names = bus.topic_names();
println!("Topics: {:?}", names);

// Manually clean up unused topics (no automatic cleanup)
let removed = bus.cleanup_unused_topics();
println!("Removed {} unused topics", removed);
```

## 🎯 Use Cases

DropSlot is perfect for:

- **Real-time notifications** (email, SMS, push notifications)
- **Live data feeds** (stock prices, sensor data, metrics)
- **Event sourcing** with latest-state semantics
- **Microservice communication** for status updates
- **Game state synchronization**
- **IoT device coordination**

## 🔩 Architecture

### Core Components

- **`Bus<T>`**: Main message broker managing topics
- **`Topic<T>`**: Individual message topics with publishers and subscribers
- **`Sub<T>`**: Subscriber receiving messages from topics
- **`BusError`**: Unified error handling

### Design Principles

- **Latest-only semantics**: Built on `tokio::sync::watch` channels
- **Memory safety**: Extensive use of `Arc` and `Weak` references
- **Performance first**: Optimized data structures and algorithms
- **Zero-copy where possible**: Efficient handling of byte data

## 🛠️ Features

### Default Features

- `bytes` - Zero-copy operations for `bytes::Bytes`

### Optional Features

- `serde` - Serialization support for complex message types

Enable features in your `Cargo.toml`:

```toml
[dependencies]
dropslot = { version = "0.1", features = ["serde"] }
```

## 📈 Benchmarks

Run benchmarks with:

```bash
cargo bench
```

## 🧪 Testing

Run tests with:

```bash
cargo test
```

Run examples:

```bash
cargo run --example basic_usage
cargo run --example high_performance
cargo run --example real_world --features="serde"
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

## 🔍 Changelog

See [CHANGELOG.md](CHANGELOG.md) for recent changes.

---

**Note**: This library implements latest-only delivery semantics, meaning subscribers only receive the most recent message. Topic cleanup is manual via `bus.cleanup_unused_topics()` - call this periodically in long-running applications to prevent memory leaks. For all-message delivery, consider using `tokio::sync::broadcast` or similar alternatives.