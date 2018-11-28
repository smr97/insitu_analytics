extern crate grouille;
extern crate itertools;
extern crate rand;
extern crate rayon;
extern crate rayon_logs;
extern crate time;
pub mod clique;
pub mod mymerge;
mod parallel_rayon;
mod sequential_algorithm;
mod wrapper_functions;
use rayon_logs::ThreadPoolBuilder;
use wrapper_functions::*;
const THRESHOLD_DISTANCE: f64 = 0.01;
const NUM_POINTS: usize = 100_000;
const NUM_THREADS: usize = 2;
fn main() {
    let pool = ThreadPoolBuilder::new()
        .num_threads(NUM_THREADS)
        //.bind_threads()
        .build()
        .expect("Pool creation failed");
    let input = get_random_points(NUM_POINTS);
    println!("inside logs");
    pool.install(|| wrapper_parallel(&input, THRESHOLD_DISTANCE))
        .1
        .save_svg("parallel_rayon.html")
        .expect("Failed");

    pool.install(|| wrapper_parallel_opt(&input, THRESHOLD_DISTANCE))
        .1
        .save_svg("parallel_rayon_opt.html")
        .expect("Failed");
    //pool.compare()
    //    .runs_number(5)
    //    .attach_algorithm_with_setup(
    //        "sequential",
    //        || get_random_points(NUM_POINTS),
    //        |vec| {
    //            wrapper_sequential(&vec, THRESHOLD_DISTANCE);
    //            vec
    //        },
    //    ).attach_algorithm_with_setup(
    //        "parallel",
    //        || get_random_points(NUM_POINTS),
    //        |vec| {
    //            wrapper_parallel(&vec, THRESHOLD_DISTANCE);
    //            vec
    //        },
    //    ).attach_algorithm_with_setup(
    //        "parallel optimal reduce",
    //        || get_random_points(NUM_POINTS),
    //        |vec| {
    //            wrapper_parallel_opt(&vec, THRESHOLD_DISTANCE);
    //            vec
    //        },
    //    ).generate_logs(format!(
    //        "comparisons_{}K_{}threads.html",
    //        NUM_POINTS as u32 / (1e3 as u32),
    //        NUM_THREADS
    //    )).expect("comparison failed");
}
