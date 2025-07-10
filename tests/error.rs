use dropslot::BusError;
use std::error::Error;

#[test]
fn test_try_get_message_empty_error() {
    let error = BusError::message_queue_empty();

    assert!(error.is_empty());
    assert!(!error.is_disconnected());
}

#[test]
fn test_try_get_message_disconnected_error() {
    let error = BusError::topic_disconnected();

    assert!(!error.is_empty());
    assert!(error.is_disconnected());
}

#[test]
fn test_error_display_empty() {
    let error = BusError::message_queue_empty();
    let display = error.to_string();

    assert!(display.contains("No message available"));
}

#[test]
fn test_error_display_disconnected() {
    let error = BusError::topic_disconnected();
    let display = error.to_string();

    assert!(display.contains("Topic disconnected"));
}

#[test]
fn test_error_debug_format() {
    let error = BusError::message_queue_empty();
    let debug = format!("{error:?}");

    assert!(debug.contains("TryRecv"));
}

#[test]
fn test_error_equality() {
    let error1 = BusError::message_queue_empty();
    let error2 = BusError::message_queue_empty();

    assert_eq!(error1, error2);
}

#[test]
fn test_error_inequality() {
    let empty_error = BusError::message_queue_empty();
    let disconnected_error = BusError::topic_disconnected();

    assert_ne!(empty_error, disconnected_error);
}

#[test]
fn test_error_clone() {
    let error = BusError::message_queue_empty();
    let cloned = error.clone();

    assert_eq!(error, cloned);
}

#[test]
fn test_error_traits() {
    let error = BusError::message_queue_empty();

    // Test that it implements std::error::Error trait
    let _: &dyn Error = &error;

    // Test that it implements Send and Sync
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<BusError>();
}

#[test]
fn test_error_is_methods_consistency() {
    let empty_error = BusError::message_queue_empty();
    let disconnected_error = BusError::topic_disconnected();

    // Empty error should only be empty, not disconnected
    assert!(empty_error.is_empty());
    assert!(!empty_error.is_disconnected());

    // Disconnected error should only be disconnected, not empty
    assert!(!disconnected_error.is_empty());
    assert!(disconnected_error.is_disconnected());
}

#[test]
fn test_error_source() {
    let error = BusError::message_queue_empty();

    // BusError doesn't have a source error
    assert!(error.source().is_none());
}

#[test]
fn test_error_mutually_exclusive_states() {
    let empty_error = BusError::message_queue_empty();
    let disconnected_error = BusError::topic_disconnected();

    // An error cannot be both empty and disconnected
    assert!(!(empty_error.is_empty() && empty_error.is_disconnected()));
    assert!(!(disconnected_error.is_empty() && disconnected_error.is_disconnected()));
}

#[test]
fn test_error_from_try_get_message_operations() {
    use dropslot::Bus;

    let bus = Bus::<String>::new();
    let topic = bus.topic("error_test");
    let mut subscriber = topic.subscribe();

    // Should get empty error when no message
    let result = subscriber.try_get_message();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.is_empty());
    assert!(!error.is_disconnected());
}

#[test]
fn test_error_from_disconnected_topic() {
    use dropslot::Bus;

    let bus = Bus::<String>::new();
    let topic = bus.topic("disconnect_test");
    let mut subscriber = topic.subscribe();

    // Remove the topic from the bus to simulate disconnection
    drop(topic); // Drop the local reference first
    let removed_count = bus.remove_topic("disconnect_test");
    assert_eq!(removed_count, Some(1));

    let result = subscriber.try_get_message();
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(!error.is_empty());
    assert!(error.is_disconnected());
}

#[test]
fn test_error_match_patterns() {
    let empty_error = BusError::message_queue_empty();
    let disconnected_error = BusError::topic_disconnected();

    // Test pattern matching
    match empty_error {
        BusError::TryRecv {
            empty: true,
            disconnected: false,
        } => {
            // Expected pattern
        }
        _ => panic!("Unexpected error pattern"),
    }

    match disconnected_error {
        BusError::TryRecv {
            empty: false,
            disconnected: true,
        } => {
            // Expected pattern
        }
        _ => panic!("Unexpected error pattern"),
    }
}

#[test]
fn test_error_display_format_consistency() {
    let empty_error = BusError::message_queue_empty();
    let disconnected_error = BusError::topic_disconnected();

    let empty_display = format!("{empty_error}");
    let disconnected_display = format!("{disconnected_error}");

    assert!(empty_display.contains("TryRecv error"));
    assert!(disconnected_display.contains("TryRecv error"));
    assert_ne!(empty_display, disconnected_display);
}

#[test]
fn test_error_creation_methods() {
    // Test that the creation methods return the correct error types
    let empty = BusError::message_queue_empty();
    let disconnected = BusError::topic_disconnected();

    let BusError::TryRecv {
        empty: e,
        disconnected: d,
    } = empty;
    assert!(e);
    assert!(!d);

    let BusError::TryRecv {
        empty: e,
        disconnected: d,
    } = disconnected;
    assert!(!e);
    assert!(d);
}

#[test]
fn test_error_partial_eq() {
    let empty1 = BusError::message_queue_empty();
    let empty2 = BusError::message_queue_empty();
    let disconnected1 = BusError::topic_disconnected();
    let disconnected2 = BusError::topic_disconnected();

    assert_eq!(empty1, empty2);
    assert_eq!(disconnected1, disconnected2);
    assert_ne!(empty1, disconnected1);
}

#[test]
fn test_error_eq_reflexive() {
    let error = BusError::message_queue_empty();
    assert_eq!(error, error);
}

#[test]
fn test_error_eq_symmetric() {
    let error1 = BusError::message_queue_empty();
    let error2 = BusError::message_queue_empty();

    assert_eq!(error1, error2);
    assert_eq!(error2, error1);
}

#[test]
fn test_error_eq_transitive() {
    let error1 = BusError::message_queue_empty();
    let error2 = BusError::message_queue_empty();
    let error3 = BusError::message_queue_empty();

    assert_eq!(error1, error2);
    assert_eq!(error2, error3);
    assert_eq!(error1, error3);
}
