mod common;

use bytes::Bytes;
use common::*;

#[test]
fn test_subscriber_basic_properties() {
    let bus = create_string_bus();
    let topic1 = bus.topic("sub_test");
    let topic2 = bus.topic("different_topic");

    let subscriber1 = topic1.subscribe();
    let subscriber2 = topic1.subscribe();
    let subscriber3 = topic2.subscribe();

    assert_eq!(subscriber1.topic_name(), "sub_test");
    assert!(!subscriber1.has_latest());

    assert_eq!(subscriber1, subscriber2);
    assert_ne!(subscriber1, subscriber3);

    let debug_str = format!("{subscriber1:?}");
    assert!(debug_str.contains("Sub"));
}

#[tokio::test]
async fn test_wait_for_message_scenarios() {
    let bus = create_string_bus();
    let topic = bus.topic("wait_test");
    let mut subscriber = topic.subscribe();

    topic.publish("Hello".to_string());
    let message = subscriber.wait_for_message().await.unwrap();
    assert_eq!(message, "Hello");

    let mut subscriber2 = topic.subscribe();
    topic.publish("First".to_string());
    topic.publish("Second".to_string());
    topic.publish("Third".to_string());
    let message = subscriber2.wait_for_message().await.unwrap();
    assert_eq!(message, "Third");

    let mut subscriber3 = topic.subscribe();
    topic.publish("hello".to_string());
    let length = subscriber3
        .wait_for_message_and_apply(|msg| msg.len())
        .await
        .unwrap();
    assert_eq!(length, 5);
}

#[test]
fn test_try_get_message_scenarios() {
    let bus = create_string_bus();
    let topic = bus.topic("try_test");
    let mut subscriber = topic.subscribe();

    let result = subscriber.try_get_message();
    assert!(result.is_err());
    assert!(result.unwrap_err().is_empty());

    topic.publish("Try Message".to_string());
    let result = subscriber.try_get_message();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("Try Message".to_string()));

    let result = subscriber.try_get_message();
    assert!(result.is_err());

    topic.publish("transform".to_string());
    let result = subscriber.try_get_message_and_apply(|msg| msg.to_uppercase());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("TRANSFORM".to_string()));
}

#[test]
fn test_get_latest_operations() {
    let bus = create_string_bus();
    let topic = bus.topic("get_latest");
    let subscriber = topic.subscribe();

    assert!(subscriber.get_latest().is_none());
    assert!(!subscriber.has_latest());

    topic.publish("Latest Message".to_string());

    assert_eq!(subscriber.get_latest(), Some("Latest Message".to_string()));
    assert!(subscriber.has_latest());

    let word_count = subscriber.get_latest_with(|msg| msg.split_whitespace().count());
    assert_eq!(word_count, Some(2));
}

#[test]
fn test_subscriber_after_topic_drop() {
    let bus = create_string_bus();
    let topic = bus.topic("drop_test");
    let mut subscriber = topic.subscribe();

    drop(topic);
    bus.remove_topic("drop_test");

    let result = subscriber.try_get_message();
    assert!(result.is_err());
    assert!(result.unwrap_err().is_disconnected());
}

#[test]
fn test_subscriber_version_tracking() {
    let bus = create_string_bus();
    let topic = bus.topic("version_test");
    let mut subscriber = topic.subscribe();

    topic.publish("Message 1".to_string());
    let result1 = subscriber.try_get_message();
    assert!(result1.is_ok());

    let result2 = subscriber.try_get_message();
    assert!(result2.is_err());
    assert!(result2.unwrap_err().is_empty());

    topic.publish("Message 2".to_string());
    let result3 = subscriber.try_get_message();
    assert!(result3.is_ok());
}

#[test]
fn test_subscriber_with_bytes() {
    let bus = create_bytes_bus();
    let topic = bus.topic("bytes_test");
    let mut subscriber = topic.subscribe();

    let data = Bytes::from("test bytes data");
    topic.publish(data.clone());

    let result = subscriber.try_get_message();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(data));
}

#[tokio::test]
async fn test_subscriber_timeout() {
    let bus = create_string_bus();
    let topic = bus.topic("timeout_test");
    let mut subscriber = topic.subscribe();

    assert_timeout_on_wait(&mut subscriber, 10).await;
}

#[tokio::test]
async fn test_subscriber_after_publish() {
    let bus = create_string_bus();
    let topic = bus.topic("after_publish");

    topic.publish("Early Message".to_string());

    let mut subscriber = topic.subscribe();

    topic.publish("After Subscribe".to_string());

    let message = subscriber.wait_for_message().await.unwrap();
    assert_eq!(message, "After Subscribe");
}

#[test]
fn test_subscriber_concurrent_access() {
    let bus = create_string_bus_arc();
    let topic = bus.topic("concurrent_test");
    let mut subscribers = Vec::new();

    for _ in 0..5 {
        subscribers.push(topic.subscribe());
    }

    run_concurrent_publishers(bus, "concurrent_test", 10, |i| format!("Message {i}"));

    assert_eq!(topic.subscriber_count(), 5);
}

#[test]
fn test_subscriber_edge_cases() {
    let bus = create_string_bus();
    let topic = bus.topic(&"a".repeat(1000));
    let subscriber = topic.subscribe();
    assert_eq!(subscriber.topic_name().len(), 1000);

    let unicode_topic = bus.topic("测试🚀");
    let unicode_subscriber = unicode_topic.subscribe();
    assert_eq!(unicode_subscriber.topic_name(), "测试🚀");
}
