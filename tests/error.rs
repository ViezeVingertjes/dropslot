mod common;

use common::*;
use dropslot::BusError;
use std::error::Error;

#[test]
fn test_error_creation_and_properties() {
    let empty_error = BusError::message_queue_empty();
    let disconnected_error = BusError::topic_disconnected();

    assert_error_properties(&empty_error, true, false);
    assert_error_properties(&disconnected_error, false, true);

    assert_ne!(empty_error, disconnected_error);
    assert_eq!(empty_error, BusError::message_queue_empty());
    assert_eq!(disconnected_error, BusError::topic_disconnected());

    let cloned_empty = empty_error.clone();
    assert_eq!(empty_error, cloned_empty);
}

#[test]
fn test_error_display_formatting() {
    let empty_error = BusError::message_queue_empty();
    let disconnected_error = BusError::topic_disconnected();

    let empty_display = empty_error.to_string();
    let disconnected_display = disconnected_error.to_string();

    assert!(empty_display.contains("No message available"));
    assert!(disconnected_display.contains("Topic disconnected"));
    assert!(empty_display.contains("TryRecv error"));
    assert!(disconnected_display.contains("TryRecv error"));
    assert_ne!(empty_display, disconnected_display);

    let debug_str = format!("{empty_error:?}");
    assert!(debug_str.contains("TryRecv"));
}

#[test]
fn test_error_pattern_matching() {
    let empty_error = BusError::message_queue_empty();
    let disconnected_error = BusError::topic_disconnected();

    match empty_error {
        BusError::TryRecv {
            empty: true,
            disconnected: false,
        } => {}
        _ => panic!("Unexpected error pattern"),
    }

    match disconnected_error {
        BusError::TryRecv {
            empty: false,
            disconnected: true,
        } => {}
        _ => panic!("Unexpected error pattern"),
    }

    let BusError::TryRecv {
        empty: e,
        disconnected: d,
    } = BusError::message_queue_empty();
    assert!(e);
    assert!(!d);
}

#[test]
fn test_error_traits_and_behavior() {
    let error = BusError::message_queue_empty();

    let _: &dyn Error = &error;

    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<BusError>();

    assert!(error.source().is_none());

    let error2 = BusError::message_queue_empty();
    let error3 = BusError::topic_disconnected();
    assert_eq!(error, error2);
    assert_eq!(error2, error);
    assert_ne!(error, error3);
    assert_ne!(error3, error);
}

#[test]
fn test_error_from_actual_operations() {
    let bus = create_string_bus();
    let topic = bus.topic("error_test");
    let mut subscriber = topic.subscribe();

    let result = subscriber.try_get_message();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_error_properties(&error, true, false);

    drop(topic);
    let removed_count = bus.remove_topic("error_test");
    assert_eq!(removed_count, Some(1));

    let result = subscriber.try_get_message();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_error_properties(&error, false, true);
}
