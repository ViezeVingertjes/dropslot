use crate::{error::BusError, topic::Topic};
use std::sync::{Arc, Weak};
use tokio::sync::watch;

/// Receives the latest messages from a topic.
#[derive(Debug)]
pub struct Sub<T> {
    receiver: watch::Receiver<Option<T>>,
    topic_name: Box<str>,
    topic_ref: Weak<Topic<T>>,
    last_seen_version: u64,
    cached_topic: Option<Arc<Topic<T>>>,
}

impl<T> Sub<T>
where
    T: Clone,
{
    #[inline]
    pub(crate) fn new(
        receiver: watch::Receiver<Option<T>>,
        topic_name: Box<str>,
        topic_ref: Weak<Topic<T>>,
        last_seen_version: u64,
        cached_topic: Option<Arc<Topic<T>>>,
    ) -> Self {
        Self {
            receiver,
            topic_name,
            topic_ref,
            last_seen_version,
            cached_topic,
        }
    }

    /// Waits for the next message asynchronously.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// # use bytes::Bytes;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let bus = Bus::<Bytes>::new();
    /// let topic = bus.topic("events");
    /// let mut subscriber = topic.subscribe();
    ///
    /// topic.publish(Bytes::from("Hello"));
    /// if let Some(message) = subscriber.wait_for_message().await {
    ///     println!("Received: {:?}", message);
    /// }
    /// # }
    /// ```
    pub async fn wait_for_message(&mut self) -> Option<T> {
        loop {
            if self.receiver.changed().await.is_err() {
                return None;
            }
            let borrowed = self.receiver.borrow();
            if let Some(message) = borrowed.as_ref() {
                return Some(message.clone());
            }
        }
    }

    /// Waits for a message and applies a transformation.
    ///
    /// This method waits for the next message and applies the given function
    /// to it before returning. This is useful for processing messages without
    /// cloning them.
    ///
    /// # Arguments
    /// * `f` - Function to apply to the message
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    /// let mut subscriber = topic.subscribe();
    ///
    /// topic.publish("hello world".to_string());
    ///
    /// let length = subscriber.wait_for_message_and_apply(|msg| msg.len()).await;
    /// assert_eq!(length, Some(11));
    /// # }
    /// ```
    pub async fn wait_for_message_and_apply<R>(&mut self, f: impl FnOnce(&T) -> R) -> Option<R> {
        loop {
            if self.receiver.changed().await.is_err() {
                return None;
            }
            let borrowed = self.receiver.borrow();
            if let Some(message) = borrowed.as_ref() {
                return Some(f(message));
            }
        }
    }

    /// Attempts to receive a message without blocking.
    ///
    /// Returns `Ok(Some(message))` if a new message is available,
    /// `Ok(None)` if no new message since last check,
    /// `Err(BusError::message_queue_empty())` if no message available,
    /// `Err(BusError::topic_disconnected())` if topic is dropped.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    /// let mut subscriber = topic.subscribe();
    ///
    /// // No message published yet
    /// assert!(subscriber.try_get_message().is_err());
    ///
    /// topic.publish("Hello".to_string());
    /// assert_eq!(subscriber.try_get_message().unwrap(), Some("Hello".to_string()));
    ///
    /// // No new message since last check
    /// assert!(subscriber.try_get_message().is_err());
    /// ```
    #[inline]
    pub fn try_get_message(&mut self) -> Result<Option<T>, BusError> {
        self.try_get_message_impl(|msg| msg.clone())
    }

    /// Attempts to receive and transform a message without blocking.
    ///
    /// This method is similar to `try_get_message` but applies a transformation
    /// function to the message before returning it.
    ///
    /// # Arguments
    /// * `f` - Function to apply to the message
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    /// let mut subscriber = topic.subscribe();
    ///
    /// topic.publish("hello".to_string());
    ///
    /// let length = subscriber.try_get_message_and_apply(|msg| msg.len()).unwrap();
    /// assert_eq!(length, Some(5));
    /// ```
    #[inline]
    pub fn try_get_message_and_apply<R>(
        &mut self,
        f: impl FnOnce(&T) -> R,
    ) -> Result<Option<R>, BusError> {
        self.try_get_message_impl(f)
    }

    /// Gets the latest message without consuming it.
    ///
    /// This method returns the most recent message published to the topic,
    /// if any. It does not block and does not mark the message as consumed.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    /// let subscriber = topic.subscribe();
    ///
    /// assert_eq!(subscriber.get_latest(), None);
    ///
    /// topic.publish("Hello".to_string());
    /// assert_eq!(subscriber.get_latest(), Some("Hello".to_string()));
    /// ```
    #[inline(always)]
    pub fn get_latest(&self) -> Option<T> {
        self.receiver.borrow().clone()
    }

    /// Gets and transforms the latest message without consuming it.
    ///
    /// This method applies a transformation function to the most recent message
    /// without cloning it. Returns `None` if no message is available.
    ///
    /// # Arguments
    /// * `f` - Function to apply to the message
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    /// let subscriber = topic.subscribe();
    ///
    /// topic.publish("hello world".to_string());
    ///
    /// let word_count = subscriber.get_latest_with(|msg| msg.split_whitespace().count());
    /// assert_eq!(word_count, Some(2));
    /// ```
    #[inline(always)]
    pub fn get_latest_with<R>(&self, f: impl FnOnce(&T) -> R) -> Option<R> {
        let borrowed = self.receiver.borrow();
        borrowed.as_ref().map(f)
    }

    /// Returns true if a message is currently available.
    ///
    /// This method checks if there is a message available without retrieving it.
    /// It's useful for non-blocking checks of message availability.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    /// let subscriber = topic.subscribe();
    ///
    /// assert!(!subscriber.has_latest());
    ///
    /// topic.publish("Hello".to_string());
    /// assert!(subscriber.has_latest());
    /// ```
    #[inline(always)]
    pub fn has_latest(&self) -> bool {
        self.receiver.borrow().is_some()
    }

    /// Returns the name of the subscribed topic.
    ///
    /// # Examples
    /// ```
    /// # use dropslot::Bus;
    /// let bus = Bus::<String>::new();
    /// let topic = bus.topic("events");
    /// let subscriber = topic.subscribe();
    ///
    /// assert_eq!(subscriber.topic_name(), "events");
    /// ```
    #[inline(always)]
    pub fn topic_name(&self) -> &str {
        &self.topic_name
    }

    #[inline]
    fn try_get_message_impl<R>(
        &mut self,
        transform: impl FnOnce(&T) -> R,
    ) -> Result<Option<R>, BusError> {
        match self.get_or_refresh_topic() {
            Some(topic) => {
                let current_version = topic.get_current_version();
                if current_version > self.last_seen_version
                    || (current_version == u64::MAX && self.last_seen_version < u64::MAX)
                {
                    self.last_seen_version = current_version;
                    let borrowed = self.receiver.borrow();
                    Ok(borrowed.as_ref().map(transform))
                } else {
                    Err(BusError::message_queue_empty())
                }
            }
            None => Err(BusError::topic_disconnected()),
        }
    }

    #[inline]
    fn get_or_refresh_topic(&mut self) -> Option<&Arc<Topic<T>>> {
        let should_clear_cache = if let Some(cached) = &self.cached_topic {
            Arc::strong_count(cached) <= 1
        } else {
            false
        };

        if should_clear_cache {
            self.cached_topic = None;
        }

        if self.cached_topic.is_some() {
            return self.cached_topic.as_ref();
        }

        match self.topic_ref.upgrade() {
            Some(topic) => {
                self.cached_topic = Some(topic);
                self.cached_topic.as_ref()
            }
            _ => None,
        }
    }
}

impl<T> PartialEq for Sub<T> {
    fn eq(&self, other: &Self) -> bool {
        self.topic_name == other.topic_name
    }
}

#[cfg(test)]
mod tests {
    use crate::topic::Topic;
    use std::sync::Arc;
    use tokio::sync::watch;

    #[test]
    fn test_version_comparison_overflow_protection() {
        let topic = Arc::new(Topic::<String>::new("test".to_string()));

        topic.set_version_for_test(u64::MAX);

        let mut subscriber = topic.subscribe();
        subscriber.last_seen_version = u64::MAX - 1;

        topic.publish("test message".to_string());

        let result = subscriber.try_get_message();
        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message, Some("test message".to_string()));
    }

    #[tokio::test]
    async fn test_wait_for_message_with_disconnected_topic() {
        let (sender, receiver) = watch::channel(None::<String>);
        let topic = Arc::new(Topic::<String>::new("test".to_string()));
        let weak_topic = Arc::downgrade(&topic);

        let mut subscriber = crate::sub::Sub::new(
            receiver,
            "test".to_string().into_boxed_str(),
            weak_topic,
            0,
            None,
        );

        drop(sender);

        let result = subscriber.wait_for_message().await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_wait_for_message_with_transform_disconnected() {
        let (sender, receiver) = watch::channel(None::<String>);
        let topic = Arc::new(Topic::<String>::new("test".to_string()));
        let weak_topic = Arc::downgrade(&topic);

        let mut subscriber = crate::sub::Sub::new(
            receiver,
            "test".to_string().into_boxed_str(),
            weak_topic,
            0,
            None,
        );

        drop(sender);

        let result = subscriber.wait_for_message_and_apply(|msg| msg.len()).await;
        assert!(result.is_none());
    }

    #[test]
    fn test_get_or_refresh_topic_with_dead_weak_ref() {
        let topic = Arc::new(Topic::<String>::new("test".to_string()));
        let weak_topic = Arc::downgrade(&topic);
        let (_, receiver) = watch::channel(None::<String>);

        let mut subscriber = crate::sub::Sub::new(
            receiver,
            "test".to_string().into_boxed_str(),
            weak_topic,
            0,
            None,
        );

        drop(topic);

        let result = subscriber.get_or_refresh_topic();
        assert!(result.is_none());
    }

    #[test]
    fn test_cached_topic_clear_on_low_count() {
        let topic = Arc::new(Topic::<String>::new("test".to_string()));
        let weak_topic = Arc::downgrade(&topic);
        let (_, receiver) = watch::channel(None::<String>);

        let mut subscriber = crate::sub::Sub::new(
            receiver,
            "test".to_string().into_boxed_str(),
            weak_topic,
            0,
            Some(topic.clone()),
        );

        drop(topic);

        let result = subscriber.get_or_refresh_topic();
        assert!(result.is_none());
    }
}
