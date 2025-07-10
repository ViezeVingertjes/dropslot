use dropslot::Bus;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Event {
    id: u64,
    event_type: String,
    payload: String,
    timestamp: u64,
}

#[tokio::main]
async fn main() {
    event_processing_example().await;
    error_handling_example().await;
    multiple_subscribers_example().await;
}

async fn event_processing_example() {
    println!("=== Event Processing System ===");

    let bus = Bus::<Event>::new();
    let events_topic = bus.topic("events");

    let mut processor = events_topic.subscribe();

    let processor_handle = tokio::spawn(async move {
        println!("Event processor started, waiting for events...");
        let mut count = 0;
        loop {
            match timeout(Duration::from_millis(100), processor.wait_for_message()).await {
                Ok(Some(event)) => {
                    println!("Processing event: {} - {}", event.id, event.event_type);
                    count += 1;
                    if count >= 5 {
                        break;
                    }
                }
                Ok(None) => break,
                Err(_) => {
                    if count > 0 {
                        break;
                    }
                }
            }
        }
        count
    });

    tokio::task::yield_now().await;

    for i in 1..=5 {
        let event = Event {
            id: i,
            event_type: "user_action".to_string(),
            payload: format!("User performed action {i}"),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        events_topic.publish(event);
        tokio::task::yield_now().await;
    }

    let processed_count = processor_handle.await.unwrap();
    println!("Processed {processed_count} events");
}

async fn error_handling_example() {
    println!("\n=== Error Handling ===");

    let bus = Bus::<String>::new();

    bus.publish("nonexistent_topic", "message".to_string());
    println!("Message published");

    let mut subscriber = bus.subscribe("nonexistent_topic");

            match subscriber.try_get_message() {
        Ok(Some(msg)) => println!("Received: {msg}"),
        Ok(None) => println!("No new message"),
        Err(e) if e.is_empty() => println!("No message available"),
        Err(e) if e.is_disconnected() => println!("Topic disconnected"),
        Err(e) => println!("Error: {e}"),
    }
}

async fn multiple_subscribers_example() {
    println!("\n=== Multiple Subscribers ===");

    let bus = Bus::<String>::new();
    let news_topic = bus.topic("news");

    let mut email_notifier = news_topic.subscribe();
    let mut push_notifier = news_topic.subscribe();
    let mut analytics = news_topic.subscribe();

    let email_handle = tokio::spawn(async move {
        let mut count = 0;
        loop {
            match timeout(Duration::from_millis(100), email_notifier.wait_for_message()).await {
                Ok(Some(news)) => {
                    println!("📧 Email notification: {news}");
                    count += 1;
                    if count >= 3 {
                        break;
                    }
                }
                Ok(None) => break,
                Err(_) => {
                    if count > 0 {
                        break;
                    }
                }
            }
        }
    });

    let push_handle = tokio::spawn(async move {
        let mut count = 0;
        loop {
            match timeout(Duration::from_millis(100), push_notifier.wait_for_message()).await {
                Ok(Some(news)) => {
                    println!("📱 Push notification: {news}");
                    count += 1;
                    if count >= 3 {
                        break;
                    }
                }
                Ok(None) => break,
                Err(_) => {
                    if count > 0 {
                        break;
                    }
                }
            }
        }
    });

    let analytics_handle = tokio::spawn(async move {
        let mut count = 0;
        loop {
            match timeout(Duration::from_millis(100), analytics.wait_for_message()).await {
                Ok(Some(news)) => {
                    println!("📊 Analytics recorded: {news}");
                    count += 1;
                    if count >= 3 {
                        break;
                    }
                }
                Ok(None) => break,
                Err(_) => {
                    if count > 0 {
                        break;
                    }
                }
            }
        }
    });

    let news_items = vec![
        "Breaking: Rust 1.75 Released!",
        "New async features announced",
        "Performance improvements in latest update",
    ];

    tokio::task::yield_now().await;

    println!(
        "Topic has {} active subscribers before publishing",
        news_topic.subscriber_count()
    );

    for news in news_items {
        println!("Publishing: {news}");
        news_topic.publish(news.to_string());
        tokio::task::yield_now().await;
    }

    email_handle.await.unwrap();
    push_handle.await.unwrap();
    analytics_handle.await.unwrap();

    println!(
        "Topic has {} active subscribers after tasks completed (subscribers dropped)",
        news_topic.subscriber_count()
    );
}
