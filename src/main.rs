#[macro_use]
extern crate grouille;
extern crate itertools;
extern crate rand;
extern crate rayon;
extern crate time;
//mod parallel_rayon;
mod sequential_algorithm;
use grouille::{tycat::colored_display, Point};
use itertools::*;
//use parallel_rayon::*;
use rand::random;
use sequential_algorithm::*;

fn main() {
    let start = time::precise_time_ns();
    let points: Vec<_> = repeat_call(|| Point::new(random(), random()))
        .take(200_000)
        .collect();
    let squares = hash_points(&points);
    /*colored_display(
        squares[0]
            .values()
            .map(|v| v.iter().map(|i| points[*i]).collect::<Vec<Point>>()),
    );*/
    let graphs: Vec<Vec<Vec<usize>>> = squares
        .iter()
        .map(|square| make_graph(&square, &points))
        .collect();
    //    for graph in &graphs {
    //        println!("{:?}", graph);
    //        //display_graph(&points, &graph);
    //    }
    //println!("the fused graph is");
    //display_graph(&points, &fuse_graphs(&graphs, &points));
    let final_graph = fuse_graphs(&graphs, &points);
    //println!("{:?}", final_graph);
    let connected_components = compute_connected_components(&final_graph);
    let end = time::precise_time_ns();
    println!(
        "count is {}; time taken: {}",
        connected_components.len(),
        (end - start) as f64 / 1e6
    );
    //println!("{:?}", connected_components);
}
