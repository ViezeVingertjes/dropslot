//! # DropSlot
//!
//! High-performance publish-subscribe library with latest-only delivery semantics.
//!
//! ## Key Features
//!
//! - **Latest-only delivery**: Subscribers receive the most recent message only
//! - **Zero-copy operations**: Optimized for `bytes::Bytes` and other types
//! - **String-keyed topics**: Simple string-based topic naming
//! - **High performance**: Optimized data structures and memory layout
//! - **Async/sync APIs**: Both `async` and non-blocking sync operations
//!
//! ## Quick Start
//!
//! ```rust
//! use dropslot::prelude::*;
//! use bytes::Bytes;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let bus = Bus::<Bytes>::new();
//! let topic = bus.topic("events");
//! let mut subscriber = topic.subscribe();
//!
//! topic.publish(Bytes::from("Hello, World!"));
//!
//! if let Some(message) = subscriber.wait_for_message().await {
//!     println!("Received: {:?}", message);
//! }
//! # }
//! ```
//!
//! ## Performance Configurations
//!
//! ```rust
//! use dropslot::prelude::*;
//! use bytes::Bytes;
//!
//! // High throughput: larger initial capacity (optimized for many topics)
//! let ht_bus = Bus::<Bytes>::with_capacity(64);
//!
//! // Low latency: smaller initial capacity (optimized for few topics)
//! let ll_bus = Bus::<Bytes>::with_capacity(8);
//! ```
//!
//! ## Zero-Copy Bytes
//!
//! ```rust
//! use dropslot::prelude::*;
//! use bytes::Bytes;
//!
//! let bus = Bus::<Bytes>::new();
//! let topic = bus.topic("data");
//! topic.publish(Bytes::from("zero-copy message"));
//! ```

pub mod bus;
pub mod error;
pub mod prelude;
pub mod sub;
pub mod topic;

pub use bus::Bus;
pub use error::BusError;
pub use sub::Sub;
pub use topic::Topic;
