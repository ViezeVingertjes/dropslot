use bytes::Bytes;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use dropslot::Bus;
use std::hint::black_box;
use std::sync::Arc;
use std::thread;
use tokio::runtime::Runtime;

mod common;
use common::configure_criterion;

fn bench_basic_pubsub(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();

    c.bench_function("basic_pubsub", |b| {
        b.iter(|| {
            _rt.block_on(async {
                let bus = Bus::<String>::new();
                let topic = bus.topic("test");
                let mut subscriber = topic.subscribe();

                topic.publish(black_box("Hello".to_string()));

                let message = subscriber.wait_for_message().await.unwrap();
                black_box(message);
            })
        })
    });
}

fn bench_latest_only_semantics(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();

    c.bench_function("latest_only_semantics", |b| {
        b.iter(|| {
            _rt.block_on(async {
                let bus = Bus::<String>::new();
                let topic = bus.topic("test");
                let mut subscriber = topic.subscribe();

                for i in 0..10 {
                    topic.publish(format!("Message {i}"));
                }

                let message = subscriber.wait_for_message().await.unwrap();
                black_box(message);
            })
        })
    });
}

fn bench_multiple_subscribers(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();

    c.bench_function("multiple_subscribers", |b| {
        b.iter(|| {
            _rt.block_on(async {
                let bus = Bus::<String>::new();
                let topic = bus.topic("test");

                let mut subs: Vec<_> = (0..10).map(|_| topic.subscribe()).collect();

                topic.publish("Broadcast Message".to_string());

                for sub in &mut subs {
                    let message = sub.wait_for_message().await.unwrap();
                    black_box(message);
                }
            })
        })
    });
}

fn bench_high_frequency_publishing(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();

    c.bench_function("high_frequency_publishing", |b| {
        b.iter(|| {
            _rt.block_on(async {
                let bus = Bus::<String>::new();
                let topic = bus.topic("test");
                let mut subscriber = topic.subscribe();

                for i in 0..100 {
                    topic.publish(format!("Message {i}"));
                }

                let message = subscriber.wait_for_message().await.unwrap();
                black_box(message);
            })
        })
    });
}

fn bench_bytes_operations(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();

    c.bench_function("bytes_operations", |b| {
        b.iter(|| {
            _rt.block_on(async {
                let bus = Bus::<Bytes>::new();
                let topic = bus.topic("bytes_test");
                let mut subscriber = topic.subscribe();

                let data = vec![1, 2, 3, 4, 5];
                topic.publish_vec(black_box(data));

                let message = subscriber.wait_for_message().await.unwrap();
                black_box(message);
            })
        })
    });
}

fn bench_concurrent_access(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("concurrent_access");
    for num_threads in [2, 4, 8, 16].iter() {
        group.bench_with_input(
            BenchmarkId::new("threads", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter(|| {
                    _rt.block_on(async {
                        let bus = Arc::new(Bus::<String>::new());
                        let mut handles = Vec::new();

                        for i in 0..num_threads {
                            let bus_clone = bus.clone();
                            let handle = thread::spawn(move || {
                                let topic = bus_clone.topic(&format!("thread_{i}"));
                                let mut subscriber = topic.subscribe();

                                topic.publish(format!("Message from thread {i}"));

                                // Use tokio's block_on in thread
                                let rt = Runtime::new().unwrap();
                                rt.block_on(async {
                                    let message = subscriber.wait_for_message().await.unwrap();
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

fn bench_topic_creation_and_reuse(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();

    c.bench_function("topic_creation", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic(black_box("test_topic"));
            black_box(topic);
        })
    });

    c.bench_function("topic_reuse", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic(black_box("reuse_test"));
            black_box(topic);
        })
    });
}

fn bench_subscriber_operations(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();

    c.bench_function("subscriber_creation", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("test_topic");
            let subscriber = topic.subscribe();
            black_box(subscriber);
        })
    });

    c.bench_function("try_get_message_empty", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("test_topic");
            let mut subscriber = topic.subscribe();

            let result = subscriber.try_get_message();
            let _ = black_box(result);
        })
    });

    c.bench_function("try_get_message_with_message", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("test_topic");
            let mut subscriber = topic.subscribe();

            topic.publish("Test Message".to_string());

            let result = subscriber.try_get_message();
            let _ = black_box(result);
        })
    });
}

fn bench_performance_configurations(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();

    c.bench_function("high_throughput_config", |b| {
        b.iter(|| {
            _rt.block_on(async {
                let bus = Bus::<String>::with_capacity(64);
                let topic = bus.topic("test");
                let mut subscriber = topic.subscribe();

                topic.publish(black_box("Test".to_string()));

                let message = subscriber.wait_for_message().await.unwrap();
                black_box(message);
            })
        })
    });

    c.bench_function("low_latency_config", |b| {
        b.iter(|| {
            _rt.block_on(async {
                let bus = Bus::<String>::with_capacity(8);
                let topic = bus.topic("test");
                let mut subscriber = topic.subscribe();

                topic.publish(black_box("Test".to_string()));

                let message = subscriber.wait_for_message().await.unwrap();
                black_box(message);
            })
        })
    });

    c.bench_function("custom_capacity_config", |b| {
        b.iter(|| {
            _rt.block_on(async {
                let bus = Bus::<String>::with_capacity(256);
                let topic = bus.topic("test");
                let mut subscriber = topic.subscribe();

                topic.publish(black_box("Test".to_string()));

                let message = subscriber.wait_for_message().await.unwrap();
                black_box(message);
            })
        })
    });
}

fn bench_memory_usage(c: &mut Criterion) {
    let _rt = Runtime::new().unwrap();

    c.bench_function("topic_name_generation", |b| {
        let topic_names: Vec<String> = (0..1000).map(|i| format!("topic_{i}")).collect();

        b.iter(|| {
            let bus = Bus::<String>::new();
            for name in &topic_names {
                let topic = bus.topic(name);
                black_box(topic);
            }
        })
    });

    c.bench_function("many_topics", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            for i in 0..100 {
                let topic = bus.topic(&format!("topic_{i}"));
                black_box(topic);
            }
            let count = bus.topic_count();
            black_box(count);
        })
    });

    c.bench_function("memory_cleanup", |b| {
        b.iter(|| {
            _rt.block_on(async {
                let bus = Bus::<String>::new();
                let topic = bus.topic("cleanup_test");

                // Create and drop many subscribers
                for _ in 0..100 {
                    let subscriber = topic.subscribe();
                    drop(subscriber);
                }

                let removed = bus.cleanup_unused_topics();
                black_box(removed);
            })
        })
    });
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = bench_basic_pubsub,
    bench_latest_only_semantics,
    bench_multiple_subscribers,
    bench_high_frequency_publishing,
    bench_bytes_operations,
    bench_concurrent_access,
    bench_topic_creation_and_reuse,
    bench_subscriber_operations,
    bench_performance_configurations,
    bench_memory_usage
);

criterion_main!(benches);
