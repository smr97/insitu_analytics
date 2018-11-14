extern crate analytics;
#[macro_use]
extern crate criterion;
extern crate grouille;
extern crate itertools;
extern crate rand;
use analytics::sequential_algorithm::*;
use criterion::Criterion;
use grouille::Point;
use itertools::repeat_call;
use rand::random;
const SIZE: usize = 200_000;
const THRESHOLD_DISTANCE: f64 = 0.0032;
fn process_points(points: &[Point]) {
    let squares = hash_points(points, THRESHOLD_DISTANCE);
    let hashing_offsets = [
        (0.0, 0.0),
        (THRESHOLD_DISTANCE, 0.0),
        (0.0, THRESHOLD_DISTANCE),
        (THRESHOLD_DISTANCE, THRESHOLD_DISTANCE),
    ];
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
            Graph::new(&square, &points, THRESHOLD_DISTANCE, *hashing_offsets)
        }).collect();
    let final_graph = fuse_graphs(graphs, points.len());
    let connected_components = final_graph.compute_connected_components();
    assert!(connected_components.len() > 0);
}

fn analytics_bench(c: &mut Criterion) {
    c.bench_function(&format!("adaptive infix (size={})", SIZE), |b| {
        b.iter_with_setup(
            || {
                repeat_call(|| Point::new(random(), random()))
                    .take(SIZE)
                    .collect::<Vec<Point>>()
            },
            |testin| {
                process_points(&testin);
                testin
            },
        )
    });
}
criterion_group!(benches, analytics_bench);
criterion_main!(benches);
