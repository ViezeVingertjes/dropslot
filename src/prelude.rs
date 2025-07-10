//! Convenience re-export of common DropSlot types.
//!
//! This prelude module re-exports the most commonly used types so that users
//! can easily import everything they need with a single `use` statement:
//!
//! ```rust
//! use dropslot::prelude::*;
//!
//! // Now you can use Bus, Topic, Sub, and BusError directly
//! let bus = Bus::<String>::new();
//! let topic = bus.topic("events");
//! let mut subscriber = topic.subscribe();
//! ```

pub use crate::{Bus, BusError, Sub, Topic};
