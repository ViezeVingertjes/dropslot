mod common;

use common::*;
use dropslot::Bus;
use std::sync::Arc;

#[test]
fn test_bus_creation_variants() {
    let default_bus = Bus::<String>::new();
    assert_eq!(default_bus.topic_count(), 0);

    let default_bus2 = Bus::<String>::default();
    assert_eq!(default_bus2.topic_count(), 0);

    let capacity_bus = Bus::<String>::with_capacity(32);
    assert_eq!(capacity_bus.topic_count(), 0);

    let high_throughput_bus = Bus::<String>::with_capacity(64);
    assert_eq!(high_throughput_bus.topic_count(), 0);

    let low_latency_bus = Bus::<String>::with_capacity(8);
    assert_eq!(low_latency_bus.topic_count(), 0);
}

#[test]
fn test_topic_lifecycle() {
    let bus = create_string_bus();

    assert_eq!(bus.topic_count(), 0);

    let topic = bus.topic("test_topic");
    assert_eq!(topic.name(), "test_topic");
    assert_eq!(bus.topic_count(), 1);

    let topic_reuse = bus.topic("test_topic");
    assert!(Arc::ptr_eq(&topic, &topic_reuse));
    assert_eq!(bus.topic_count(), 1);
}

#[test]
fn test_multiple_topics_management() {
    let bus = create_string_bus();
    let _topic1 = bus.topic("alpha");
    let _topic2 = bus.topic("beta");
    let _topic3 = bus.topic("gamma");

    assert_topic_counts(&bus, 3, &["alpha", "beta", "gamma"]);
}

#[test]
fn test_topic_creation_via_operations() {
    let bus = create_string_bus();
    assert_eq!(bus.topic_count(), 0);

    bus.publish("publish_topic", "Hello".to_string());
    assert_eq!(bus.topic_count(), 1);
    assert!(bus.topic_names().contains(&"publish_topic".to_string()));

    let _subscriber = bus.subscribe("subscribe_topic");
    assert_eq!(bus.topic_count(), 2);
    assert!(bus.topic_names().contains(&"subscribe_topic".to_string()));
}

#[test]
fn test_topic_removal() {
    let bus = create_string_bus();

    let topic = bus.topic("removable");
    let _sub = topic.subscribe();
    assert_eq!(bus.topic_count(), 1);

    let subscriber_count = bus.remove_topic("removable");
    assert_eq!(subscriber_count, Some(1));
    assert_eq!(bus.topic_count(), 0);

    let result = bus.remove_topic("nonexistent");
    assert_eq!(result, None);
}

#[test]
fn test_cleanup_scenarios() {
    let bus = create_string_bus();

    let topic1 = bus.topic("keep");
    let topic2 = bus.topic("remove1");
    let topic3 = bus.topic("remove2");

    let _keeper = topic1.subscribe();
    let remover1 = topic2.subscribe();
    let remover2 = topic3.subscribe();

    assert_eq!(bus.topic_count(), 3);

    drop(remover1);
    drop(remover2);
    let removed = bus.cleanup_unused_topics();
    assert_eq!(removed, 2);
    assert_eq!(bus.topic_count(), 1);

    let removed = bus.cleanup_unused_topics();
    assert_eq!(removed, 0);
}

#[test]
fn test_bytes_operations() {
    let bus = create_bytes_bus();

    let slice_data = b"test data";
    bus.publish_slice("bytes_topic", slice_data);

    let vec_data = vec![1, 2, 3, 4, 5];
    bus.publish_vec("bytes_topic", vec_data);

    assert_eq!(bus.topic_count(), 1);
}

#[test]
fn test_concurrent_access() {
    let bus = create_string_bus_arc();
    let results = run_concurrent_topic_creators(bus.clone(), 10);

    assert_eq!(results.len(), 10);
    assert_eq!(bus.topic_count(), 10);
}
