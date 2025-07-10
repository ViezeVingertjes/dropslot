use criterion::Criterion;
use std::time::Duration;

/// Configure Criterion for faster execution in CI environments
pub fn configure_criterion() -> Criterion {
    let mut criterion = Criterion::default();

    // Check if we're in a CI environment
    let is_ci = std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || std::env::var("CARGO_BENCH_FAST").is_ok();

    if is_ci {
        // Fast configuration for CI
        criterion = criterion
            .measurement_time(Duration::from_secs(3)) // Reduced from 5s
            .warm_up_time(Duration::from_secs(1)) // Reduced from 3s
            .sample_size(20) // Reduced from 100
            .significance_level(0.1) // Relaxed from 0.05
            .noise_threshold(0.05); // Relaxed from 0.01
    } else {
        // Balanced configuration for local development
        criterion = criterion
            .measurement_time(Duration::from_secs(3)) // Slightly reduced
            .warm_up_time(Duration::from_secs(2)) // Slightly reduced
            .sample_size(50) // Balanced sample size
            .significance_level(0.05) // Standard significance
            .noise_threshold(0.02); // Balanced noise threshold
    }

    criterion
}
