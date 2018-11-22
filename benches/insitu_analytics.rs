extern crate analytics;
#[macro_use]
extern crate criterion;
extern crate grouille;
extern crate itertools;
extern crate rand;
extern crate rayon;
use analytics::sequential_algorithm::*;
use criterion::Criterion;
use grouille::Point;
use itertools::repeat_call;
use rand::random;
use rayon::prelude::*;
const NUM_POINTS: usize = 150_000;
const THRESHOLD_DISTANCE: f64 = 0.01;

fn wrapper_sequential(points: &[Point]) {
    let squares = hash_points(points, THRESHOLD_DISTANCE);
    let graphs: Vec<Graph> = squares
        .iter()
        .zip(
            [
                (0.0, 0.0),
                (THRESHOLD_DISTANCE, 0.0),
                (0.0, THRESHOLD_DISTANCE),
                (THRESHOLD_DISTANCE, THRESHOLD_DISTANCE),
            ]
                .into_iter(),
        ) // TODO: fixme
        .map(|(square, hashing_offsets)| {
            Graph::new(&square, points, THRESHOLD_DISTANCE, *hashing_offsets)
        }).collect();
    let final_graph = fuse_graphs(graphs, points.len());
    let connected_components = final_graph.compute_connected_components();
    assert!(connected_components.len() > 0);
}

fn wrapper_parallel(points: &[Point]) {
    let squares = hash_points(points, THRESHOLD_DISTANCE);
    let hashing_offsets = vec![
        (0.0, 0.0),
        (THRESHOLD_DISTANCE, 0.0),
        (0.0, THRESHOLD_DISTANCE),
        (THRESHOLD_DISTANCE, THRESHOLD_DISTANCE),
    ];

    let graphs: Vec<Graph> = squares
        .par_iter()
        .zip(hashing_offsets.par_iter())
        .map(|(square, hashing_offset)| {
            Graph::parallel_new(&square, points, THRESHOLD_DISTANCE, *hashing_offset)
        }).collect();
    let final_graph = fuse_graphs(graphs, points.len());
    let connected_components = final_graph.compute_connected_components();
    assert!(connected_components.len() > 0);
}

fn get_random_points() -> Vec<Point> {
    repeat_call(|| Point::new(random(), random()))
        .take(NUM_POINTS)
        .collect()
}

fn analytics_bench(c: &mut Criterion) {
    c.bench_function(
        &format!("sequential analytics (size={})", NUM_POINTS),
        |b| {
            b.iter_with_setup(
                || get_random_points(),
                |testin| {
                    wrapper_sequential(&testin);
                    testin
                },
            )
        },
    );
    c.bench_function(
        &format!("rayon parallel analytics (size={})", NUM_POINTS),
        |b| {
            b.iter_with_setup(
                || get_random_points(),
                |testin| {
                    wrapper_parallel(&testin);
                    testin
                },
            )
        },
    );
}
criterion_group!(benches, analytics_bench);
criterion_main!(benches);
