# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] – 2025-07-10

### Changed
**BREAKING CHANGES**: Renamed subscriber methods for better API clarity:

- `Sub<T>::next()` → `Sub<T>::wait_for_message()`
- `Sub<T>::try_next()` → `Sub<T>::try_get_message()`
- `Sub<T>::next_with(f)` → `Sub<T>::wait_for_message_and_apply(f)`
- `Sub<T>::try_next_with(f)` → `Sub<T>::try_get_message_and_apply(f)`

**BREAKING CHANGES**: Renamed error constructors for better clarity:

- `BusError::try_recv_empty()` → `BusError::message_queue_empty()`
- `BusError::try_recv_disconnected()` → `BusError::topic_disconnected()`

**BREAKING CHANGES**: Removed convenience methods that duplicated `with_capacity()`:

- `Bus<T>::high_throughput()` → use `Bus<T>::with_capacity(64)` instead
- `Bus<T>::low_latency()` → use `Bus<T>::with_capacity(8)` instead

### Added
- **Prelude module**: Added `dropslot::prelude` for convenient importing of common types:
  ```rust
  use dropslot::prelude::*; // Imports Bus, Topic, Sub, and BusError
  ```

### Removed
**BREAKING CHANGES**: Removed alias methods:

- `Sub<T>::recv()` (use `wait_for_message()` instead)
- `Sub<T>::try_recv()` (use `try_get_message()` instead)
- `Sub<T>::recv_with(f)` (use `wait_for_message_and_apply(f)` instead)
- `Sub<T>::try_recv_with(f)` (use `try_get_message_and_apply(f)` instead)

## [0.1.0] – 2025-07-10
### Added
- Initial release of DropSlot.

[0.2.0]: https://github.com/ViezeVingertjes/dropslot/releases/tag/v0.2.0
[0.1.0]: https://github.com/ViezeVingertjes/dropslot/releases/tag/v0.1.0
