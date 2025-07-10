use bytes::Bytes;
use dropslot::Bus;
use std::sync::Arc;

#[test]
fn test_bus_creation() {
    let bus = Bus::<String>::new();
    assert_eq!(bus.topic_count(), 0);
}

#[test]
fn test_bus_with_capacity() {
    let bus = Bus::<String>::with_capacity(32);
    assert_eq!(bus.topic_count(), 0);
}

#[test]
fn test_bus_high_throughput() {
    let bus = Bus::<String>::high_throughput();
    assert_eq!(bus.topic_count(), 0);
}

#[test]
fn test_bus_low_latency() {
    let bus = Bus::<String>::low_latency();
    assert_eq!(bus.topic_count(), 0);
}

#[test]
fn test_bus_default() {
    let bus = Bus::<String>::default();
    assert_eq!(bus.topic_count(), 0);
}

#[test]
fn test_topic_creation() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("test");
    assert_eq!(topic.name(), "test");
    assert_eq!(bus.topic_count(), 1);
}

#[test]
fn test_topic_reuse() {
    let bus = Bus::<String>::new();
    let topic1 = bus.topic("same");
    let topic2 = bus.topic("same");

    assert!(Arc::ptr_eq(&topic1, &topic2));
    assert_eq!(bus.topic_count(), 1);
}

#[test]
fn test_multiple_topics() {
    let bus = Bus::<String>::new();
    let _topic1 = bus.topic("topic1");
    let _topic2 = bus.topic("topic2");
    let _topic3 = bus.topic("topic3");

    assert_eq!(bus.topic_count(), 3);
}

#[test]
fn test_topic_names() {
    let bus = Bus::<String>::new();
    let _topic1 = bus.topic("alpha");
    let _topic2 = bus.topic("beta");
    let _topic3 = bus.topic("gamma");

    let names = bus.topic_names();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&"alpha".to_string()));
    assert!(names.contains(&"beta".to_string()));
    assert!(names.contains(&"gamma".to_string()));
}

#[test]
fn test_publish_creates_topic() {
    let bus = Bus::<String>::new();
    assert_eq!(bus.topic_count(), 0);

    bus.publish("new_topic", "Hello".to_string());
    assert_eq!(bus.topic_count(), 1);
    assert!(bus.topic_names().contains(&"new_topic".to_string()));
}

#[test]
fn test_subscribe_creates_topic() {
    let bus = Bus::<String>::new();
    assert_eq!(bus.topic_count(), 0);

    let _subscriber = bus.subscribe("new_topic");
    assert_eq!(bus.topic_count(), 1);
    assert!(bus.topic_names().contains(&"new_topic".to_string()));
}

#[test]
fn test_remove_topic() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("removable");
    let _sub = topic.subscribe();

    assert_eq!(bus.topic_count(), 1);

    let subscriber_count = bus.remove_topic("removable");
    assert_eq!(subscriber_count, Some(1));
    assert_eq!(bus.topic_count(), 0);
}

#[test]
fn test_remove_nonexistent_topic() {
    let bus = Bus::<String>::new();
    let result = bus.remove_topic("nonexistent");
    assert_eq!(result, None);
}

#[test]
fn test_cleanup_unused_topics() {
    let bus = Bus::<String>::new();
    let topic1 = bus.topic("keep");
    let topic2 = bus.topic("remove");

    let _keeper = topic1.subscribe();
    let remover = topic2.subscribe();

    assert_eq!(bus.topic_count(), 2);

    drop(remover);
    let removed = bus.cleanup_unused_topics();
    assert_eq!(removed, 1);
    assert_eq!(bus.topic_count(), 1);
}

#[test]
fn test_cleanup_all_unused() {
    let bus = Bus::<String>::new();
    let topic1 = bus.topic("temp1");
    let topic2 = bus.topic("temp2");

    let sub1 = topic1.subscribe();
    let sub2 = topic2.subscribe();

    drop(sub1);
    drop(sub2);

    let removed = bus.cleanup_unused_topics();
    assert_eq!(removed, 2);
    assert_eq!(bus.topic_count(), 0);
}

#[test]
fn test_cleanup_no_unused() {
    let bus = Bus::<String>::new();
    let topic = bus.topic("active");
    let _sub = topic.subscribe();

    let removed = bus.cleanup_unused_topics();
    assert_eq!(removed, 0);
    assert_eq!(bus.topic_count(), 1);
}

#[test]
fn test_bytes_bus_operations() {
    let bus = Bus::<Bytes>::new();

    let data = b"test data";
    bus.publish_slice("bytes_topic", data);

    let vec_data = vec![1, 2, 3, 4, 5];
    bus.publish_vec("bytes_topic", vec_data);

    assert_eq!(bus.topic_count(), 1);
}

#[test]
fn test_concurrent_topic_access() {
    use std::sync::Arc;
    use std::thread;

    let bus = Arc::new(Bus::<String>::new());
    let mut handles = vec![];

    for i in 0..10 {
        let bus_clone = bus.clone();
        let handle = thread::spawn(move || {
            let topic = bus_clone.topic(&format!("topic{i}"));
            topic.name().to_string()
        });
        handles.push(handle);
    }

    let results: Vec<String> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    assert_eq!(results.len(), 10);
    assert_eq!(bus.topic_count(), 10);
}

#[test]
fn test_topic_name_consistency() {
    let bus = Bus::<String>::new();
    let topic_name = "consistency_test";
    let topic = bus.topic(topic_name);

    assert_eq!(topic.name(), topic_name);
    assert!(bus.topic_names().contains(&topic_name.to_string()));
}

#[test]
fn test_bus_with_large_capacity() {
    let bus = Bus::<String>::with_capacity(1024);

    for i in 0..100 {
        let _ = bus.topic(&format!("topic{i}"));
    }

    assert_eq!(bus.topic_count(), 100);
}
