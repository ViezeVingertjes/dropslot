use bytes::Bytes;
use dropslot::{Bus, BusError};
use std::time::Duration;
use tokio::time::timeout;

#[test]
fn test_subscriber_creation() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("sub_test");
    let subscriber = topic.subscribe();

    assert_eq!(subscriber.topic_name(), "sub_test");
    assert!(!subscriber.has_latest());
}

#[test]
fn test_subscriber_topic_name() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("my_topic");
    let subscriber = topic.subscribe();

    assert_eq!(subscriber.topic_name(), "my_topic");
}

#[test]
fn test_subscriber_equality() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("equal_test");
    let sub1 = topic.subscribe();
    let sub2 = topic.subscribe();

    assert_eq!(sub1, sub2); // Same topic name
}

#[test]
fn test_subscriber_inequality() {
    let bus = Bus::<String>::new();
    let topic1 = bus.topic("topic1");
    let topic2 = bus.topic("topic2");
    let sub1 = topic1.subscribe();
    let sub2 = topic2.subscribe();

    assert_ne!(sub1, sub2); // Different topic names
}

#[test]
fn test_subscriber_debug_format() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("debug_test");
    let subscriber = topic.subscribe();

    let debug_str = format!("{subscriber:?}");
    assert!(debug_str.contains("Sub"));
}

#[tokio::test]
async fn test_wait_for_message_with_message() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("next_test");
    let mut subscriber = topic.subscribe();

    topic.publish("Hello".to_string());

    let message = subscriber.wait_for_message().await.unwrap();
    assert_eq!(message, "Hello");
}

#[tokio::test]
async fn test_wait_for_message_with_multiple_messages() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("multi_next");
    let mut subscriber = topic.subscribe();

    topic.publish("First".to_string());
    topic.publish("Second".to_string());
    topic.publish("Third".to_string());

    let message = subscriber.wait_for_message().await.unwrap();
    assert_eq!(message, "Third"); // Latest only
}

#[tokio::test]
async fn test_wait_for_message_with_transformation() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("transform_test");
    let mut subscriber = topic.subscribe();

    topic.publish("hello".to_string());

    let length = subscriber.wait_for_message_and_apply(|msg| msg.len()).await.unwrap();
    assert_eq!(length, 5);
}

#[tokio::test]
async fn test_recv_alias() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("recv_test");
    let mut subscriber = topic.subscribe();

    topic.publish("Recv Test".to_string());

    let message = subscriber.wait_for_message().await.unwrap();
    assert_eq!(message, "Recv Test");
}

#[tokio::test]
async fn test_recv_with_alias() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("recv_with_test");
    let mut subscriber = topic.subscribe();

    topic.publish("UPPERCASE".to_string());

    let lowercase = subscriber
        .wait_for_message_and_apply(|msg| msg.to_lowercase())
        .await
        .unwrap();
    assert_eq!(lowercase, "uppercase");
}

#[test]
fn test_try_get_message_empty() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("try_empty");
    let mut subscriber = topic.subscribe();

    let result = subscriber.try_get_message();
    assert!(result.is_err());
    assert!(result.unwrap_err().is_empty());
}

#[test]
fn test_try_get_message_with_message() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("try_with_msg");
    let mut subscriber = topic.subscribe();

    topic.publish("Try Message".to_string());

    let result = subscriber.try_get_message();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("Try Message".to_string()));
}

#[test]
fn test_try_get_message_with_transformation() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("try_transform");
    let mut subscriber = topic.subscribe();

    topic.publish("transform".to_string());

    let result = subscriber.try_get_message_and_apply(|msg| msg.to_uppercase());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("TRANSFORM".to_string()));
}

#[test]
fn test_try_get_message_alias_alias() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("try_recv");
    let mut subscriber = topic.subscribe();

    topic.publish("Try Recv".to_string());

    let result = subscriber.try_get_message();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("Try Recv".to_string()));
}

#[test]
fn test_try_get_message_alias_with_alias() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("try_recv_with");
    let mut subscriber = topic.subscribe();

    topic.publish("test".to_string());

    let result = subscriber.try_get_message_and_apply(|msg| msg.len());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(4));
}

#[test]
fn test_get_latest_empty() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("get_empty");
    let subscriber = topic.subscribe();

    let latest = subscriber.get_latest();
    assert!(latest.is_none());
}

#[test]
fn test_get_latest_with_message() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("get_latest");
    let subscriber = topic.subscribe();

    topic.publish("Latest Message".to_string());

    let latest = subscriber.get_latest();
    assert_eq!(latest, Some("Latest Message".to_string()));
}

#[test]
fn test_get_latest_with_transformation() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("get_latest_with");
    let subscriber = topic.subscribe();

    topic.publish("hello world".to_string());

    let word_count = subscriber.get_latest_with(|msg| msg.split_whitespace().count());
    assert_eq!(word_count, Some(2));
}

#[test]
fn test_has_latest_empty() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("has_empty");
    let subscriber = topic.subscribe();

    assert!(!subscriber.has_latest());
}

#[test]
fn test_has_latest_with_message() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("has_latest");
    let subscriber = topic.subscribe();

    topic.publish("Has Latest".to_string());

    assert!(subscriber.has_latest());
}

#[test]
fn test_try_get_message_after_topic_drop() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("drop_topic");
    let mut subscriber = topic.subscribe();

    // Remove the topic from the bus to simulate disconnection
    drop(topic); // Drop the local reference first
    let removed_count = bus.remove_topic("drop_topic");
    assert_eq!(removed_count, Some(1));

    let result = subscriber.try_get_message();
    assert!(result.is_err());
    assert!(result.unwrap_err().is_disconnected());
}

#[test]
fn test_try_get_message_version_tracking() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("version_test");
    let mut subscriber = topic.subscribe();

    topic.publish("First".to_string());

    let result1 = subscriber.try_get_message();
    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), Some("First".to_string()));

    let result2 = subscriber.try_get_message();
    assert!(result2.is_err());
    assert!(result2.unwrap_err().is_empty());
}

#[test]
fn test_subscriber_with_bytes() {
    let bus = Bus::<Bytes>::new();
    let topic = bus.topic("bytes_sub");
    let subscriber = topic.subscribe();

    let data = b"byte data";
    topic.publish_slice(data);

    let latest = subscriber.get_latest().unwrap();
    assert_eq!(latest.as_ref(), data);
}

#[tokio::test]
async fn test_subscriber_timeout() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("timeout_test");
    let mut subscriber = topic.subscribe();

    let result = timeout(Duration::from_millis(10), subscriber.wait_for_message()).await;
    assert!(result.is_err()); // Timeout
}

#[tokio::test]
async fn test_subscriber_after_publish() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("after_pub");

    topic.publish("Already Published".to_string());

    let mut subscriber = topic.subscribe();

    // Subscribing after publishing should NOT give us the previous message
    // This should timeout because no new messages are published
    let result = timeout(Duration::from_millis(10), subscriber.wait_for_message()).await;
    assert!(result.is_err()); // Should timeout

    // But if we publish a new message, the subscriber should receive it
    topic.publish("New Message".to_string());
    let message = subscriber.wait_for_message().await.unwrap();
    assert_eq!(message, "New Message");
}

#[test]
fn test_subscriber_concurrent_access() {
    use std::sync::Arc;
    use std::sync::mpsc;
    use std::thread;

    let bus = Bus::<String>::new();
    let topic = Arc::new(bus.topic("concurrent_sub"));
    let mut handles = vec![];
    let (tx, rx) = mpsc::channel();

    for _i in 0..10 {
        let topic_clone = topic.clone();
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let subscriber = topic_clone.subscribe();
            // Send the subscriber back to keep it alive
            tx_clone.send(subscriber).unwrap();
        });
        handles.push(handle);
    }

    // Collect all subscribers to keep them alive
    let mut subscribers = Vec::new();
    for _ in 0..10 {
        let subscriber = rx.recv().unwrap();
        subscribers.push(subscriber);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(topic.subscriber_count(), 10);

    // Drop subscribers at the end
    drop(subscribers);
}

#[test]
fn test_error_display() {
    let empty_error = BusError::message_queue_empty();
    let disconnected_error = BusError::topic_disconnected();

    assert!(empty_error.to_string().contains("No message available"));
    assert!(
        disconnected_error
            .to_string()
            .contains("Topic disconnected")
    );
}

#[test]
fn test_error_is_methods() {
    let empty_error = BusError::message_queue_empty();
    let disconnected_error = BusError::topic_disconnected();

    assert!(empty_error.is_empty());
    assert!(!empty_error.is_disconnected());

    assert!(!disconnected_error.is_empty());
    assert!(disconnected_error.is_disconnected());
}

#[test]
fn test_subscriber_with_long_topic_name() {
    let bus = Bus::<String>::new();
    let long_name = "a".repeat(1000);
    let topic = bus.topic(&long_name);
    let subscriber = topic.subscribe();

    assert_eq!(subscriber.topic_name(), long_name);
}

#[test]
fn test_subscriber_with_unicode_topic() {
    let bus = Bus::<String>::new();
    let unicode_name = "测试主题🚀";
    let topic = bus.topic(unicode_name);
    let subscriber = topic.subscribe();

    assert_eq!(subscriber.topic_name(), unicode_name);
}
