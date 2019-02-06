#![type_length_limit = "2097152"]
mod clique;
mod parallel_adaptive;
mod parallel_rayon;
mod sequential_algorithm;
mod wrapper_functions;
use self::wrapper_functions::*;
#[cfg(feature = "rayon_logs")]
use crate::sequential_algorithm::*;
use grouille::Point;
#[macro_use]
extern crate itertools;
use itertools::Itertools;
use rand::random;
use rayon::prelude::*;
#[cfg(feature = "rayon_logs")]
use rayon_adaptive::{prelude::*, Policy};
#[cfg(feature = "rayon_logs")]
use rayon_logs::{Logged, RunLog, Stats, ThreadPoolBuilder};
use std::iter::repeat_with;
const THRESHOLD_DISTANCE: f64 = 0.01;
const NUM_POINTS: usize = 100_000;
const RUNS_NUMBER: usize = 1;

fn get_random_points(num_points: usize) -> Vec<Point> {
    repeat_with(|| Point::new(random(), random()))
        .take(num_points)
        .collect()
}

#[cfg(feature = "rayon_logs")]
fn print_stats(run_log: Vec<RunLog>, num_threads: usize) {
    let vec_run_logs = vec![run_log];
    let stats = Stats::get_statistics(&vec_run_logs, num_threads as f64, RUNS_NUMBER as f64);
    println!("The total times are");
    for time in stats.total_times() {
        println!("{}, ", time);
    }
    println!("The idle times are");
    for time in stats.idle_times() {
        println!("{}, ", time);
    }
    println!("The tagged times are");
    for time in stats.sequential_times() {
        println!("{:?}, ", time);
    }
    println!("Average number of tasks is",);
    for num in stats.tasks_count() {
        println!("{}, ", num);
    }
    println!("Average number of stolen tasks is",);
    for num in stats.succesfull_average_steals() {
        println!("{}, ", num);
    }
}

fn main() {
    #[cfg(feature = "rayon_logs")]
    {
        let thread_nums = vec![7];
        let numbers_of_points = vec![75_000];
        let thresholds = vec![0.007];
        for (num_threads, num_points, threshold_distance) in iproduct!(
            thread_nums.into_iter(),
            numbers_of_points.into_iter(),
            thresholds.into_iter()
        ) {
            println!(
                "number of threads {}, number of points {}, threshold {}",
                num_threads, num_points, threshold_distance
            );
            let pool = rayon_logs::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                //.bind_threads()
                .build()
                .expect("logging pool creation failed");
            let input = get_random_points(num_points);
            let squares = hash_points(&input, threshold_distance);
            for square in &squares {
                println!("The number of squares will be {}", square.len());
            }
            let hashing_offsets = vec![
                (0.0, 0.0),
                (threshold_distance, 0.0),
                (0.0, threshold_distance),
                (threshold_distance, threshold_distance),
            ];
            let run_log = repeat_with(|| {
                pool.install(|| {
                    squares
                        .into_adapt_iter()
                        .zip(hashing_offsets.into_adapt_iter())
                        .map(|(square, hashing_offset)| {
                            Graph::adaptive_parallel_new(
                                &square,
                                &input,
                                threshold_distance,
                                *hashing_offset,
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .1
            })
            .take(RUNS_NUMBER)
            .collect::<Vec<RunLog>>();
            run_log[RUNS_NUMBER / 2].save_svg_with_filter(
                format!(
                    "adaptive_log_{}_threshold_{}_points.svg",
                    threshold_distance, num_points
                ),
                2,
            );
            //run_log[RUNS_NUMBER / 2]
            //    .save(format!(
            //        "parallel_adaptive_{}_threads_{}_pts_{}_thresh.json",
            //        num_threads, num_points, threshold_distance
            //    ))
            //    .expect("Failed");
            println!("Adaptive stats:");
            print_stats(run_log, num_threads);
            //let run_log = repeat_with(|| {
            //    pool.install(|| {
            //        Logged::new(
            //            rayon::prelude::IntoParallelRefIterator::par_iter(&squares).zip(
            //                rayon::prelude::IntoParallelRefIterator::par_iter(&hashing_offsets),
            //            ),
            //        )
            //        .map(|(square, hashing_offset)| {
            //            Graph::parallel_new_opt(
            //                &square,
            //                &input,
            //                threshold_distance,
            //                *hashing_offset,
            //            )
            //        })
            //        .collect::<Vec<_>>()
            //    })
            //    .1
            //})
            //.take(RUNS_NUMBER)
            //.collect::<Vec<RunLog>>();
            //run_log[RUNS_NUMBER / 2].save_svg_with_filter(
            //    format!(
            //        "rayon_log_{}_threshold_{}_points.svg",
            //        threshold_distance, num_points
            //    ),
            //    2,
            //);
            //run_log[RUNS_NUMBER / 2]
            //    .save(format!(
            //        "rayon_{}_threads_{}_pts_{}_thresh.json",
            //        num_threads, num_points, threshold_distance
            //    ))
            //    .expect("Failed");
            //println!("Rayon stats:");
            //print_stats(run_log, num_threads);
            //let run_log = repeat_with(|| {
            //    pool.install(|| {
            //        squares
            //            .into_adapt_iter()
            //            .zip(hashing_offsets.into_adapt_iter())
            //            .map(|(square, hashing_offset)| {
            //                Graph::adaptive_rayon_new(
            //                    &square,
            //                    &input,
            //                    threshold_distance,
            //                    *hashing_offset,
            //                )
            //            })
            //            .with_policy(Policy::Rayon)
            //            .collect::<Vec<_>>()
            //    })
            //    .1
            //})
            //.take(RUNS_NUMBER)
            //.collect::<Vec<RunLog>>();
            //run_log[RUNS_NUMBER / 2].save_svg(format!(
            //    "rayon_log_{}_threshold_{}_points.svg",
            //    threshold_distance, num_points
            //));
            //run_log[RUNS_NUMBER / 2]
            //    .save(format!(
            //        "with_policy_rayon_{}_threads_{}_pts_{}_thresh.json",
            //        num_threads, num_points, threshold_distance
            //    ))
            //    .expect("Failed");
            //println!("with_policy(Policy::Rayon) stats:");
            //print_stats(run_log, num_threads);
            //let run_log = repeat_with(|| {
            //    pool.install(|| {
            //        squares
            //            .iter()
            //            .zip(hashing_offsets.iter())
            //            .map(|(square, hashing_offset)| {
            //                Graph::new(&square, &input, threshold_distance, *hashing_offset)
            //            })
            //            .collect::<Vec<_>>()
            //    })
            //    .1
            //})
            //.take(RUNS_NUMBER)
            //.collect::<Vec<RunLog>>();
            //println!("Sequential stats:");
            //print_stats(run_log, num_threads);
        }
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
        (13..14).for_each(|thread_num| {
            let pool =
                thread_binder::BindableThreadPool::new(thread_binder::POLICY::ROUND_ROBIN_CORE)
                    .num_threads(thread_num)
                    .build()
                    .expect("Pool creation failed");
            pool.install(|| {
                (0..RUNS_NUMBER).for_each(|run| {
                    let number_of_points = NUM_POINTS + run * 100_000;
                    let input = get_random_points(number_of_points);
                    let parallel_rayon_time_seconds = wrapper_parallel(&input, THRESHOLD_DISTANCE);
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
        });
    }
}
