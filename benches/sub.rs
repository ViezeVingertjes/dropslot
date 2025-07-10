use bytes::Bytes;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use dropslot::Bus;
use std::hint::black_box;
use std::sync::Arc;
use std::thread;
use tokio::runtime::Runtime;

mod common;
use common::configure_criterion;

fn bench_subscriber_creation(c: &mut Criterion) {
    c.bench_function("subscriber_creation", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("sub_test");
            let subscriber = topic.subscribe();
            black_box(subscriber);
        })
    });

    c.bench_function("multiple_subscribers", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("multi_sub");
            let subscribers: Vec<_> = (0..10).map(|_| topic.subscribe()).collect();
            black_box(subscribers);
        })
    });
}

fn bench_try_recv_operations(c: &mut Criterion) {
    c.bench_function("try_next_empty", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("try_empty");
            let mut subscriber = topic.subscribe();

            let result = subscriber.try_next();
            let _ = black_box(result);
        })
    });

    c.bench_function("try_next_with_message", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("try_msg");
            let mut subscriber = topic.subscribe();

            topic.publish("Test Message".to_string());

            let result = subscriber.try_next();
            let _ = black_box(result);
        })
    });

    c.bench_function("try_next_with_transform", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("try_transform");
            let mut subscriber = topic.subscribe();

            topic.publish("test".to_string());

            let result = subscriber.try_next_with(|msg| msg.len());
            let _ = black_box(result);
        })
    });
}

fn bench_async_recv_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("next_with_message", |b| {
        b.iter(|| {
            rt.block_on(async {
                let bus = Bus::<String>::new();
                let topic = bus.topic("async_test");
                let mut subscriber = topic.subscribe();

                topic.publish(black_box("Hello World".to_string()));

                let message = subscriber.next().await.unwrap();
                black_box(message);
            })
        })
    });

    c.bench_function("next_with_transform", |b| {
        b.iter(|| {
            rt.block_on(async {
                let bus = Bus::<String>::new();
                let topic = bus.topic("transform_test");
                let mut subscriber = topic.subscribe();

                topic.publish("hello world".to_string());

                let length = subscriber.next_with(|msg| msg.len()).await.unwrap();
                black_box(length);
            })
        })
    });

    c.bench_function("recv_with_message", |b| {
        b.iter(|| {
            rt.block_on(async {
                let bus = Bus::<String>::new();
                let topic = bus.topic("recv_test");
                let mut subscriber = topic.subscribe();

                topic.publish("Test Message".to_string());

                let message = subscriber.recv().await.unwrap();
                black_box(message);
            })
        })
    });
}

fn bench_latest_operations(c: &mut Criterion) {
    c.bench_function("get_latest_empty", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("latest_empty");
            let subscriber = topic.subscribe();

            let latest = subscriber.get_latest();
            black_box(latest);
        })
    });

    c.bench_function("get_latest_with_message", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("latest_msg");
            let subscriber = topic.subscribe();

            topic.publish("Latest Message".to_string());

            let latest = subscriber.get_latest();
            black_box(latest);
        })
    });

    c.bench_function("get_latest_with_transform", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("latest_transform");
            let subscriber = topic.subscribe();

            topic.publish("hello world".to_string());

            let word_count = subscriber.get_latest_with(|msg| msg.split_whitespace().count());
            black_box(word_count);
        })
    });

    c.bench_function("has_latest", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("has_latest");
            let subscriber = topic.subscribe();

            topic.publish("Test".to_string());

            let has = subscriber.has_latest();
            black_box(has);
        })
    });
}

fn bench_high_frequency_receiving(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("high_frequency_next", |b| {
        b.iter(|| {
            rt.block_on(async {
                let bus = Bus::<String>::new();
                let topic = bus.topic("high_freq");
                let mut subscriber = topic.subscribe();

                for i in 0..100 {
                    topic.publish(format!("Message {i}"));
                }

                let message = subscriber.next().await.unwrap();
                black_box(message);
            })
        })
    });
}

fn bench_bytes_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("bytes_next", |b| {
        b.iter(|| {
            rt.block_on(async {
                let bus = Bus::<Bytes>::new();
                let topic = bus.topic("bytes_test");
                let mut subscriber = topic.subscribe();

                let data = vec![1, 2, 3, 4, 5];
                topic.publish_vec(black_box(data));

                let message = subscriber.next().await.unwrap();
                black_box(message);
            })
        })
    });

    c.bench_function("bytes_try_next", |b| {
        b.iter(|| {
            let bus = Bus::<Bytes>::new();
            let topic = bus.topic("bytes_try");
            let mut subscriber = topic.subscribe();

            let data = vec![1, 2, 3, 4, 5];
            topic.publish_vec(black_box(data));

            let result = subscriber.try_next();
            let _ = black_box(result);
        })
    });
}

fn bench_subscriber_disconnect(c: &mut Criterion) {
    c.bench_function("try_next_after_disconnect", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("disconnect_test");
            let mut subscriber = topic.subscribe();

            drop(topic);
            let _ = bus.remove_topic("disconnect_test");

            let result = subscriber.try_next();
            let _ = black_box(result);
        })
    });
}

fn bench_subscriber_properties(c: &mut Criterion) {
    c.bench_function("topic_name", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("name_test");
            let subscriber = topic.subscribe();

            let name = subscriber.topic_name();
            black_box(name);
        })
    });

    c.bench_function("equality_check", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("eq_test");
            let sub1 = topic.subscribe();
            let sub2 = topic.subscribe();

            let equal = sub1 == sub2;
            black_box(equal);
        })
    });
}

fn bench_version_tracking(c: &mut Criterion) {
    c.bench_function("version_tracking", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("version_test");
            let mut subscriber = topic.subscribe();

            topic.publish("First".to_string());
            let result1 = subscriber.try_next();
            let _ = black_box(result1);

            let result2 = subscriber.try_next();
            let _ = black_box(result2);

            topic.publish("Second".to_string());
            let result3 = subscriber.try_next();
            let _ = black_box(result3);
        })
    });
}

fn bench_concurrent_subscribers(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("concurrent_subscribers");

    for num_threads in [2, 4, 8, 16].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_recv", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter(|| {
                    rt.block_on(async {
                        let bus = Arc::new(Bus::<String>::new());
                        let topic = bus.topic("concurrent_recv");
                        let mut handles = Vec::new();

                        for i in 0..num_threads {
                            let topic_clone = topic.clone();
                            let handle = thread::spawn(move || {
                                let rt = Runtime::new().unwrap();
                                rt.block_on(async {
                                    let mut subscriber = topic_clone.subscribe();
                                    topic_clone.publish(format!("Message {i}"));

                                    let message = subscriber.next().await.unwrap();
                                    black_box(message);
                                })
                            });
                            handles.push(handle);
                        }

                        for handle in handles {
                            handle.join().unwrap();
                        }
                    })
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = bench_subscriber_creation,
    bench_try_recv_operations,
    bench_async_recv_operations,
    bench_latest_operations,
    bench_high_frequency_receiving,
    bench_bytes_operations,
    bench_subscriber_disconnect,
    bench_subscriber_properties,
    bench_version_tracking,
    bench_concurrent_subscribers
);

criterion_main!(benches);
