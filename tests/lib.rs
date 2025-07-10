mod common;

use common::*;
use dropslot::Bus;

#[tokio::test]
async fn test_basic_pubsub_flow() {
    let bus = create_string_bus();
    let topic = bus.topic("test_topic");
    let mut subscriber = topic.subscribe();

    topic.publish("Hello".to_string());

    let message = subscriber.wait_for_message().await.unwrap();
    assert_eq!(message, "Hello");
}

#[tokio::test]
async fn test_latest_only_semantics() {
    let bus = create_string_bus();
    let topic = bus.topic("latest_test");
    let mut subscriber = topic.subscribe();

    topic.publish("First".to_string());
    topic.publish("Second".to_string());
    topic.publish("Third".to_string());

    let message = subscriber.wait_for_message().await.unwrap();
    assert_eq!(message, "Third");
}

#[tokio::test]
async fn test_multiple_subscribers() {
    let bus = create_string_bus();
    let topic = bus.topic("broadcast");

    let mut sub1 = topic.subscribe();
    let mut sub2 = topic.subscribe();
    let mut sub3 = topic.subscribe();

    topic.publish("Broadcast Message".to_string());

    let msg1 = sub1.wait_for_message().await.unwrap();
    let msg2 = sub2.wait_for_message().await.unwrap();
    let msg3 = sub3.wait_for_message().await.unwrap();

    assert_eq!(msg1, "Broadcast Message");
    assert_eq!(msg2, "Broadcast Message");
    assert_eq!(msg3, "Broadcast Message");
}

#[tokio::test]
async fn test_topic_subscriber_count() {
    let bus = create_string_bus();
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
    let bus = create_string_bus();

    assert_eq!(bus.topic_count(), 0);

    let _topic1 = bus.topic("topic1");
    let _topic2 = bus.topic("topic2");

    assert_topic_counts(&bus, 2, &["topic1", "topic2"]);
}

#[tokio::test]
async fn test_topic_cleanup() {
    let bus = create_string_bus();

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

    let received = subscriber.wait_for_message().await.unwrap();
    assert_eq!(received, msg);
}

#[tokio::test]
async fn test_bytes_zero_copy() {
    let bus = create_bytes_bus();
    let topic = bus.topic("bytes");
    let mut subscriber = topic.subscribe();

    let original_data = b"zero copy test data";
    topic.publish_slice(original_data);

    let received = subscriber.wait_for_message().await.unwrap();
    assert_eq!(received.as_ref(), original_data);
}

#[tokio::test]
async fn test_performance_configurations() {
    let _high_throughput = Bus::<String>::with_capacity(64);
    let _low_latency = Bus::<String>::with_capacity(8);
    let _custom_capacity = Bus::<String>::with_capacity(256);
}

#[tokio::test]
async fn test_async_timeout_scenarios() {
    let bus = create_string_bus();
    let topic = bus.topic("timeout_test");
    let mut subscriber = topic.subscribe();

    assert_timeout_on_wait(&mut subscriber, 10).await;

    topic.publish("Finally!".to_string());

    let message = subscriber.wait_for_message().await.unwrap();
    assert_eq!(message, "Finally!");
}
