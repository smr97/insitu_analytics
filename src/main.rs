extern crate grouille;
extern crate itertools;
extern crate rand;
extern crate rayon;
#[cfg(feature = "logs")]
extern crate rayon_logs;
extern crate thread_binder;
extern crate time;
pub mod clique;
pub mod mymerge;
pub mod parallel_adaptive;
pub mod parallel_rayon;
pub mod sequential_algorithm;
mod wrapper_functions;
#[cfg(feature = "logs")]
use rayon_logs::ThreadPoolBuilder;
use thread_binder::*;
use time::precise_time_ns;
use wrapper_functions::*;
const THRESHOLD_DISTANCE: f64 = 0.01;
const NUM_POINTS: usize = 100_000;
const NUM_THREADS: usize = 2;
const RUNS_NUMBER: usize = 5;
fn main() {
    #[cfg(feature = "logs")]
    {
        //println!("In features");
        let pool = ThreadPoolBuilder::new()
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
    #[cfg(not(feature = "logs"))]
    {
        (2..15).for_each(|thread_num| {
            let pool = BindableThreadPool::new(POLICY::ROUND_ROBIN_CORE)
                .num_threads(thread_num)
                .build()
                .expect("Pool creation failed");
            pool.install(|| {
                (0..RUNS_NUMBER).for_each(|run| {
                    let number_of_points = NUM_POINTS + run * 100_000;
                    let input = get_random_points(number_of_points);
                    let sequential_time_st = precise_time_ns();
                    wrapper_sequential(&input, THRESHOLD_DISTANCE);
                    let sequential_time_end = precise_time_ns();
                    println!(
                        "SEQUENTIAL, {}, {}",
                        1,
                        sequential_time_end - sequential_time_st
                    );

                    let input = get_random_points(number_of_points);
                    let parallel_time_st = precise_time_ns();
                    wrapper_parallel(&input, THRESHOLD_DISTANCE);
                    let parallel_time_end = precise_time_ns();
                    println!(
                        "RAYON PARALLEL, {}, {}",
                        thread_num,
                        parallel_time_end - parallel_time_st
                    );

                    let input = get_random_points(number_of_points);
                    let adaptive_time_st = precise_time_ns();
                    wrapper_parallel_adaptive(&input, THRESHOLD_DISTANCE);
                    let adaptive_time_end = precise_time_ns();
                    println!(
                        "ADAPTIVE PARALLEL OPT, {}, {}, {}",
                        thread_num,
                        adaptive_time_end - adaptive_time_st,
                        number_of_points
                    );
                })
            });
        });
    }
}
