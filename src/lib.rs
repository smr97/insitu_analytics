#[macro_use]
extern crate grouille;
extern crate itertools;
extern crate rand;
extern crate rayon;
pub mod clique;
pub mod mymerge;
pub mod parallel_rayon;
pub mod sequential_algorithm;
pub use mymerge::*;
pub use sequential_algorithm::*;
