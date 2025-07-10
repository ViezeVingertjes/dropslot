mod common;

use common::*;
use std::sync::Arc;

#[test]
fn test_topic_basic_properties() {
    let bus = create_string_bus();
    let topic = bus.topic("test_topic");

    assert_eq!(topic.name(), "test_topic");
    assert_eq!(topic.subscriber_count(), 0);
    assert!(!topic.has_subscribers());

    let debug_str = format!("{topic:?}");
    assert!(debug_str.contains("Topic"));
}

#[test]
fn test_topic_subscriber_management() {
    let bus = create_string_bus();
    let topic = bus.topic("sub_mgmt");

    let _sub1 = topic.subscribe();
    assert_eq!(topic.subscriber_count(), 1);
    assert!(topic.has_subscribers());

    let _sub2 = topic.subscribe();
    let _sub3 = topic.subscribe();
    assert_eq!(topic.subscriber_count(), 3);

    let subscribers: Vec<_> = (0..2).map(|_| topic.subscribe()).collect();
    assert_eq!(topic.subscriber_count(), 5);

    drop(subscribers);
    assert_eq!(topic.subscriber_count(), 3);
}

#[test]
fn test_topic_publish_operations() {
    let bus = create_string_bus();
    let topic = bus.topic("publish_test");

    topic.publish("Hello".to_string());

    topic.publish("Nobody listening".to_string());
}

#[test]
fn test_bytes_topic_operations() {
    let bus = create_bytes_bus();
    let topic = bus.topic("bytes_test");

    let slice_data = b"test data";
    topic.publish_slice(slice_data);

    let vec_data = vec![1, 2, 3, 4, 5];
    topic.publish_vec(vec_data);
}

#[tokio::test]
async fn test_publish_and_receive() {
    let bus = create_string_bus();
    let topic = bus.topic("pub_recv");
    let mut subscriber = topic.subscribe();

    topic.publish("Test Message".to_string());

    let message = subscriber.wait_for_message().await.unwrap();
    assert_eq!(message, "Test Message");
}

#[tokio::test]
async fn test_latest_message_semantics() {
    let bus = create_string_bus();
    let topic = bus.topic("latest_test");
    let mut subscriber = topic.subscribe();

    topic.publish("First".to_string());
    topic.publish("Second".to_string());
    topic.publish("Latest".to_string());

    let message = subscriber.wait_for_message().await.unwrap();
    assert_eq!(message, "Latest");
}

#[tokio::test]
async fn test_multiple_subscribers_same_message() {
    let bus = create_string_bus();
    let topic = bus.topic("broadcast");

    let mut sub1 = topic.subscribe();
    let mut sub2 = topic.subscribe();
    let mut sub3 = topic.subscribe();

    topic.publish("Broadcast".to_string());

    let msg1 = sub1.wait_for_message().await.unwrap();
    let msg2 = sub2.wait_for_message().await.unwrap();
    let msg3 = sub3.wait_for_message().await.unwrap();

    assert_eq!(msg1, "Broadcast");
    assert_eq!(msg2, "Broadcast");
    assert_eq!(msg3, "Broadcast");
}

#[test]
fn test_topic_concurrent_access() {
    let bus = create_string_bus_arc();
    run_concurrent_publishers(bus, "concurrent", 10, |i| format!("Message {i}"));
}

#[test]
fn test_topic_reuse_and_naming() {
    let bus = create_string_bus();
    let topic1 = bus.topic("immutable");
    let topic2 = bus.topic("immutable");

    assert!(Arc::ptr_eq(&topic1, &topic2));

    let name1 = topic1.name();
    let name2 = topic1.name();
    assert_eq!(name1, name2);
    assert_eq!(name1, "immutable");
}

#[test]
fn test_topic_edge_cases() {
    let bus = create_string_bus();

    let empty_topic = bus.topic("");
    assert_eq!(empty_topic.name(), "");

    let special_topic = bus.topic("!@#$%^&*()");
    assert_eq!(special_topic.name(), "!@#$%^&*()");

    let unicode_topic = bus.topic("测试🚀");
    assert_eq!(unicode_topic.name(), "测试🚀");

    let long_name = "a".repeat(1000);
    let long_topic = bus.topic(&long_name);
    assert_eq!(long_topic.name(), long_name);
}
