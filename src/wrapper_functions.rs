use crate::sequential_algorithm::*;
use grouille::Point;
#[cfg(not(features = "rayonlogs"))]
use rayon::prelude::*;
use rayon_adaptive::prelude::*;
use time::precise_time_ns;
pub fn wrapper_sequential(points: &[grouille::Point], threshold_distance: f64) -> f64 {
    let squares = hash_points(points, threshold_distance);
    let st = time::precise_time_ns();
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
        })
        .collect();
    let end = time::precise_time_ns();
    let final_graph = fuse_graphs(graphs, points.len());
    let connected_components = final_graph.compute_connected_components();
    assert!(connected_components.len() > 0);
    (end - st) as f64 / (1e6 as f64)
}

pub fn wrapper_parallel(points: &[Point], threshold_distance: f64) -> f64 {
    let squares = hash_points(points, threshold_distance);
    let hashing_offsets = vec![
        (0.0, 0.0),
        (threshold_distance, 0.0),
        (0.0, threshold_distance),
        (threshold_distance, threshold_distance),
    ];
    #[cfg(feature = "logs")]
    {
        use rayon::prelude::IndexedParallelIterator;
        use rayon_logs::Logged;
        let st = precise_time_ns();
        let graphs: Vec<Graph> = Logged::new(
            rayon::prelude::IntoParallelRefIterator::par_iter(&squares).zip(
                rayon::prelude::IntoParallelRefIterator::par_iter(&hashing_offsets),
            ),
        ) // TODO: fixme
        .map(|(square, hashing_offset)| {
            Graph::parallel_new(&square, points, threshold_distance, *hashing_offset)
        })
        .collect();
        let end = precise_time_ns();
        let final_graph = fuse_graphs(graphs, points.len());
        let connected_components = final_graph.compute_connected_components();
        assert!(connected_components.len() > 0);
        (end - st) as f64 / (1e6 as f64)
    }
    #[cfg(not(feature = "logs"))]
    {
        let st = precise_time_ns();
        let graphs: Vec<Graph> = squares
            .par_iter()
            .zip(hashing_offsets.par_iter())
            .map(|(square, hashing_offset)| {
                Graph::parallel_new(&square, points, threshold_distance, *hashing_offset)
            })
            .collect();
        let end = precise_time_ns();
        let final_graph = fuse_graphs(graphs, points.len());
        let connected_components = final_graph.compute_connected_components();
        assert!(connected_components.len() > 0);
        (end - st) as f64 / (1e6 as f64)
    }
}

pub fn wrapper_parallel_adaptive(points: &[Point], threshold_distance: f64) -> f64 {
    let squares = hash_points(points, threshold_distance);
    let hashing_offsets = vec![
        (0.0, 0.0),
        (threshold_distance, 0.0),
        (0.0, threshold_distance),
        (threshold_distance, threshold_distance),
    ];
    let st = precise_time_ns();
    let graphs: Vec<Graph> = squares
        .into_adapt_iter()
        .zip(hashing_offsets.into_adapt_iter())
        .map(|(square, hashing_offset)| {
            Graph::adaptive_parallel_new(&square, points, threshold_distance, *hashing_offset)
        })
        .collect();
    let end = precise_time_ns();
    let final_graph = fuse_graphs(graphs, points.len());
    let connected_components = final_graph.compute_connected_components();
    assert!(connected_components.len() > 0);
    (end - st) as f64 / (1e6 as f64)
}

pub fn wrapper_parallel_opt(points: &[Point], threshold_distance: f64) -> f64 {
    let squares = hash_points(points, threshold_distance);
    let hashing_offsets = vec![
        (0.0, 0.0),
        (threshold_distance, 0.0),
        (0.0, threshold_distance),
        (threshold_distance, threshold_distance),
    ];

    #[cfg(feature = "logs")]
    {
        use self::rayon_logs::Logged;
        use rayon::prelude::IndexedParallelIterator;
        let st = precise_time_ns();
        let graphs: Vec<Graph> = Logged::new(
            rayon::prelude::IntoParallelRefIterator::par_iter(&squares).zip(
                rayon::prelude::IntoParallelRefIterator::par_iter(&hashing_offsets),
            ),
        ) // TODO: fixme
        .map(|(square, hashing_offset)| {
            Graph::parallel_new_opt(&square, points, threshold_distance, *hashing_offset)
        })
        .collect();
        let end = precise_time_ns();
        let final_graph = fuse_graphs(graphs, points.len());
        let connected_components = final_graph.compute_connected_components();
        assert!(connected_components.len() > 0);
        (end - st) as f64 / (1e6 as f64)
    }
    #[cfg(not(feature = "logs"))]
    {
        let st = precise_time_ns();
        let graphs: Vec<Graph> = squares
            .par_iter()
            .zip(hashing_offsets.par_iter())
            .map(|(square, hashing_offset)| {
                Graph::parallel_new_opt(&square, points, threshold_distance, *hashing_offset)
            })
            .collect();
        let end = precise_time_ns();
        let final_graph = fuse_graphs(graphs, points.len());
        let connected_components = final_graph.compute_connected_components();
        assert!(connected_components.len() > 0);
        (end - st) as f64 / (1e6 as f64)
    }
}
