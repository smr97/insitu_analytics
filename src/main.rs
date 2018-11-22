extern crate grouille;
extern crate itertools;
extern crate rand;
extern crate rayon;
extern crate rayon_logs;
extern crate time;
pub mod clique;
pub mod mymerge;
mod parallel_rayon;
mod sequential_algorithm;
use grouille::Point;
use itertools::*;
//use rayon::prelude::*;
use rayon_logs::prelude::*;
use rayon_logs::Logged;
use rayon_logs::ThreadPoolBuilder;
//use parallel_rayon::*;
use rand::random;
use sequential_algorithm::*;
const THRESHOLD_DISTANCE: f64 = 0.5;
const NUM_POINTS: usize = 150_000;
const NUM_THREADS: usize = 2;
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
    use rayon::prelude::IndexedParallelIterator;
    let hashing_offsets = vec![
        (0.0, 0.0),
        (THRESHOLD_DISTANCE, 0.0),
        (0.0, THRESHOLD_DISTANCE),
        (THRESHOLD_DISTANCE, THRESHOLD_DISTANCE),
    ];

    let graphs: Vec<Graph> = Logged::new(
        rayon::prelude::IntoParallelRefIterator::par_iter(&squares).zip(
            rayon::prelude::IntoParallelRefIterator::par_iter(&hashing_offsets),
        ),
    ) // TODO: fixme
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

fn main() {
    let pool = ThreadPoolBuilder::new()
        .num_threads(NUM_THREADS)
        //.bind_threads()
        .build()
        .expect("Pool creation failed");
    let input = get_random_points();
    pool.install(|| wrapper_parallel(&input))
        .1
        .save_svg("analytics_visualisation.html")
        .expect("Failed");
    //    pool.compare()
    //        .runs_number(5)
    //        .attach_algorithm_with_setup(
    //            "sequential",
    //            || get_random_points(),
    //            |vec| {
    //                wrapper_sequential(&vec);
    //                vec
    //            },
    //        ).attach_algorithm_with_setup(
    //            "parallel",
    //            || get_random_points(),
    //            |vec| {
    //                wrapper_parallel(&vec);
    //                vec
    //            },
    //        ).generate_logs(format!(
    //            "comparisons_{}K_{}threads.html",
    //            NUM_POINTS as u32 / (1e3 as u32),
    //            NUM_THREADS
    //        )).expect("comparison failed");
}
