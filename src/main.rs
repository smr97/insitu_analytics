#![type_length_limit = "2097152"]
mod clique;
mod parallel_adaptive;
mod parallel_rayon;
mod sequential_algorithm;
mod wrapper_functions;
#[cfg(feature = "rayon_logs")]
use crate::sequential_algorithm::*;
use grouille::Point;
use rand::random;
use rayon::prelude::*;
use std::cell::UnsafeCell;
use std::iter::repeat_with;
const THRESHOLD_DISTANCE: f64 = 0.5;
const MAX_NUM_POINTS: usize = 40;
const RUNS_NUM: f64 = 50.0;
struct SharedGraph(UnsafeCell<Vec<Vec<usize>>>);
unsafe impl Sync for SharedGraph {}
fn get_random_points(num_points: usize) -> Vec<Point> {
    repeat_with(|| Point::new(random(), random()))
        .take(num_points)
        .collect()
}

fn main() {
    (13..14).for_each(|thread_num| {
        let pool = thread_binder::BindableThreadPool::new(thread_binder::POLICY::ROUND_ROBIN_CORE)
            .num_threads(thread_num)
            .build()
            .expect("Pool creation failed");
        pool.install(|| {
            (1..MAX_NUM_POINTS + 1).for_each(|points| {
                let number_of_points = points * 100;
                let input = &get_random_points(number_of_points);
                let point_indices = (0..number_of_points).collect::<Vec<_>>();
                let final_graph: Vec<Vec<usize>> = repeat_with(|| Vec::with_capacity(input.len()))
                    .take(input.len())
                    .collect();
                let final_graph_cell = SharedGraph(UnsafeCell::new(final_graph));
                let mut parallel_time_ms = 0.0;
                (0..RUNS_NUM as usize).for_each(|_| {
                    let start = time::precise_time_ns();
                    point_indices.par_iter().for_each(|point| {
                        unsafe { final_graph_cell.0.get().as_mut() }.unwrap()[*point].extend(
                            point_indices
                                .iter()
                                .filter(|&p| {
                                    p != point
                                        && input[*point as usize].distance_to(&input[*p as usize])
                                            <= THRESHOLD_DISTANCE
                                })
                                .cloned(),
                        );
                    });
                    let end = time::precise_time_ns();
                    parallel_time_ms += (end - start) as f64 / 1e6;
                });
                parallel_time_ms /= RUNS_NUM;
                //Sequential run
                let input = &get_random_points(number_of_points);
                let point_indices = (0..number_of_points).collect::<Vec<_>>();
                let mut final_graph: Vec<Vec<usize>> =
                    repeat_with(|| Vec::with_capacity(input.len()))
                        .take(input.len())
                        .collect();
                let mut sequential_time_ms = 0.0;
                (0..RUNS_NUM as usize).for_each(|_| {
                    let start = time::precise_time_ns();
                    for point in &point_indices {
                        final_graph[*point as usize].extend(
                            point_indices
                                .iter()
                                .filter(|&p| {
                                    p != point
                                        && input[*point as usize].distance_to(&input[*p as usize])
                                            <= THRESHOLD_DISTANCE
                                })
                                .cloned(),
                        );
                    }
                    let end = time::precise_time_ns();
                    sequential_time_ms += (end - start) as f64 / 1e6;
                });
                sequential_time_ms /= RUNS_NUM;
                println!(
                    "{}, {}, {}",
                    number_of_points, parallel_time_ms, sequential_time_ms
                );
            })
        });
    });
}
