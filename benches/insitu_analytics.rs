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

fn process_points(points: &[Point]) {
    let squares = hash_points(points);
    let graphs: Vec<Vec<Vec<usize>>> = squares
        .iter()
        .map(|square| make_graph(&square, points))
        .collect();
    let final_graph = fuse_graphs(&graphs, points);
    let connected_components = compute_connected_components(&final_graph);
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
