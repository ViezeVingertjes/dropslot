use criterion::{Criterion, criterion_group, criterion_main};
use dropslot::error::BusError;
use std::hint::black_box;

mod common;
use common::configure_criterion;

fn bench_error_creation(c: &mut Criterion) {
    c.bench_function("error_empty", |b| {
        b.iter(|| {
            let error = BusError::message_queue_empty();
            black_box(error);
        })
    });

    c.bench_function("error_disconnected", |b| {
        b.iter(|| {
            let error = BusError::topic_disconnected();
            black_box(error);
        })
    });
}

fn bench_error_checking(c: &mut Criterion) {
    c.bench_function("is_empty_check", |b| {
        let error = BusError::message_queue_empty();

        b.iter(|| {
            let is_empty = error.is_empty();
            black_box(is_empty);
        })
    });

    c.bench_function("is_disconnected_check", |b| {
        let error = BusError::topic_disconnected();

        b.iter(|| {
            let is_disconnected = error.is_disconnected();
            black_box(is_disconnected);
        })
    });

    c.bench_function("is_empty_on_disconnected", |b| {
        let error = BusError::topic_disconnected();

        b.iter(|| {
            let is_empty = error.is_empty();
            black_box(is_empty);
        })
    });

    c.bench_function("is_disconnected_on_empty", |b| {
        let error = BusError::message_queue_empty();

        b.iter(|| {
            let is_disconnected = error.is_disconnected();
            black_box(is_disconnected);
        })
    });
}

fn bench_error_display(c: &mut Criterion) {
    c.bench_function("display_empty", |b| {
        let error = BusError::message_queue_empty();

        b.iter(|| {
            let display = error.to_string();
            black_box(display);
        })
    });

    c.bench_function("display_disconnected", |b| {
        let error = BusError::topic_disconnected();

        b.iter(|| {
            let display = error.to_string();
            black_box(display);
        })
    });

    c.bench_function("format_empty", |b| {
        let error = BusError::message_queue_empty();

        b.iter(|| {
            let formatted = format!("{error}");
            black_box(formatted);
        })
    });

    c.bench_function("debug_empty", |b| {
        let error = BusError::message_queue_empty();

        b.iter(|| {
            let debug = format!("{error:?}");
            black_box(debug);
        })
    });
}

fn bench_error_clone(c: &mut Criterion) {
    c.bench_function("clone_empty", |b| {
        let error = BusError::message_queue_empty();

        b.iter(|| {
            let cloned = error.clone();
            black_box(cloned);
        })
    });

    c.bench_function("clone_disconnected", |b| {
        let error = BusError::topic_disconnected();

        b.iter(|| {
            let cloned = error.clone();
            black_box(cloned);
        })
    });
}

fn bench_error_equality(c: &mut Criterion) {
    c.bench_function("equality_same", |b| {
        let error1 = BusError::message_queue_empty();
        let error2 = BusError::message_queue_empty();

        b.iter(|| {
            let equal = error1 == error2;
            black_box(equal);
        })
    });

    c.bench_function("equality_different", |b| {
        let error1 = BusError::message_queue_empty();
        let error2 = BusError::topic_disconnected();

        b.iter(|| {
            let equal = error1 == error2;
            black_box(equal);
        })
    });

    c.bench_function("inequality", |b| {
        let error1 = BusError::message_queue_empty();
        let error2 = BusError::topic_disconnected();

        b.iter(|| {
            let not_equal = error1 != error2;
            black_box(not_equal);
        })
    });
}

fn bench_error_conversion(c: &mut Criterion) {
    c.bench_function("from_empty", |b| {
        b.iter(|| {
            let error: BusError = BusError::message_queue_empty();
            black_box(error);
        })
    });

    c.bench_function("from_disconnected", |b| {
        b.iter(|| {
            let error: BusError = BusError::topic_disconnected();
            black_box(error);
        })
    });
}

fn bench_error_std_error(c: &mut Criterion) {
    c.bench_function("std_error_display", |b| {
        let error = BusError::message_queue_empty();

        b.iter(|| {
            let display = error.to_string();
            black_box(display);
        })
    });

    c.bench_function("std_error_source", |b| {
        let error = BusError::message_queue_empty();

        b.iter(|| {
            let source = std::error::Error::source(&error);
            black_box(source);
        })
    });
}

fn bench_error_patterns(c: &mut Criterion) {
    c.bench_function("pattern_matching", |b| {
        let errors = vec![
            BusError::message_queue_empty(),
            BusError::topic_disconnected(),
            BusError::message_queue_empty(),
        ];

        b.iter(|| {
            for error in &errors {
                match error {
                    BusError::TryRecv { empty: true, .. } => {
                        black_box("empty");
                    }
                    BusError::TryRecv {
                        disconnected: true, ..
                    } => {
                        black_box("disconnected");
                    }
                    _ => {
                        black_box("other");
                    }
                }
            }
        })
    });
}

fn bench_error_categorization(c: &mut Criterion) {
    c.bench_function("categorize_errors", |b| {
        let errors = vec![
            BusError::message_queue_empty(),
            BusError::topic_disconnected(),
            BusError::message_queue_empty(),
            BusError::topic_disconnected(),
        ];

        b.iter(|| {
            let mut empty_count = 0;
            let mut disconnected_count = 0;

            for error in &errors {
                if error.is_empty() {
                    empty_count += 1;
                } else if error.is_disconnected() {
                    disconnected_count += 1;
                }
            }

            black_box((empty_count, disconnected_count));
        })
    });
}

fn bench_error_result_patterns(c: &mut Criterion) {
    c.bench_function("unwrap_success", |b| {
        b.iter(|| {
            let value = "success".to_string();
            black_box(value);
        })
    });

    c.bench_function("unwrap_err_empty", |b| {
        b.iter(|| {
            let error = BusError::message_queue_empty();
            black_box(error);
        })
    });

    c.bench_function("unwrap_err_disconnected", |b| {
        b.iter(|| {
            let error = BusError::topic_disconnected();
            black_box(error);
        })
    });
}

fn bench_error_handling_patterns(c: &mut Criterion) {
    c.bench_function("error_handling", |b| {
        b.iter(|| {
            let result: Result<String, BusError> = Err(BusError::message_queue_empty());

            let handled = match result {
                Ok(value) => format!("Got: {value}"),
                Err(e) if e.is_empty() => "Empty".to_string(),
                Err(e) if e.is_disconnected() => "Disconnected".to_string(),
                Err(_) => "Other".to_string(),
            };

            black_box(handled);
        })
    });
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = bench_error_creation,
    bench_error_checking,
    bench_error_display,
    bench_error_clone,
    bench_error_equality,
    bench_error_conversion,
    bench_error_std_error,
    bench_error_patterns,
    bench_error_categorization,
    bench_error_result_patterns,
    bench_error_handling_patterns
);

criterion_main!(benches);
