#![allow(dead_code)]

use bytes::Bytes;
use dropslot::{Bus, BusError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::time::timeout;

pub fn create_string_bus() -> Bus<String> {
    Bus::<String>::new()
}

pub fn create_bytes_bus() -> Bus<Bytes> {
    Bus::<Bytes>::new()
}

pub fn assert_error_properties(
    error: &BusError,
    should_be_empty: bool,
    should_be_disconnected: bool,
) {
    assert_eq!(error.is_empty(), should_be_empty);
    assert_eq!(error.is_disconnected(), should_be_disconnected);
}

pub fn create_string_bus_arc() -> Arc<Bus<String>> {
    Arc::new(Bus::<String>::new())
}

pub async fn assert_timeout_on_wait(subscriber: &mut dropslot::Sub<String>, timeout_ms: u64) {
    let timeout_result = timeout(
        Duration::from_millis(timeout_ms),
        subscriber.wait_for_message(),
    )
    .await;
    assert!(timeout_result.is_err());
}

pub fn run_concurrent_publishers<F>(
    bus: Arc<Bus<String>>,
    topic_name: &str,
    thread_count: usize,
    publisher_fn: F,
) where
    F: Fn(usize) -> String + Send + Sync + Clone + 'static,
{
    let handles: Vec<_> = (0..thread_count)
        .map(|i| {
            let bus_clone = bus.clone();
            let topic_name = topic_name.to_string();
            let publisher_fn = publisher_fn.clone();
            thread::spawn(move || {
                let topic = bus_clone.topic(&topic_name);
                topic.publish(publisher_fn(i));
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn run_concurrent_topic_creators(bus: Arc<Bus<String>>, count: usize) -> Vec<String> {
    let handles: Vec<_> = (0..count)
        .map(|i| {
            let bus_clone = bus.clone();
            thread::spawn(move || {
                let topic = bus_clone.topic(&format!("topic{i}"));
                topic.name().to_string()
            })
        })
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

pub fn assert_topic_counts(bus: &Bus<String>, expected_count: usize, expected_names: &[&str]) {
    assert_eq!(bus.topic_count(), expected_count);
    let names = bus.topic_names();
    assert_eq!(names.len(), expected_count);
    for expected_name in expected_names {
        assert!(names.contains(&expected_name.to_string()));
    }
}
