#![type_length_limit = "2097152"]
mod clique;
mod parallel_adaptive;
mod parallel_rayon;
mod sequential_algorithm;
mod wrapper_functions;
#[cfg(feature = "rayon_logs")]
use crate::sequential_algorithm::*;
use grouille::Point;
#[macro_use]
extern crate itertools;
use itertools::Itertools;
use rand::random;
use std::cell::UnsafeCell;
use std::iter::repeat_with;
const THRESHOLD_DISTANCE: f64 = 0.01;
const NUM_POINTS: usize = 200_000;
const RUNS_NUMBER: usize = 25;

const PREALLOCATION_FACTOR: usize = 100;
struct SharedGraph(UnsafeCell<Vec<Vec<usize>>>);
unsafe impl Sync for SharedGraph {}
fn get_random_points(num_points: usize) -> Vec<Point> {
    repeat_with(|| Point::new(random(), random()))
        .take(num_points)
        .collect()
}

fn main() {
    use rayon_adaptive::prelude::*;
    (13..14).for_each(|thread_num| {
        let pool = thread_binder::BindableThreadPool::new(thread_binder::POLICY::ROUND_ROBIN_CORE)
            .num_threads(thread_num)
            .build()
            .expect("Pool creation failed");
        pool.install(|| {
            (0..RUNS_NUMBER).for_each(|_| {
                println!(
                    "Experimental setup:: NUM_POINTS: {}, NUM_THREADS: {}",
                    NUM_POINTS, thread_num
                );
                let input = &get_random_points(NUM_POINTS);
                let point_indices = (0..NUM_POINTS).collect::<Vec<_>>();
                let final_graph: Vec<Vec<usize>> =
                    repeat_with(|| Vec::with_capacity(input.len() / PREALLOCATION_FACTOR))
                        .take(input.len())
                        .collect();
                let final_graph_cell = SharedGraph(UnsafeCell::new(final_graph));
                let start = time::precise_time_ns();
                point_indices.into_adapt_iter().for_each(|point| {
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
                let parallel_time_ms = (end - start) as f64 / 1e6;
                println!("Adaptive time {}", parallel_time_ms);

                //Sequential run
                let input = &get_random_points(NUM_POINTS);
                let point_indices = (0..NUM_POINTS).collect::<Vec<_>>();
                let mut final_graph: Vec<Vec<usize>> =
                    repeat_with(|| Vec::with_capacity(input.len() / PREALLOCATION_FACTOR))
                        .take(input.len())
                        .collect();
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
                let sequential_time_ms = (end - start) as f64 / 1e6;
                println!("Sequential time {}", sequential_time_ms);
            })
        });
    });
}
