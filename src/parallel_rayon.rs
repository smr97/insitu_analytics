#[cfg(feature = "logs")]
extern crate rayon_logs;
use clique::update_side;
use grouille::Point;
use itertools::repeat_call;
#[cfg(feature = "logs")]
use parallel_rayon::rayon_logs::prelude::*;
#[cfg(not(feature = "logs"))]
use rayon::prelude::*;
use sequential_algorithm::*;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
const PREALLOCATION_FACTOR: usize = 100;
const SWITCH_THRESHOLD: usize = 500;

struct SharedGraph(UnsafeCell<Vec<Vec<usize>>>);
unsafe impl Sync for SharedGraph {}

impl Graph {
    pub fn parallel_new(
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
        let inner_points: LinkedList<Vec<usize>> = grid
            .into_par_iter()
            .map(|(square_coordinate, square)| {
                let mut inner_points: LinkedList<Vec<usize>> = LinkedList::new();
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
                    relevant_points.par_iter().for_each(|point| {
                        unsafe { final_graph_cell.0.get().as_mut() }.unwrap()[*point].extend(
                            relevant_points
                                .iter()
                                .filter(|&p| {
                                    *p != *point
                                        && points[*point].distance_to(&points[*p])
                                            <= threshold_distance
                                }).cloned(),
                        );
                    })
                } else {
                    square.into_par_iter().for_each(|point| {
                        unsafe { final_graph_cell.0.get().as_mut() }.unwrap()[*point].extend(
                            square
                                .iter()
                                .filter(|&p| {
                                    p != point
                                        && points[*point as usize].distance_to(&points[*p as usize])
                                            <= threshold_distance
                                }).cloned(),
                        );
                    });
                }
                inner_points
            }).reduce(
                || LinkedList::new(),
                move |mut l1, mut l2| {
                    l1.append(&mut l2);
                    l1
                },
            );
        Graph {
            relevant_points: final_graph_cell.0.into_inner(),
            cliques: inner_points.into_iter().collect(),
        }
    }

    pub fn parallel_new_opt(
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
        let inner_points: Vec<Vec<usize>> = grid
            .into_par_iter()
            .fold(
                || Vec::new(),
                |mut inner_points, (square_coordinate, square)| {
                    //let mut inner_points: LinkedList<Vec<usize>> = LinkedList::new();
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
                        relevant_points.par_iter().for_each(|point| {
                            unsafe { final_graph_cell.0.get().as_mut() }.unwrap()[*point].extend(
                                relevant_points
                                    .iter()
                                    .filter(|&p| {
                                        *p != *point
                                            && points[*point].distance_to(&points[*p])
                                                <= threshold_distance
                                    }).cloned(),
                            );
                        })
                    } else {
                        square.into_par_iter().for_each(|point| {
                            unsafe { final_graph_cell.0.get().as_mut() }.unwrap()[*point].extend(
                                square
                                    .iter()
                                    .filter(|&p| {
                                        p != point
                                            && points[*point as usize]
                                                .distance_to(&points[*p as usize])
                                                <= threshold_distance
                                    }).cloned(),
                            );
                        });
                    }
                    inner_points
                },
            ).map(|some_vec| {
                let mut list = LinkedList::new();
                list.push_back(some_vec);
                list
            }).reduce(
                || LinkedList::new(),
                move |mut l1, mut l2| {
                    l1.append(&mut l2);
                    l1
                },
            ).into_iter()
            .fold(Vec::new(), |mut final_vec, list_elem| {
                final_vec.extend(list_elem);
                final_vec
            });
        Graph {
            relevant_points: final_graph_cell.0.into_inner(),
            cliques: inner_points,
        }
    }
}
