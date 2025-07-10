use bytes::Bytes;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use dropslot::Bus;
use std::hint::black_box;
use std::sync::Arc;
use std::thread;

mod common;
use common::configure_criterion;

fn bench_bus_creation(c: &mut Criterion) {
    c.bench_function("bus_new", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            black_box(bus);
        })
    });

    c.bench_function("bus_with_capacity", |b| {
        b.iter(|| {
            let bus = Bus::<String>::with_capacity(black_box(64));
            black_box(bus);
        })
    });

    c.bench_function("bus_high_throughput", |b| {
        b.iter(|| {
            let bus = Bus::<String>::high_throughput();
            black_box(bus);
        })
    });

    c.bench_function("bus_low_latency", |b| {
        b.iter(|| {
            let bus = Bus::<String>::low_latency();
            black_box(bus);
        })
    });
}

fn bench_topic_operations(c: &mut Criterion) {
    c.bench_function("topic_creation", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic(black_box("new_topic"));
            black_box(topic);
        })
    });

    c.bench_function("topic_reuse", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic(black_box("existing"));
            black_box(topic);
        })
    });

    c.bench_function("topic_count", |b| {
        let bus = Bus::<String>::new();
        let _topic = bus.topic("count_test");

        b.iter(|| {
            let count = bus.topic_count();
            black_box(count);
        })
    });

    c.bench_function("topic_names", |b| {
        let bus = Bus::<String>::new();
        let _topic1 = bus.topic("topic1");
        let _topic2 = bus.topic("topic2");

        b.iter(|| {
            let names = bus.topic_names();
            black_box(names);
        })
    });
}

fn bench_bus_publish(c: &mut Criterion) {
    c.bench_function("bus_publish", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            bus.publish(
                black_box("test_topic"),
                black_box("Hello World".to_string()),
            );
        })
    });

    c.bench_function("topic_publish", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("test_topic");
            topic.publish(black_box("Hello World".to_string()));
        })
    });

    c.bench_function("bus_publish_slice", |b| {
        let data = b"Hello World";
        b.iter(|| {
            let bus = Bus::<Bytes>::new();
            bus.publish_slice(black_box("bytes_topic"), black_box(data));
        })
    });

    c.bench_function("bus_publish_vec", |b| {
        let data = vec![1, 2, 3, 4, 5];
        b.iter(|| {
            let bus = Bus::<Bytes>::new();
            bus.publish_vec(black_box("bytes_topic"), black_box(data.clone()));
        })
    });
}

fn bench_topic_cleanup(c: &mut Criterion) {
    c.bench_function("remove_topic", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("removable");
            let _subscriber = topic.subscribe();
            drop(topic);

            let result = bus.remove_topic(black_box("removable"));
            black_box(result);
        })
    });

    c.bench_function("cleanup_unused_topics", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("unused");
            let subscriber = topic.subscribe();
            drop(topic);
            drop(subscriber);

            let removed = bus.cleanup_unused_topics();
            black_box(removed);
        })
    });
}

fn bench_bus_subscribe(c: &mut Criterion) {
    c.bench_function("bus_subscribe", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let subscriber = bus.subscribe(black_box("test_topic"));
            black_box(subscriber);
        })
    });

    c.bench_function("topic_subscribe", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            let topic = bus.topic("test_topic");
            let subscriber = topic.subscribe();
            black_box(subscriber);
        })
    });
}

fn bench_concurrent_bus_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_bus_operations");

    for num_threads in [2, 4, 8, 16].iter() {
        group.bench_with_input(
            BenchmarkId::new("topic_creation", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter(|| {
                    let bus = Arc::new(Bus::<String>::new());
                    let mut handles = Vec::new();

                    for i in 0..num_threads {
                        let bus_clone = bus.clone();
                        let handle = thread::spawn(move || {
                            let topic = bus_clone.topic(&format!("thread_{i}"));
                            black_box(topic);
                        });
                        handles.push(handle);
                    }

                    for handle in handles {
                        handle.join().unwrap();
                    }
                })
            },
        );
    }

    for num_threads in [2, 4, 8, 16].iter() {
        group.bench_with_input(
            BenchmarkId::new("publish", num_threads),
            num_threads,
            |b, &num_threads| {
                b.iter(|| {
                    let bus = Arc::new(Bus::<String>::new());
                    let mut handles = Vec::new();

                    for i in 0..num_threads {
                        let bus_clone = bus.clone();
                        let handle = thread::spawn(move || {
                            bus_clone.publish(
                                &format!("thread_{i}"),
                                format!("Message from thread {i}"),
                            );
                        });
                        handles.push(handle);
                    }

                    for handle in handles {
                        handle.join().unwrap();
                    }
                })
            },
        );
    }

    group.finish();
}

fn bench_bus_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("bus_scalability");

    for num_topics in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("many_topics", num_topics),
            num_topics,
            |b, &num_topics| {
                b.iter(|| {
                    let bus = Bus::<String>::new();
                    for i in 0..num_topics {
                        let topic = bus.topic(&format!("topic_{i}"));
                        black_box(topic);
                    }
                    let count = bus.topic_count();
                    black_box(count);
                })
            },
        );
    }

    group.finish();
}

fn bench_bus_cleanup_performance(c: &mut Criterion) {
    c.bench_function("cleanup_with_many_topics", |b| {
        b.iter(|| {
            let bus = Bus::<String>::new();
            for i in 0..100 {
                let topic = bus.topic(&format!("topic_{i}"));
                black_box(topic);
            }
            let topic = bus.topic("cleanup_test");
            black_box(topic);
        })
    });
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = bench_bus_creation,
    bench_topic_operations,
    bench_bus_publish,
    bench_topic_cleanup,
    bench_bus_subscribe,
    bench_concurrent_bus_operations,
    bench_bus_scalability,
    bench_bus_cleanup_performance
);

criterion_main!(benches);
