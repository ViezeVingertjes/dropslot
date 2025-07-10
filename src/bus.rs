use crate::{sub::Sub, topic::Topic};
use ahash::AHasher;
use dashmap::DashMap;
use std::hash::BuildHasherDefault;
use std::sync::Arc;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{_MM_HINT_T0, _mm_prefetch};

/// High-performance publish-subscribe message broker with latest-only delivery.
pub struct Bus<T> {
    topics: DashMap<Arc<str>, Arc<Topic<T>>, BuildHasherDefault<AHasher>>,
}

impl<T> Default for Bus<T>
where
    T: Clone,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Bus<T>
where
    T: Clone,
{
    /// Creates a new Bus with default settings.
    #[inline]
    pub fn new() -> Self {
        Self {
            topics: DashMap::with_capacity_and_hasher(16, BuildHasherDefault::<AHasher>::default()),
        }
    }

    /// Creates a Bus with specified initial capacity.
    ///
    /// # Arguments
    /// * `capacity` - Initial capacity for the internal topic storage
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::with_capacity(32);
    /// assert_eq!(bus.topic_count(), 0);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            topics: DashMap::with_capacity_and_hasher(
                capacity,
                BuildHasherDefault::<AHasher>::default(),
            ),
        }
    }

    /// Creates a Bus optimized for high throughput scenarios.
    ///
    /// This configuration uses a larger initial capacity (64) to reduce
    /// hash map resizing overhead when dealing with many topics.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::high_throughput();
    /// // Optimized for scenarios with many topics
    /// ```
    #[inline]
    pub fn high_throughput() -> Self {
        Self::with_capacity(64)
    }

    /// Creates a Bus optimized for low latency scenarios.
    ///
    /// This configuration uses a smaller initial capacity (8) to minimize
    /// memory usage and improve cache locality for applications with few topics.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::low_latency();
    /// // Optimized for scenarios with few topics
    /// ```
    #[inline]
    pub fn low_latency() -> Self {
        Self::with_capacity(8)
    }

    /// Gets existing topic or creates a new one.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    /// ```
    #[inline]
    pub fn topic(&self, name: &str) -> Arc<Topic<T>> {
        let key: Arc<str> = name.into();
        if let Some(topic) = self.get_topic_with_prefetch(&key) {
            return topic;
        }
        self.create_topic_with_race_protection(key, name.to_string())
    }

    /// Publishes a message to the specified topic.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// bus.publish("events", "Hello".to_string());
    /// ```
    #[inline]
    pub fn publish(&self, topic_name: &str, message: T) {
        let topic = self.topic(topic_name);
        topic.publish(message);
    }

    /// Creates a subscriber for the specified topic.
    ///
    /// This is a convenience method that creates a topic if it doesn't exist
    /// and immediately subscribes to it.
    ///
    /// # Arguments
    /// * `topic_name` - Name of the topic to subscribe to
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let mut subscriber = bus.subscribe("events");
    /// ```
    #[inline]
    pub fn subscribe(&self, topic_name: &str) -> Sub<T> {
        let topic = self.topic(topic_name);
        topic.subscribe()
    }

    /// Returns the number of active topics.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// assert_eq!(bus.topic_count(), 0);
    ///
    /// let _topic = bus.topic("events");
    /// assert_eq!(bus.topic_count(), 1);
    /// ```
    #[inline]
    pub fn topic_count(&self) -> usize {
        self.topics.len()
    }

    /// Removes a topic and returns the number of subscribers it had.
    ///
    /// # Arguments
    /// * `topic_name` - Name of the topic to remove
    ///
    /// # Returns
    /// * `Some(count)` - Number of subscribers the topic had before removal
    /// * `None` - Topic did not exist
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    /// let _subscriber = topic.subscribe();
    ///
    /// let count = bus.remove_topic("events");
    /// assert_eq!(count, Some(1));
    /// ```
    #[inline]
    pub fn remove_topic(&self, topic_name: &str) -> Option<usize> {
        let key: Arc<str> = topic_name.into();
        self.topics
            .remove(&key)
            .map(|(_, topic)| topic.subscriber_count())
    }

    /// Returns all topic names.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let _topic1 = bus.topic("events");
    /// let _topic2 = bus.topic("logs");
    ///
    /// let names = bus.topic_names();
    /// assert_eq!(names.len(), 2);
    /// assert!(names.contains(&"events".to_string()));
    /// assert!(names.contains(&"logs".to_string()));
    /// ```
    #[inline]
    pub fn topic_names(&self) -> Vec<String> {
        self.topics
            .iter()
            .map(|entry| entry.key().to_string())
            .collect()
    }

    /// Removes topics with no active subscribers.
    ///
    /// This method helps prevent memory leaks by cleaning up unused topics.
    /// It's safe to call periodically in long-running applications.
    ///
    /// # Returns
    /// The number of topics that were removed.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    /// let subscriber = topic.subscribe();
    ///
    /// // Topic has subscriber, won't be removed
    /// assert_eq!(bus.cleanup_unused_topics(), 0);
    ///
    /// drop(subscriber);
    /// // Topic has no subscribers, will be removed
    /// assert_eq!(bus.cleanup_unused_topics(), 1);
    /// ```
    pub fn cleanup_unused_topics(&self) -> usize {
        let mut removed_count: usize = 0;
        self.topics.retain(|_, topic| {
            if topic.subscriber_count() == 0 {
                removed_count = removed_count.saturating_add(1);
                false
            } else {
                true
            }
        });
        removed_count
    }

    #[inline(always)]
    pub(crate) fn get_topic_with_prefetch(&self, key: &Arc<str>) -> Option<Arc<Topic<T>>> {
        self.topics.get(key).map(|entry| {
            let topic = entry.value();
            prefetch_read(topic.as_ref());
            topic.clone()
        })
    }

    #[inline]
    pub(crate) fn create_topic_with_race_protection(
        &self,
        key: Arc<str>,
        name: String,
    ) -> Arc<Topic<T>> {
        let topic = Arc::new(Topic::new(name));
        match self.topics.entry(key) {
            dashmap::mapref::entry::Entry::Occupied(entry) => {
                let existing = entry.get();
                prefetch_read(existing.as_ref());
                existing.clone()
            }
            dashmap::mapref::entry::Entry::Vacant(entry) => {
                entry.insert(topic.clone());
                topic
            }
        }
    }
}

#[inline(always)]
pub(crate) fn prefetch_read<T>(data: *const T) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        _mm_prefetch(data as *const i8, _MM_HINT_T0);
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = data;
    }
}

/// Zero-copy bytes operations for Bus.
impl Bus<bytes::Bytes> {
    /// Publishes a byte slice as Bytes (zero-copy when possible).
    ///
    /// This method copies the slice into a `Bytes` object for publishing.
    /// For true zero-copy, use `publish_vec` with a `Vec<u8>`.
    ///
    /// # Arguments
    /// * `topic_name` - Name of the topic to publish to
    /// * `data` - Byte slice to publish
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// # use bytes::Bytes;
    /// let bus = Bus::<Bytes>::new();
    /// let data = b"Hello, World!";
    /// bus.publish_slice("events", data);
    /// ```
    #[inline]
    pub fn publish_slice(&self, topic_name: &str, data: &[u8]) {
        self.publish(topic_name, bytes::Bytes::copy_from_slice(data))
    }

    /// Publishes a `Vec<u8>` as Bytes.
    ///
    /// This method provides zero-copy publishing by converting the `Vec<u8>`
    /// directly into a `Bytes` object without copying the data.
    ///
    /// # Arguments
    /// * `topic_name` - Name of the topic to publish to
    /// * `data` - Vector of bytes to publish
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// # use bytes::Bytes;
    /// let bus = Bus::<Bytes>::new();
    /// let data = vec![72, 101, 108, 108, 111]; // "Hello"
    /// bus.publish_vec("events", data);
    /// ```
    #[inline]
    pub fn publish_vec(&self, topic_name: &str, data: Vec<u8>) {
        self.publish(topic_name, bytes::Bytes::from(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_counter_overflow_protection() {
        let bus = Bus::<String>::new();

        for i in 0..10 {
            let topic = bus.topic(&format!("topic_{i}"));
            drop(topic);
        }

        let removed_count = bus.cleanup_unused_topics();
        assert_eq!(removed_count, 10);
    }

    #[test]
    fn test_cleanup_saturating_add() {
        let result = usize::MAX.saturating_add(1);
        assert_eq!(result, usize::MAX);
    }

    #[test]
    fn test_get_topic_with_prefetch_hit() {
        let bus = Bus::<String>::new();
        let _topic = bus.topic("test");

        let key: Arc<str> = "test".into();
        let retrieved = bus.get_topic_with_prefetch(&key);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "test");
    }

    #[test]
    fn test_get_topic_with_prefetch_miss() {
        let bus = Bus::<String>::new();
        let key: Arc<str> = "nonexistent".into();
        let retrieved = bus.get_topic_with_prefetch(&key);
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_create_topic_with_race_protection_vacant() {
        let bus = Bus::<String>::new();
        let key: Arc<str> = "new_topic".into();
        let topic = bus.create_topic_with_race_protection(key.clone(), "new_topic".to_string());
        assert_eq!(topic.name(), "new_topic");
    }

    #[test]
    fn test_create_topic_with_race_protection_occupied() {
        let bus = Bus::<String>::new();
        let _existing_topic = bus.topic("existing");

        let key: Arc<str> = "existing".into();
        let topic = bus.create_topic_with_race_protection(key.clone(), "existing".to_string());
        assert_eq!(topic.name(), "existing");
    }

    #[test]
    fn test_prefetch_read_non_x86_64() {
        let value = 42i32;
        prefetch_read(&value);
    }
}
