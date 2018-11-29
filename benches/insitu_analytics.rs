extern crate analytics;
#[macro_use]
extern crate criterion;
extern crate grouille;
extern crate itertools;
extern crate rand;
extern crate rayon;
extern crate thread_binder;
use analytics::wrapper_functions::*;
use criterion::Criterion;
use thread_binder::*;
const THRESHOLD_DISTANCE: f64 = 0.01;
const NUM_POINTS: usize = 100_000;
const NUM_THREADS: usize = 2;
fn analytics_bench(c: &mut Criterion) {
    BindableThreadPool::new(POLICY::ROUND_ROBIN_CORE)
        .num_threads(NUM_THREADS)
        .build_global()
        .expect("Bindable thread pool failed");
    c.bench_function(
        &format!("sequential analytics (size={})", NUM_POINTS),
        |b| {
            b.iter_with_setup(
                || get_random_points(NUM_POINTS),
                |testin| {
                    wrapper_sequential(&testin, THRESHOLD_DISTANCE);
                    testin
                },
            )
        },
    );
    c.bench_function(
        &format!("rayon parallel analytics (size={})", NUM_POINTS),
        |b| {
            b.iter_with_setup(
                || get_random_points(NUM_POINTS),
                |testin| {
                    wrapper_parallel(&testin, THRESHOLD_DISTANCE);
                    testin
                },
            )
        },
    );
    c.bench_function(
        &format!(
            "rayon parallel analytics with optimal fold(size={})",
            NUM_POINTS
        ),
        |b| {
            b.iter_with_setup(
                || get_random_points(NUM_POINTS),
                |testin| {
                    wrapper_parallel_opt(&testin, THRESHOLD_DISTANCE);
                    testin
                },
            )
        },
    );
    c.bench_function(
        &format!("adaptive parallel analytics (size={})", NUM_POINTS),
        |b| {
            b.iter_with_setup(
                || get_random_points(NUM_POINTS),
                |testin| {
                    wrapper_parallel_adaptive(&testin, THRESHOLD_DISTANCE);
                    testin
                },
            )
        },
    );
}
criterion_group!(benches, analytics_bench);
criterion_main!(benches);
