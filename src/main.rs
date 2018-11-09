#[macro_use]
extern crate grouille;
extern crate itertools;
extern crate rand;
extern crate rayon;
extern crate time;
pub mod clique;
mod sequential_algorithm;
use grouille::{tycat::colored_display, Point};
use itertools::*;
//use parallel_rayon::*;
use rand::random;
use sequential_algorithm::*;
const THRESHOLD_DISTANCE: f64 = 0.42;
const NUM_POINTS: usize = 200_000;
const RUNS_NUMBER: u32 = 100;
fn main() {
    let times_per_square: Vec<(f64, f64)> = (0..RUNS_NUMBER)
        .map(|run_index| {
            let points: Vec<_> = repeat_call(|| Point::new(random(), random()))
                .take(NUM_POINTS)
                .collect();
            let squares = hash_points(&points, THRESHOLD_DISTANCE + run_index as f64 / 100_000.0);
            /*colored_display(
        squares[0]
            .values()
            .map(|v| v.iter().map(|i| points[*i]).collect::<Vec<Point>>()),
    );*/
            let compute_time_start = time::precise_time_ns();
            let graphs: Vec<Graph> = squares
                .iter()
                .map(|square| {
                    Graph::new(
                        &square,
                        &points,
                        THRESHOLD_DISTANCE + run_index as f64 / 100_000.0,
                    )
                }).collect();
            let number_of_squares = squares[0].keys().len();
            //    for graph in &graphs {
            //        println!("{:?}", graph);
            //        //display_graph(&points, &graph);
            //    }
            //println!("the fused graph is");
            //display_graph(&points, &fuse_graphs(&graphs, &points));
            let final_graph = fuse_graphs(graphs, points.len());
            //println!("{:?}", final_graph);
            let connected_components = final_graph.compute_connected_components();
            //println!("{}", connected_components.len());
            assert!(connected_components.len() > 1);
            let compute_time_end = time::precise_time_ns();
            (
                (compute_time_end - compute_time_start) as f64,
                THRESHOLD_DISTANCE + run_index as f64 / 100_000.0,
            )
        }).collect();
    println!("{:?}", times_per_square);
}
