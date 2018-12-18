#![type_length_limit = "2097152"]
mod clique;
mod parallel_adaptive;
mod parallel_rayon;
mod sequential_algorithm;
mod wrapper_functions;
use self::wrapper_functions::*;
use grouille::Point;
#[macro_use]
extern crate itertools;
use rand::random;
#[cfg(feature = "rayonlogs")]
use rayon_logs::ThreadPoolBuilder;
use std::iter::repeat_with;
const THRESHOLD_DISTANCE: f64 = 0.01;
const NUM_POINTS: usize = 100_000;
const RUNS_NUMBER: usize = 5;

fn get_random_points(num_points: usize) -> Vec<Point> {
    repeat_with(|| Point::new(random(), random()))
        .take(num_points)
        .collect()
}

fn main() {
    #[cfg(feature = "rayon_logs")]
    {
        let thread_nums = vec![3, 7, 11, 13, 16];
        let numbers_of_points = vec![50_000, 200_000, 300_000];
        let thresholds = vec![0.01, 0.1, 0.5];
        //let multi_prod = vec![thread_nums, numbers_of_points, thresholds]
        //    .into_iter()
        //    .multi_cartesian_product();
        //        thread_nums.into_iter().for_each(|num_threads| {
        //            numbers_of_points.into_iter().for_each(|num_points| {
        //                thresholds.into_iter().for_each(|threshold_distance| {
        for (num_threads, num_points, threshold_distance) in iproduct!(
            thread_nums.into_iter(),
            numbers_of_points.into_iter(),
            thresholds.into_iter()
        ) {
            let pool = rayon_logs::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build()
                .expect("logging pool creation failed");
            let input = get_random_points(num_points);
            pool.install(|| wrapper_parallel_adaptive(&input, threshold_distance))
                .1
                .save_svg(format!(
                    "parallel_adaptive_{}_threads_{}_pts_{}_thresh.html",
                    num_threads, num_points, threshold_distance
                ))
                .expect("Failed");
        }
        //                });
        //            });
        //        });
    }
    #[cfg(not(feature = "rayon_logs"))]
    {
        (0..RUNS_NUMBER).for_each(|run| {
            let number_of_points = NUM_POINTS + run * 100_000;
            let input: Vec<Point> = get_random_points(number_of_points);
            let sequential_time_seconds = wrapper_sequential(&input, THRESHOLD_DISTANCE);
            println!(
                "SEQUENTIAL, {}, {}, {}",
                1, sequential_time_seconds, number_of_points
            );
        });
        (2..17).for_each(|thread_num| {
            if thread_num % 2 == 1 {
                ()
            } else {
                let pool =
                    thread_binder::BindableThreadPool::new(thread_binder::POLICY::ROUND_ROBIN_CORE)
                        .num_threads(thread_num)
                        .build()
                        .expect("Pool creation failed");
                pool.install(|| {
                    (0..RUNS_NUMBER).for_each(|run| {
                        let number_of_points = NUM_POINTS + run * 100_000;
                        let input = get_random_points(number_of_points);
                        let parallel_rayon_time_seconds =
                            wrapper_parallel(&input, THRESHOLD_DISTANCE);
                        println!(
                            "RAYON PARALLEL, {}, {}, {}",
                            thread_num, parallel_rayon_time_seconds, number_of_points
                        );

                        let input = get_random_points(number_of_points);
                        let parallel_rayon_opt_time_seconds =
                            wrapper_parallel_opt(&input, THRESHOLD_DISTANCE);
                        println!(
                            "RAYON PARALLEL OPT, {}, {}, {}",
                            thread_num, parallel_rayon_opt_time_seconds, number_of_points
                        );

                        let input = get_random_points(number_of_points);
                        let parallel_adaptive_time_seconds =
                            wrapper_parallel_adaptive(&input, THRESHOLD_DISTANCE);
                        println!(
                            "ADAPTIVE PARALLEL OPT, {}, {}, {}",
                            thread_num, parallel_adaptive_time_seconds, number_of_points
                        );
                    })
                });
            }
        });
    }
}
