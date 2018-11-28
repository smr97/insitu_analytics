extern crate grouille;
extern crate itertools;
extern crate rand;
extern crate rayon;
extern crate rayon_logs;
extern crate time;
use self::rayon_logs::prelude::*;
use self::rayon_logs::Logged;
use grouille::Point;
use itertools::*;
use rand::random;
use sequential_algorithm::*;
pub fn wrapper_sequential(points: &[Point], threshold_distance: f64) {
    let squares = hash_points(points, threshold_distance);
    let graphs: Vec<Graph> = squares
        .iter()
        .zip(
            [
                (0.0, 0.0),
                (threshold_distance, 0.0),
                (0.0, threshold_distance),
                (threshold_distance, threshold_distance),
            ]
                .into_iter(),
        ) // TODO: fixme
        .map(|(square, hashing_offsets)| {
            Graph::new(&square, points, threshold_distance, *hashing_offsets)
        }).collect();
    let final_graph = fuse_graphs(graphs, points.len());
    let connected_components = final_graph.compute_connected_components();
    assert!(connected_components.len() > 0);
}

pub fn wrapper_parallel(points: &[Point], threshold_distance: f64) {
    let squares = hash_points(points, threshold_distance);
    use rayon::prelude::IndexedParallelIterator;
    let hashing_offsets = vec![
        (0.0, 0.0),
        (threshold_distance, 0.0),
        (0.0, threshold_distance),
        (threshold_distance, threshold_distance),
    ];
    //let graphs = Vec::with_capacity(0);
    //#[cfg(features = "logs")]
    //{
    let graphs: Vec<Graph> = Logged::new(
        rayon::prelude::IntoParallelRefIterator::par_iter(&squares).zip(
            rayon::prelude::IntoParallelRefIterator::par_iter(&hashing_offsets),
        ),
    ) // TODO: fixme
    .map(|(square, hashing_offset)| {
        Graph::parallel_new(&square, points, threshold_distance, *hashing_offset)
    }).collect();
    let final_graph = fuse_graphs(graphs, points.len());
    let connected_components = final_graph.compute_connected_components();
    assert!(connected_components.len() > 0);
    //}
    //#[cfg(not(features = "logs"))]
    //let graphs: Vec<Graph> = squares
    //    .par_iter()
    //    .zip(hashing_offsets.par_iter())
    //    .map(|(square, hashing_offset)| {
    //        Graph::parallel_new(&square, points, threshold_distance, *hashing_offset)
    //    }).collect();
    //let final_graph = fuse_graphs(graphs, points.len());
    //let connected_components = final_graph.compute_connected_components();
    //assert!(connected_components.len() > 0);
}

pub fn wrapper_parallel_opt(points: &[Point], threshold_distance: f64) {
    let squares = hash_points(points, threshold_distance);
    use rayon::prelude::IndexedParallelIterator;
    let hashing_offsets = vec![
        (0.0, 0.0),
        (threshold_distance, 0.0),
        (0.0, threshold_distance),
        (threshold_distance, threshold_distance),
    ];

    //#[cfg(features = "logs")]
    //{
    let graphs: Vec<Graph> = Logged::new(
        rayon::prelude::IntoParallelRefIterator::par_iter(&squares).zip(
            rayon::prelude::IntoParallelRefIterator::par_iter(&hashing_offsets),
        ),
    ) // TODO: fixme
    .map(|(square, hashing_offset)| {
        Graph::parallel_new_opt(&square, points, threshold_distance, *hashing_offset)
    }).collect();
    let final_graph = fuse_graphs(graphs, points.len());
    let connected_components = final_graph.compute_connected_components();
    assert!(connected_components.len() > 0);
    //}
    //#[cfg(not(features = "logs"))]
    //{
    //let graphs: Vec<Graph> = squares
    //    .par_iter()
    //    .zip(hashing_offsets.par_iter())
    //    .map(|(square, hashing_offset)| {
    //        Graph::parallel_new_opt(&square, points, threshold_distance, *hashing_offset)
    //    }).collect();
    //let final_graph = fuse_graphs(graphs, points.len());
    //let connected_components = final_graph.compute_connected_components();
    //assert!(connected_components.len() > 0);
    //}
}

pub fn get_random_points(num_points: usize) -> Vec<Point> {
    repeat_call(|| Point::new(random(), random()))
        .take(num_points)
        .collect()
}
