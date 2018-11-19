extern crate grouille;
extern crate itertools;
extern crate rand;
extern crate rayon;
extern crate time;
pub mod clique;
pub mod mymerge;
mod parallel_rayon;
mod sequential_algorithm;
use grouille::Point;
use itertools::*;
use rayon::ThreadPoolBuilder;
//use parallel_rayon::*;
use rand::random;
use sequential_algorithm::*;
const THRESHOLD_DISTANCE: f64 = 0.5;
const NUM_POINTS: usize = 150_000;
const RUNS_NUMBER: u32 = 1;
const NUM_THREADS: usize = 2;
fn main() {
    ThreadPoolBuilder::new()
        .num_threads(NUM_THREADS)
        .build_global()
        .expect("Global pool build failed");
    let times_per_square_sequential: Vec<(f64, f64)> = (0..RUNS_NUMBER)
        .map(|run_index| {
            let points: Vec<_> = repeat_call(|| Point::new(random(), random()))
                .take(NUM_POINTS)
                .collect();
            let squares = hash_points(
                &points,
                THRESHOLD_DISTANCE + run_index as f64 * 2.5 / 1_000.0,
            );
            let compute_time_start = time::precise_time_ns();
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
                    Graph::new(
                        &square,
                        &points,
                        THRESHOLD_DISTANCE + run_index as f64 * 2.5 / 1_000.0,
                        *hashing_offsets,
                    )
                }).collect();
            let final_graph = fuse_graphs(graphs, points.len());
            let connected_components = final_graph.compute_connected_components();
            assert!(connected_components.len() > 0);
            let compute_time_end = time::precise_time_ns();
            (
                (compute_time_end - compute_time_start) as f64,
                THRESHOLD_DISTANCE + run_index as f64 * 2.5 / 1_000.0,
            )
        }).collect();
    let times_per_square_parallel: Vec<(f64, f64)> = (0..RUNS_NUMBER)
        .map(|run_index| {
            let points: Vec<_> = repeat_call(|| Point::new(random(), random()))
                .take(NUM_POINTS)
                .collect();
            let squares = hash_points(
                &points,
                THRESHOLD_DISTANCE + run_index as f64 * 2.5 / 1_000.0,
            );
            let compute_time_start = time::precise_time_ns();
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
                    Graph::parallel_new(
                        &square,
                        &points,
                        THRESHOLD_DISTANCE + run_index as f64 * 2.5 / 1_000.0,
                        *hashing_offsets,
                    )
                }).collect();
            let final_graph = fuse_graphs(graphs, points.len());
            let connected_components = final_graph.compute_connected_components();
            assert!(connected_components.len() > 0);
            let compute_time_end = time::precise_time_ns();
            (
                (compute_time_end - compute_time_start) as f64,
                THRESHOLD_DISTANCE + run_index as f64 * 2.5 / 1_000.0,
            )
        }).collect();

    println!("{:?}", times_per_square_sequential);
    println!("{:?}", times_per_square_parallel);
}
