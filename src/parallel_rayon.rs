use clique::update_side;
use grouille::Point;
use itertools::repeat_call;
//use rand::random;
//use rayon::prelude::*;
use rayon_logs::prelude::*;
use sequential_algorithm::*;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
const PREALLOCATION_FACTOR: usize = 100;
const SWITCH_THRESHOLD: usize = 500;

//pub struct Graph {
//    relevant_points: Vec<Vec<usize>>,
//    cliques: Vec<Vec<usize>>,
//}

struct SharedGraph(UnsafeCell<Vec<Vec<usize>>>);
unsafe impl Sync for SharedGraph {}

impl Graph {
    //parallelize this.
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

    //    fn traverse(
    //        &self,
    //        point_index: usize,
    //        visited: &mut Vec<bool>,
    //        connected_component: &mut Vec<usize>,
    //    ) {
    //        let mut mystack = vec![point_index];
    //        while let Some(point_index) = mystack.pop() {
    //            if visited[point_index] {
    //                continue;
    //            }
    //            visited[point_index] = true;
    //            connected_component.push(point_index);
    //            mystack.extend(
    //                self.relevant_points[point_index]
    //                    .iter()
    //                    .filter(|neighbour_index| visited[**neighbour_index] == false),
    //            );
    //        }
    //    }
    //
    //    pub fn compute_connected_components(&self) -> Vec<Vec<usize>> {
    //        let mut visited: Vec<bool> = (0..self.relevant_points.len()).map(|_| false).collect();
    //        (0..self.relevant_points.len())
    //            .filter_map(|point_index| {
    //                if visited[point_index] == false {
    //                    let mut temp = Vec::new();
    //                    self.traverse(point_index, &mut visited, &mut temp);
    //                    Some(temp)
    //                } else {
    //                    None
    //                }
    //            }).collect::<Vec<Vec<usize>>>()
    //    }
}

//fn hash_internal<I>(
//    points: I,
//    threshold_distance: f64,
//    hashing_offsets: (f64, f64),
//    coordinate: &(usize, usize),
//) -> HashMap<(usize, usize), Vec<usize>>
//where
//    I: Iterator<Item = (usize, Point)>,
//{
//    let side = threshold_distance / 2.0f64.sqrt();
//    let mut small_squares = HashMap::new();
//    let hash_function = |p: &Point| {
//        (
//            ((p.x + hashing_offsets.0 - coordinate.0 as f64 * 2.0 * threshold_distance) / side)
//                .floor() as usize,
//            ((p.y + hashing_offsets.1 - coordinate.1 as f64 * 2.0 * threshold_distance) / side)
//                .floor() as usize,
//        )
//    };
//    points.for_each(|(index, point)| {
//        let key = hash_function(&point);
//        small_squares
//            .entry(key)
//            .or_insert_with(Vec::new)
//            .push(index);
//    });
//    small_squares
//}
//
//pub fn fuse_graphs(graphs: Vec<Graph>, number_of_points: usize) -> Graph {
//    let mut outer_vertices: Vec<Vec<usize>> = Vec::with_capacity(number_of_points);
//    outer_vertices.extend({
//        (0..number_of_points).map(|point_index| {
//            let mut row: Vec<usize> = Vec::with_capacity(number_of_points / PREALLOCATION_FACTOR);
//            row.extend(
//                graphs
//                    .iter()
//                    .map(|graph| graph.relevant_points[point_index].iter())
//                    .kmerge()
//                    .dedup()
//                    .clone(),
//            );
//            row
//        })
//    });
//    let mut graphs = graphs.into_iter();
//    let mut cliques = graphs.next().unwrap().cliques;
//    graphs.for_each(|g| {
//        cliques.extend(g.cliques);
//    });
//    Graph {
//        relevant_points: outer_vertices,
//        cliques,
//    }
//}
//
//pub fn hash_points(
//    points: &[Point],
//    threshold_distance: f64,
//) -> Vec<HashMap<(usize, usize), Vec<usize>>> {
//    let side: f64 = threshold_distance * 2.0f64;
//    let hash_functions = [
//        Box::new(|p: &Point| ((p.x / side).floor() as usize, (p.y / side).floor() as usize))
//            as Box<Fn(&Point) -> (usize, usize)>,
//        Box::new(|p: &Point| {
//            (
//                ((p.x + side / 2.0) / side).floor() as usize,
//                (p.y / side).floor() as usize,
//            )
//        }),
//        Box::new(|p: &Point| {
//            (
//                (p.x / side).floor() as usize,
//                ((p.y + side / 2.0) / side).floor() as usize,
//            )
//        }),
//        Box::new(|p: &Point| {
//            (
//                ((p.x + side / 2.0) / side).floor() as usize,
//                ((p.y + side / 2.0) / side).floor() as usize,
//            )
//        }),
//    ];
//    let mut squares: Vec<HashMap<(usize, usize), Vec<usize>>> =
//        repeat_call(HashMap::new).take(4).collect();
//    for (index, point) in points.iter().enumerate() {
//        for (map, hash_function) in squares.iter_mut().zip(hash_functions.iter()) {
//            let key = hash_function(point);
//            map.entry(key).or_insert_with(Vec::new).push(index);
//        }
//    }
//    squares
//}
