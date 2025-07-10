use bytes::Bytes;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use dropslot::Bus;
use std::hint::black_box;
use std::sync::Arc;
use tokio::runtime::Runtime;

mod common;
use common::configure_criterion;

fn bench_topic_publish(c: &mut Criterion) {
    c.bench_function("topic_publish_string", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("publish_test");

            topic.publish(black_box("Hello World".to_string()));
        })
    });

    c.bench_function("topic_publish_bytes", |b| {
        b.iter(|| {
            let bus = Bus::<Bytes>::new();
            let topic = bus.topic("bytes_test");

            let data = vec![1, 2, 3, 4, 5];
            topic.publish_vec(black_box(data));
        })
    });

    c.bench_function("topic_publish_slice", |b| {
        let data = b"Hello World";
        b.iter(|| {
            let bus = Bus::<Bytes>::new();
            let topic = bus.topic("slice_test");

            topic.publish_slice(black_box(data));
        })
    });
}

fn bench_topic_subscribe(c: &mut Criterion) {
    c.bench_function("topic_subscribe", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("subscribe_test");

            let subscriber = topic.subscribe();
            black_box(subscriber);
        })
    });

    c.bench_function("topic_multiple_subscribe", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("multi_test");

            let subs: Vec<_> = (0..10).map(|_| topic.subscribe()).collect();
            black_box(subs);
        })
    });
}

fn bench_topic_metadata(c: &mut Criterion) {
    c.bench_function("topic_name", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("name_test");

            let name = topic.name();
            black_box(name);
        })
    });

    c.bench_function("topic_subscriber_count", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("count_test");
            let _sub = topic.subscribe();

            let count = topic.subscriber_count();
            black_box(count);
        })
    });

    c.bench_function("topic_has_subscribers", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("has_test");
            let _sub = topic.subscribe();

            let has = topic.has_subscribers();
            black_box(has);
        })
    });
}

fn bench_topic_versioning(c: &mut Criterion) {
    c.bench_function("topic_version_increment", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("version_test");

            for i in 0..10 {
                topic.publish(format!("Message {i}"));
            }
        })
    });
}

fn bench_topic_high_frequency(c: &mut Criterion) {
    let mut group = c.benchmark_group("topic_high_frequency");

    group.bench_function("high_frequency_publish", |b| {
        let bus = Bus::<String>::new();
        let topic = bus.topic("high_freq_test");

        b.iter(|| {
            for i in 0..1000 {
                topic.publish(format!("Message {i}"));
            }
        })
    });

    group.bench_function("high_throughput_bytes", |b| {
        let bus = Bus::<bytes::Bytes>::new();
        let topic = bus.topic("throughput_test");
        let data = vec![42u8; 1024]; // 1KB messages

        b.iter(|| {
            for _ in 0..1000 {
                topic.publish_vec(data.clone());
            }
        })
    });

    group.bench_function("zero_copy_publish", |b| {
        let bus = Bus::<bytes::Bytes>::new();
        let topic = bus.topic("zero_copy_test");
        let data = b"Hello World! This is a test message.";

        b.iter(|| {
            for _ in 0..10000 {
                topic.publish_slice(data);
            }
        })
    });

    group.finish();
}

fn bench_topic_concurrent_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("topic_concurrent_access");

    // Test with different numbers of threads
    for num_threads in [2, 4, 8, 16].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_publish", num_threads),
            num_threads,
            |b, &num_threads| {
                // Create bus and topic OUTSIDE the benchmark iteration
                let bus = Arc::new(Bus::<String>::new());
                let topic = bus.topic("concurrent_test");
                let messages_per_thread = 100;

                b.iter(|| {
                    // Use crossbeam scoped threads to avoid allocation overhead
                    std::thread::scope(|s| {
                        for i in 0..num_threads {
                            let topic_clone = topic.clone();
                            s.spawn(move || {
                                // Each thread publishes multiple messages
                                for j in 0..messages_per_thread {
                                    topic_clone.publish(format!("Thread {i} Message {j}"));
                                }
                            });
                        }
                        // Threads automatically joined when scope ends
                    });
                })
            },
        );
    }

    // Add a subscriber-heavy benchmark
    group.bench_function("concurrent_subscribe", |b| {
        let bus = Arc::new(Bus::<String>::new());
        let topic = bus.topic("subscribe_test");

        b.iter(|| {
            std::thread::scope(|s| {
                for i in 0..8 {
                    let topic_clone = topic.clone();
                    s.spawn(move || {
                        let _subscriber = topic_clone.subscribe();
                        // Simulate some work
                        std::hint::black_box(format!("Subscriber {i}"));
                    });
                }
            });
        });
    });

    // Add a mixed read/write benchmark
    group.bench_function("concurrent_mixed_operations", |b| {
        let bus = Arc::new(Bus::<String>::new());
        let topic = bus.topic("mixed_test");

        b.iter(|| {
            std::thread::scope(|s| {
                // Publisher threads
                for i in 0..4 {
                    let topic_clone = topic.clone();
                    s.spawn(move || {
                        for j in 0..50 {
                            topic_clone.publish(format!("Pub {i} Msg {j}"));
                        }
                    });
                }

                // Subscriber threads
                for _i in 0..4 {
                    let topic_clone = topic.clone();
                    s.spawn(move || {
                        let _subscriber = topic_clone.subscribe();
                        // Check subscriber count
                        std::hint::black_box(topic_clone.subscriber_count());
                    });
                }
            });
        });
    });

    group.finish();
}

fn bench_topic_mixed_operations(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("topic_mixed_operations", |b| {
        b.iter(|| {
            rt.block_on(async {
                let bus = Bus::<String>::new();
                let topic = bus.topic("mixed_test");

                for i in 0..10 {
                    topic.publish(format!("Message {i}"));
                }
            })
        })
    });
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = bench_topic_publish,
    bench_topic_subscribe,
    bench_topic_metadata,
    bench_topic_versioning,
    bench_topic_high_frequency,
    bench_topic_concurrent_access,
    bench_topic_mixed_operations
);

criterion_main!(benches);
