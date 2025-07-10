use bytes::Bytes;
use dropslot::Bus;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_basic_pubsub_flow() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("test");
    let mut subscriber = topic.subscribe();

    topic.publish("Hello".to_string());

    let message = subscriber.next().await.unwrap();
    assert_eq!(message, "Hello");
}

#[tokio::test]
async fn test_latest_only_semantics() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("test");
    let mut subscriber = topic.subscribe();

    topic.publish("First".to_string());
    topic.publish("Second".to_string());
    topic.publish("Third".to_string());

    let message = subscriber.next().await.unwrap();
    assert_eq!(message, "Third");
}

#[tokio::test]
async fn test_multiple_subscribers() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("broadcast");

    let mut sub1 = topic.subscribe();
    let mut sub2 = topic.subscribe();
    let mut sub3 = topic.subscribe();

    topic.publish("Broadcast Message".to_string());

    let msg1 = sub1.next().await.unwrap();
    let msg2 = sub2.next().await.unwrap();
    let msg3 = sub3.next().await.unwrap();

    assert_eq!(msg1, "Broadcast Message");
    assert_eq!(msg2, "Broadcast Message");
    assert_eq!(msg3, "Broadcast Message");
}

#[tokio::test]
async fn test_topic_subscriber_count() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("count_test");

    assert_eq!(topic.subscriber_count(), 0);

    let _sub1 = topic.subscribe();
    assert_eq!(topic.subscriber_count(), 1);

    let _sub2 = topic.subscribe();
    assert_eq!(topic.subscriber_count(), 2);

    drop(_sub1);
    assert_eq!(topic.subscriber_count(), 1);
}

#[tokio::test]
async fn test_bus_topic_management() {
    let bus = Bus::<String>::new();

    assert_eq!(bus.topic_count(), 0);

    let _topic1 = bus.topic("topic1");
    let _topic2 = bus.topic("topic2");

    assert_eq!(bus.topic_count(), 2);

    let names = bus.topic_names();
    assert!(names.contains(&"topic1".to_string()));
    assert!(names.contains(&"topic2".to_string()));
}

#[tokio::test]
async fn test_topic_cleanup() {
    let bus = Bus::<String>::new();

    let topic1 = bus.topic("cleanup1");
    let topic2 = bus.topic("cleanup2");

    let _sub1 = topic1.subscribe();
    let _sub2 = topic2.subscribe();

    assert_eq!(bus.topic_count(), 2);

    drop(_sub2);
    let removed = bus.cleanup_unused_topics();
    assert_eq!(removed, 1);
    assert_eq!(bus.topic_count(), 1);
}

#[tokio::test]
async fn test_custom_message_types() {
    #[derive(Debug, Clone, PartialEq)]
    struct CustomMessage {
        id: u64,
        content: String,
    }

    let bus = Bus::<CustomMessage>::new();
    let topic = bus.topic("custom");
    let mut subscriber = topic.subscribe();

    let msg = CustomMessage {
        id: 42,
        content: "Custom content".to_string(),
    };

    topic.publish(msg.clone());

    let received = subscriber.next().await.unwrap();
    assert_eq!(received, msg);
}

#[tokio::test]
async fn test_bytes_zero_copy() {
    let bus = Bus::<Bytes>::new();
    let topic = bus.topic("bytes");
    let mut subscriber = topic.subscribe();

    let original_data = b"zero copy test data";
    topic.publish_slice(original_data);

    let received = subscriber.next().await.unwrap();
    assert_eq!(received.as_ref(), original_data);
}

#[tokio::test]
async fn test_performance_configurations() {
    let _high_throughput = Bus::<String>::high_throughput();
    let _low_latency = Bus::<String>::low_latency();
    let _custom_capacity = Bus::<String>::with_capacity(256);

    // Just ensure they can be created without panicking
}

#[tokio::test]
async fn test_async_timeout_scenarios() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("timeout_test");
    let mut subscriber = topic.subscribe();

    let timeout_result = timeout(Duration::from_millis(10), subscriber.next()).await;
    assert!(timeout_result.is_err());

    topic.publish("Finally!".to_string());

    let message = subscriber.next().await.unwrap();
    assert_eq!(message, "Finally!");
}

#[test]
fn test_topic_equality() {
    let bus = Bus::<String>::new();
    let topic1 = bus.topic("same");
    let topic2 = bus.topic("same");

    assert!(Arc::ptr_eq(&topic1, &topic2));
}

#[test]
fn test_subscriber_equality() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("test");
    let sub1 = topic.subscribe();
    let sub2 = topic.subscribe();

    assert_eq!(sub1, sub2); // Same topic name
}

#[test]
fn test_default_bus_creation() {
    let bus1 = Bus::<String>::new();
    let bus2 = Bus::<String>::default();

    assert_eq!(bus1.topic_count(), bus2.topic_count());
}
