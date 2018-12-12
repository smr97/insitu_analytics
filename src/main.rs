#![type_length_limit = "2097152"]
mod clique;
mod parallel_adaptive;
mod parallel_rayon;
mod sequential_algorithm;
mod wrapper_functions;
use self::wrapper_functions::*;
use grouille::Point;
use itertools::repeat_call;
use rand::random;
#[cfg(feature = "rayonlogs")]
use rayon_logs::ThreadPoolBuilder;
const THRESHOLD_DISTANCE: f64 = 0.01;
const NUM_POINTS: usize = 100_000;
const NUM_THREADS: usize = 14;
const RUNS_NUMBER: usize = 5;

fn get_random_points(num_points: usize) -> Vec<Point> {
    repeat_call(|| Point::new(random(), random()))
        .take(num_points)
        .collect()
}

fn main() {
    #[cfg(feature = "rayonlogs")]
    {
        let pool = rayon_logs::ThreadPoolBuilder::new()
            .num_threads(NUM_THREADS)
            .build()
            .expect("logging pool creation failed");
        let input = get_random_points(NUM_POINTS);
        //pool.install(|| wrapper_parallel(&input, THRESHOLD_DISTANCE))
        //    .1
        //    .save_svg("parallel_rayon.html")
        //    .expect("Failed");
        //pool.install(|| wrapper_parallel_opt(&input, THRESHOLD_DISTANCE))
        //    .1
        //    .save_svg("parallel_rayon_opt.html")
        //    .expect("Failed");
        pool.install(|| wrapper_parallel_adaptive(&input, THRESHOLD_DISTANCE))
            .1
            .save_svg("parallel_adaptive.html")
            .expect("Failed");
    }
    #[cfg(not(feature = "rayonlogs"))]
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
