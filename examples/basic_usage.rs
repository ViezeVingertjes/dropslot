use bytes::Bytes;
use dropslot::prelude::*;

#[tokio::main]
async fn main() {
    let bus = Bus::<Bytes>::new();

    let events_topic = bus.topic("events");
    let mut subscriber = events_topic.subscribe();

    println!("Publishing messages...");
    events_topic.publish(Bytes::from("Hello"));
    events_topic.publish(Bytes::from("World"));

    if let Some(message) = subscriber.wait_for_message().await {
        println!("Received: {message:?}");
    }

    println!("Topic has {} subscribers", events_topic.subscriber_count());
}
