use bytes::Bytes;
use dropslot::Bus;
use std::time::Instant;

#[tokio::main]
async fn main() {
    high_throughput_example().await;
    zero_copy_bytes_example().await;
}

async fn high_throughput_example() {
    println!("=== High Throughput Performance ===");

    let bus = Bus::<Bytes>::high_throughput();

    let topic = bus.topic("high_freq_data");
    let mut subscriber = topic.subscribe();

    let start = Instant::now();
    for i in 0..1000 {
        topic.publish(Bytes::from(format!("message_{i}")));
    }
    let elapsed = start.elapsed();

            if let Some(message) = subscriber.wait_for_message().await {
        println!("Last message: {message:?}");
    }

    println!("Published 1000 messages in {elapsed:?}");
}

async fn zero_copy_bytes_example() {
    println!("\n=== Zero-Copy Bytes ===");

    let bus = Bus::<Bytes>::new();
    let topic = bus.topic("bytes_topic");

    let data = b"zero-copy message payload";
    topic.publish_slice(data);

    println!("Published zero-copy bytes message");
}
