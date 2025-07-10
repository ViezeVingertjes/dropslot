use crate::sub::Sub;
use std::sync::Arc;
use tokio::sync::watch;

/// A message topic that delivers only the latest published message to subscribers.
#[repr(align(64))]
#[derive(Debug)]
pub struct Topic<T> {
    sender: watch::Sender<Option<T>>,
    name: Box<str>,
    version: std::sync::atomic::AtomicU64,
}

impl<T> Topic<T>
where
    T: Clone,
{
    #[inline]
    pub(crate) fn new(name: String) -> Self {
        let (sender, _receiver) = watch::channel(None);
        Self {
            sender,
            name: name.into_boxed_str(),
            version: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Publishes a message to this topic.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// # use bytes::Bytes;
    /// let bus = Bus::<Bytes>::new();
    /// let topic = bus.topic("events");
    /// topic.publish(Bytes::from("Hello, World!"));
    /// ```
    #[inline(always)]
    pub fn publish(&self, message: T) {
        let _ = self.sender.send(Some(message));
        self.increment_version();
    }

    /// Creates a new subscriber for this topic.
    ///
    /// Subscribers receive only messages published after they subscribe.
    /// The subscriber will receive the latest published message and all
    /// subsequent messages until it's dropped.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    /// let mut subscriber = topic.subscribe();
    /// ```
    #[inline]
    pub fn subscribe(self: &Arc<Self>) -> Sub<T> {
        let current_version = self.get_current_version();
        Sub::new(
            self.sender.subscribe(),
            self.name.clone(),
            Arc::downgrade(self),
            current_version,
            Some(self.clone()),
        )
    }

    /// Returns the number of active subscribers.
    ///
    /// This count includes all subscribers that haven't been dropped yet.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    ///
    /// assert_eq!(topic.subscriber_count(), 0);
    ///
    /// let _sub1 = topic.subscribe();
    /// let _sub2 = topic.subscribe();
    /// assert_eq!(topic.subscriber_count(), 2);
    /// ```
    #[inline(always)]
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }

    /// Returns the topic name.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    ///
    /// assert_eq!(topic.name(), "events");
    /// ```
    #[inline(always)]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns true if the topic has any subscribers.
    ///
    /// This is equivalent to `topic.subscriber_count() > 0` but more efficient.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    ///
    /// assert!(!topic.has_subscribers());
    ///
    /// let _subscriber = topic.subscribe();
    /// assert!(topic.has_subscribers());
    /// ```
    #[inline(always)]
    pub fn has_subscribers(&self) -> bool {
        self.sender.receiver_count() > 0
    }

    #[inline(always)]
    pub(crate) fn increment_version(&self) {
        let current = self.version.load(std::sync::atomic::Ordering::Relaxed);
        if current < u64::MAX {
            self.version
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }

    #[inline(always)]
    pub(crate) fn get_current_version(&self) -> u64 {
        self.version.load(std::sync::atomic::Ordering::Relaxed)
    }

    #[cfg(test)]
    pub(crate) fn set_version_for_test(&self, version: u64) {
        self.version
            .store(version, std::sync::atomic::Ordering::Relaxed);
    }
}

/// Zero-copy bytes operations for Topics.
impl Topic<bytes::Bytes> {
    /// Publishes a byte slice to this topic.
    ///
    /// This method copies the slice into a `Bytes` object for publishing.
    /// For true zero-copy, use `publish_vec` with a `Vec<u8>`.
    ///
    /// # Arguments
    /// * `data` - Byte slice to publish
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// # use bytes::Bytes;
    /// let bus = Bus::<Bytes>::new();
    /// let topic = bus.topic("events");
    ///
    /// let data = b"Hello, World!";
    /// topic.publish_slice(data);
    /// ```
    #[inline]
    pub fn publish_slice(&self, data: &[u8]) {
        self.publish(bytes::Bytes::copy_from_slice(data));
    }

    /// Publishes a `Vec<u8>` to this topic.
    ///
    /// This method provides zero-copy publishing by converting the `Vec<u8>`
    /// directly into a `Bytes` object without copying the data.
    ///
    /// # Arguments
    /// * `data` - Vector of bytes to publish
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// # use bytes::Bytes;
    /// let bus = Bus::<Bytes>::new();
    /// let topic = bus.topic("events");
    ///
    /// let data = vec![72, 101, 108, 108, 111]; // "Hello"
    /// topic.publish_vec(data);
    /// ```
    #[inline]
    pub fn publish_vec(&self, data: Vec<u8>) {
        self.publish(bytes::Bytes::from(data));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_overflow_protection() {
        let topic = Topic::<String>::new("test".to_string());

        topic.set_version_for_test(u64::MAX);

        let version_before = topic.get_current_version();
        assert_eq!(version_before, u64::MAX);

        topic.increment_version();

        let version_after = topic.get_current_version();
        assert_eq!(version_after, u64::MAX);
    }

    #[test]
    fn test_normal_version_increment() {
        let topic = Topic::<String>::new("test".to_string());

        let version_before = topic.get_current_version();
        assert_eq!(version_before, 0);

        topic.increment_version();

        let version_after = topic.get_current_version();
        assert_eq!(version_after, 1);
    }
}
