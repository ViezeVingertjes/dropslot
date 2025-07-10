use bytes::Bytes;
use dropslot::Bus;
use std::sync::Arc;

#[test]
fn test_topic_creation() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("test_topic");
    assert_eq!(topic.name(), "test_topic");
    assert_eq!(topic.subscriber_count(), 0);
    assert!(!topic.has_subscribers());
}

#[test]
fn test_topic_name() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("my_topic");
    assert_eq!(topic.name(), "my_topic");
}

#[test]
fn test_topic_publish() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("publish_test");

    topic.publish("Hello".to_string());
    // Publishing doesn't fail or panic
}

#[test]
fn test_topic_subscribe() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("subscribe_test");

    assert_eq!(topic.subscriber_count(), 0);

    let _subscriber = topic.subscribe();
    assert_eq!(topic.subscriber_count(), 1);
    assert!(topic.has_subscribers());
}

#[test]
fn test_multiple_subscribers() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("multi_sub");

    let _sub1 = topic.subscribe();
    let _sub2 = topic.subscribe();
    let _sub3 = topic.subscribe();

    assert_eq!(topic.subscriber_count(), 3);
    assert!(topic.has_subscribers());
}

#[test]
fn test_subscriber_drop() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("drop_test");

    let sub1 = topic.subscribe();
    let sub2 = topic.subscribe();

    assert_eq!(topic.subscriber_count(), 2);

    drop(sub1);
    assert_eq!(topic.subscriber_count(), 1);

    drop(sub2);
    assert_eq!(topic.subscriber_count(), 0);
    assert!(!topic.has_subscribers());
}

#[test]
fn test_topic_publish_to_no_subscribers() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("no_subs");

    topic.publish("Nobody listening".to_string());
    // Should not panic or fail
}

#[test]
fn test_bytes_topic_operations() {
    let bus = Bus::<Bytes>::new();
    let topic = bus.topic("bytes_test");

    let data = b"test data";
    topic.publish_slice(data);

    let vec_data = vec![1, 2, 3, 4, 5];
    topic.publish_vec(vec_data);

    // Operations complete without error
}

#[tokio::test]
async fn test_publish_and_receive() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("pub_recv");
    let mut subscriber = topic.subscribe();

    topic.publish("Test Message".to_string());

    let message = subscriber.next().await.unwrap();
    assert_eq!(message, "Test Message");
}

#[tokio::test]
async fn test_latest_message_semantics() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("latest_test");
    let mut subscriber = topic.subscribe();

    topic.publish("First".to_string());
    topic.publish("Second".to_string());
    topic.publish("Latest".to_string());

    let message = subscriber.next().await.unwrap();
    assert_eq!(message, "Latest");
}

#[tokio::test]
async fn test_multiple_subscribers_same_message() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("broadcast");

    let mut sub1 = topic.subscribe();
    let mut sub2 = topic.subscribe();
    let mut sub3 = topic.subscribe();

    topic.publish("Broadcast".to_string());

    let msg1 = sub1.next().await.unwrap();
    let msg2 = sub2.next().await.unwrap();
    let msg3 = sub3.next().await.unwrap();

    assert_eq!(msg1, "Broadcast");
    assert_eq!(msg2, "Broadcast");
    assert_eq!(msg3, "Broadcast");
}

#[test]
fn test_topic_debug_format() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("debug_test");

    let debug_str = format!("{topic:?}");
    assert!(debug_str.contains("Topic"));
}

#[test]
fn test_topic_concurrent_access() {
    use std::sync::Arc;
    use std::thread;

    let bus = Bus::<String>::new();
    let topic = Arc::new(bus.topic("concurrent"));
    let mut handles = vec![];

    for i in 0..10 {
        let topic_clone = topic.clone();
        let handle = thread::spawn(move || {
            topic_clone.publish(format!("Message {i}"));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // All operations complete successfully
}

#[test]
fn test_topic_subscriber_management() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("sub_mgmt");

    let subscribers: Vec<_> = (0..5).map(|_| topic.subscribe()).collect();
    assert_eq!(topic.subscriber_count(), 5);

    drop(subscribers);
    assert_eq!(topic.subscriber_count(), 0);
}

#[test]
fn test_topic_name_immutability() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("immutable");

    let name1 = topic.name();
    let name2 = topic.name();

    assert_eq!(name1, name2);
    assert_eq!(name1, "immutable");
}

#[test]
fn test_topic_with_empty_name() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("");

    assert_eq!(topic.name(), "");
    assert_eq!(topic.subscriber_count(), 0);
}

#[test]
fn test_topic_with_special_characters() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("special!@#$%^&*()_+-=[]{}|;:,.<>?");

    assert_eq!(topic.name(), "special!@#$%^&*()_+-=[]{}|;:,.<>?");
    assert_eq!(topic.subscriber_count(), 0);
}

#[test]
fn test_topic_with_unicode() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("测试主题🚀");

    assert_eq!(topic.name(), "测试主题🚀");
    assert_eq!(topic.subscriber_count(), 0);
}

#[test]
fn test_topic_reuse_same_instance() {
    let bus = Bus::<String>::new();
    let topic1 = bus.topic("same");
    let topic2 = bus.topic("same");

    assert!(Arc::ptr_eq(&topic1, &topic2));
    assert_eq!(topic1.name(), topic2.name());
}

#[test]
fn test_topic_long_name() {
    let bus = Bus::<String>::new();
    let long_name = "a".repeat(1000);
    let topic = bus.topic(&long_name);

    assert_eq!(topic.name(), long_name);
    assert_eq!(topic.subscriber_count(), 0);
}
