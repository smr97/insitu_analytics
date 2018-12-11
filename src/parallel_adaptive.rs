use crate::clique::update_side;
use crate::sequential_algorithm::{hash_internal, Graph};
use grouille::Point;
use itertools::repeat_call;
use rayon_adaptive::prelude::*;
use std::cell::UnsafeCell;
use std::collections::{HashMap, HashSet};
const PREALLOCATION_FACTOR: usize = 100;
const SWITCH_THRESHOLD: usize = 500;

struct SharedGraph(UnsafeCell<Vec<Vec<usize>>>);
unsafe impl Sync for SharedGraph {}

impl Graph {
    pub(crate) fn adaptive_parallel_new(
        grid: &HashMap<(usize, usize), Vec<usize>>,
        points: &[Point],
        threshold_distance: f64,
        hashing_offsets: (f64, f64),
    ) -> Self {
        let final_graph: Vec<Vec<usize>> =
            repeat_call(|| Vec::with_capacity(points.len() / PREALLOCATION_FACTOR))
                .take(points.len())
                .collect();
        let final_graph_cell = SharedGraph(UnsafeCell::new(final_graph));
        let hashmap_vector: Vec<_> = grid.into_iter().collect();
        //TODO [ASK] why don't we have iter on (K, V) in hashmap? I don't want to do a sequential
        //collect....
        let cliques: Vec<Vec<usize>> = hashmap_vector
            .into_adapt_iter()
            .fold(
                || Vec::new(),
                |mut inner_points, (square_coordinate, square)| {
                    if square.len() > SWITCH_THRESHOLD {
                        let mut smaller_squares = hash_internal(
                            square.iter().map(|index| (*index, points[*index])),
                            threshold_distance,
                            hashing_offsets,
                            square_coordinate,
                        );
                        let mut relevant_points = HashSet::new();
                        smaller_squares.values_mut().for_each(|mut smaller_square| {
                            update_side(
                                &mut relevant_points,
                                &mut smaller_square,
                                |i| points[*i].x,
                                |i| points[*i].y,
                            );
                            update_side(
                                &mut relevant_points,
                                &mut smaller_square,
                                |i| points[*i].x,
                                |i| -(points[*i].y),
                            );
                            update_side(
                                &mut relevant_points,
                                &mut smaller_square,
                                |i| points[*i].y,
                                |i| points[*i].x,
                            );
                            update_side(
                                &mut relevant_points,
                                &mut smaller_square,
                                |i| points[*i].y,
                                |i| -(points[*i].x),
                            );
                        });
                        inner_points.extend(
                            smaller_squares.into_iter().map(|(_, value)| value), //.cloned()
                        );
                        //TODO [ASK] Again, what do we do for this hashset?
                        let relevant_points_clone =
                            relevant_points.iter().cloned().collect::<Vec<usize>>();
                        relevant_points_clone.into_adapt_iter().for_each(|point| {
                            unsafe { final_graph_cell.0.get().as_mut() }.unwrap()[*point].extend(
                                relevant_points
                                    .iter()
                                    .filter(|&p| {
                                        *p != *point
                                            && points[*point].distance_to(&points[*p])
                                                <= threshold_distance
                                    })
                                    .cloned(),
                            );
                        });
                    } else {
                        //TODO make this adaptive.
                        square.into_adapt_iter().for_each(|point| {
                            unsafe { final_graph_cell.0.get().as_mut() }.unwrap()[*point].extend(
                                square
                                    .iter()
                                    .filter(|&p| {
                                        p != point
                                            && points[*point as usize]
                                                .distance_to(&points[*p as usize])
                                                <= threshold_distance
                                    })
                                    .cloned(),
                            );
                        });
                    }
                    inner_points
                },
            )
            .into_iter()
            .fold(Vec::new(), |mut final_vector, some_vector| {
                final_vector.extend(some_vector);
                final_vector
            });
        Graph {
            relevant_points: final_graph_cell.0.into_inner(),
            cliques: cliques,
        }
        //Graph {
        //    relevant_points: Vec::new(),
        //    cliques: Vec::new(),
        //}
    }
}
