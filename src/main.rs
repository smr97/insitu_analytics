#![type_length_limit = "2097152"]
mod clique;
mod parallel_adaptive;
mod parallel_rayon;
mod sequential_algorithm;
mod wrapper_functions;
use crate::sequential_algorithm::*;
use grouille::Point;
use rand::random;
use rayon::prelude::*;
use rayon_adaptive::{prelude::*, Policy};
use std::cell::UnsafeCell;
use std::iter::repeat_with;
const THRESHOLD_DISTANCE: f64 = 0.025;
const NUM_POINTS: usize = 200_000;
const RUNS_NUM: f64 = 25.0;
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
            let hashing_offsets = vec![
                (0.0, 0.0),
                (THRESHOLD_DISTANCE, 0.0),
                (0.0, THRESHOLD_DISTANCE),
                (THRESHOLD_DISTANCE, THRESHOLD_DISTANCE),
            ];
            let input = get_random_points(NUM_POINTS);
            let squares = hash_points(&input, THRESHOLD_DISTANCE);
            //All rayon run
            let mut rayon_time_ms = 0.0;
            (0..RUNS_NUM as usize).for_each(|_| {
                let start = time::precise_time_ns();
                let temp_vec_1 = squares
                    .into_adapt_iter()
                    .zip(hashing_offsets.into_adapt_iter())
                    .map(|(square, hashing_offset)| {
                        Graph::adaptive_rayon_new(
                            &square,
                            &input,
                            THRESHOLD_DISTANCE,
                            *hashing_offset,
                        )
                    })
                    .with_policy(Policy::Rayon)
                    .collect::<Vec<_>>();
                let end = time::precise_time_ns();
                rayon_time_ms += (end - start) as f64 / 1e6;
            });
            rayon_time_ms /= RUNS_NUM;
            //Parallel 'adaptive' run
            let mut adaptive_time_ms = 0.0;
            (0..RUNS_NUM as usize).for_each(|_| {
                let start = time::precise_time_ns();
                let temp_vec_2 = squares
                    .into_adapt_iter()
                    .zip(hashing_offsets.into_adapt_iter())
                    .map(|(square, hashing_offset)| {
                        Graph::adaptive_parallel_new(
                            &square,
                            &input,
                            THRESHOLD_DISTANCE,
                            *hashing_offset,
                        )
                    })
                    .with_policy(Policy::JoinContext(1))
                    .collect::<Vec<_>>();
                let end = time::precise_time_ns();
                adaptive_time_ms += (end - start) as f64 / 1e6;
            });
            adaptive_time_ms /= RUNS_NUM;
            //Sequential run
            let mut sequential_time_ms = 0.0;
            (0..RUNS_NUM as usize).for_each(|_| {
                let start = time::precise_time_ns();
                let temp_vec_3 = squares
                    .iter()
                    .zip(hashing_offsets.iter())
                    .map(|(square, hashing_offset)| {
                        Graph::new(&square, &input, THRESHOLD_DISTANCE, *hashing_offset)
                    })
                    .collect::<Vec<_>>();
                let end = time::precise_time_ns();
                sequential_time_ms += (end - start) as f64 / 1e6;
            });
            sequential_time_ms /= RUNS_NUM;
            println!(
                "{}, {}, {}, {}",
                NUM_POINTS, rayon_time_ms, adaptive_time_ms, sequential_time_ms
            );
        })
    });
}
