use clique::update_side;
use grouille::{tycat::colored_display, Point, Segment};
use itertools::repeat_call;
use itertools::Itertools;
use rand::random;
use std::collections::HashMap;
use std::collections::HashSet;
const TESTS_NUMBER: u64 = 100;
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn check_hashing() {
//        const THRESHOLD_DISTANCE: f64 = 0.0032;
//        (0..100).for_each(|_| {
//            let points: Vec<_> = repeat_call(|| Point::new(random(), random()))
//                .take(200)
//                .collect();
//            let squares = hash_points(&points, THRESHOLD_DISTANCE);
//            let graphs: Vec<Graph> = squares
//                .iter()
//                .map(|square| Graph::new(&square, &points, THRESHOLD_DISTANCE))
//                .collect();
//            let final_graph = fuse_graphs(graphs, points.len());
//            points
//                .iter()
//                .enumerate()
//                .cartesian_product(points.iter().enumerate())
//                .filter(
//                    |((source_index, source), (destination_index, destination))| {
//                        source.distance_to(&destination) <= THRESHOLD_DISTANCE
//                            && source_index != destination_index
//                    },
//                ).for_each(
//                    |((source_index, source), (destination_index, destination))| {
//                        assert!(
//                            match final_graph[source_index]
//                                .iter()
//                                .position(|some_point| *some_point == destination_index)
//                            {
//                                Some(num) => true,
//                                None => false,
//                            },
//                            "points are {:?}, {:?}",
//                            source,
//                            destination
//                        );
//                    },
//                );
//        });
//    }
//}

pub struct Graph {
    relevant_points: Vec<Vec<usize>>,
    cliques: Vec<Vec<usize>>,
}
impl Graph {
    //pub fn display_graph(points: &[Point], graph: &[Vec<usize>]) {
    //    let segments: Vec<Vec<Segment>> = graph
    //        .iter()
    //        .enumerate()
    //        .map(|(point_index, neighbours_indices)| {
    //            neighbours_indices
    //                .iter()
    //                .map(|neighbour_index| {
    //                    Segment::new(points[point_index], points[*neighbour_index])
    //                }).collect()
    //        }).collect();
    //    //colored_display(&segments).expect("displaying graph failed");
    //    //for s in &segments {
    //    if !segments.is_empty() {
    //        tycat!(points, segments);
    //    }
    //    //}
    //}

    pub fn new(
        grid: &HashMap<(usize, usize), Vec<usize>>,
        points: &[Point],
        threshold_distance: f64,
    ) -> Self {
        let mut final_graph: Vec<Vec<usize>> = repeat_call(Vec::new).take(points.len()).collect();
        //println!("we have {} squares", grid.len());
        let mut inner_points: Vec<Vec<usize>> = Vec::new();
        for square in grid.values() {
            let mut smaller_squares = hash_internal(
                square.iter().map(|index| (*index, points[*index])),
                threshold_distance,
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
                    |i| -points[*i].y,
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
                    |i| -points[*i].x,
                );
            });
            inner_points.extend(
                smaller_squares.into_iter().map(|(_, value)| value), //.cloned()
            );
            //tycat!(
            //    square
            //        .iter()
            //        .map(|index| points[*index])
            //        .collect::<Vec<Point>>(),
            //    relevant_points
            //        .iter()
            //        .map(|index| points[*index])
            //        .collect::<Vec<Point>>()
            //);
            for point in &relevant_points {
                final_graph[*point] = Vec::with_capacity(points.len() / 10000);
                final_graph[*point].extend(
                    relevant_points
                        .iter()
                        .filter(|&p| {
                            *p != *point
                                && points[*point].distance_to(&points[*p]) <= threshold_distance
                        }).cloned(),
                );
            }
        }
        Graph {
            relevant_points: final_graph,
            cliques: inner_points,
        }
    }

    fn traverse(
        &self,
        point_index: usize,
        visited: &mut Vec<bool>,
        connected_component: &mut Vec<usize>,
    ) {
        let mut mystack = vec![point_index];
        while let Some(point_index) = mystack.pop() {
            if visited[point_index] {
                continue;
            }
            visited[point_index] = true;
            connected_component.push(point_index);
            mystack.extend(
                self.relevant_points[point_index]
                    .iter()
                    .filter(|neighbour_index| visited[**neighbour_index] == false),
            );
        }
    }

    pub fn compute_connected_components(&self) -> Vec<Vec<usize>> {
        let mut visited: Vec<bool> = (0..self.relevant_points.len()).map(|_| false).collect();
        (0..self.relevant_points.len())
            .filter_map(|point_index| {
                if visited[point_index] == false {
                    let mut temp = Vec::new();
                    self.traverse(point_index, &mut visited, &mut temp);
                    Some(temp)
                } else {
                    None
                }
            }).collect::<Vec<Vec<usize>>>()
    }
}

fn hash_internal<I>(points: I, threshold_distance: f64) -> HashMap<(usize, usize), Vec<usize>>
where
    I: Iterator<Item = (usize, Point)>,
{
    let side = threshold_distance / 2.0f64.sqrt();
    let mut small_squares = HashMap::new();
    let hash_function = |p: &Point| ((p.x / side).floor() as usize, (p.y / side).floor() as usize);
    points.for_each(|(index, point)| {
        let key = hash_function(&point);
        small_squares
            .entry(key)
            .or_insert_with(Vec::new)
            .push(index);
    });
    small_squares
}

pub fn fuse_graphs(graphs: Vec<Graph>, number_of_points: usize) -> Graph {
    let mut outer_vertices: Vec<Vec<usize>> = Vec::with_capacity(number_of_points);
    outer_vertices.extend({
        (0..number_of_points).map(|point_index| {
            let mut row: Vec<usize> = Vec::with_capacity(number_of_points / 10000);
            row.extend(
                graphs
                    .iter()
                    .map(|graph| graph.relevant_points[point_index].iter())
                    .kmerge()
                    .dedup()
                    .clone(),
            );
            row
        })
    });
    let mut graphs = graphs.into_iter();
    let mut cliques = graphs.next().unwrap().cliques;
    graphs.for_each(|g| {
        cliques.extend(g.cliques);
    });
    Graph {
        relevant_points: outer_vertices,
        cliques,
    }
}

pub fn hash_points(
    points: &[Point],
    threshold_distance: f64,
) -> Vec<HashMap<(usize, usize), Vec<usize>>> {
    let side: f64 = threshold_distance * 2.0f64;
    let hash_functions = [
        Box::new(|p: &Point| ((p.x / side).floor() as usize, (p.y / side).floor() as usize))
            as Box<Fn(&Point) -> (usize, usize)>,
        Box::new(|p: &Point| {
            (
                ((p.x + side / 2.0) / side).floor() as usize,
                (p.y / side).floor() as usize,
            )
        }),
        Box::new(|p: &Point| {
            (
                (p.x / side).floor() as usize,
                ((p.y + side / 2.0) / side).floor() as usize,
            )
        }),
        Box::new(|p: &Point| {
            (
                ((p.x + side / 2.0) / side).floor() as usize,
                ((p.y + side / 2.0) / side).floor() as usize,
            )
        }),
    ];
    let mut squares: Vec<HashMap<(usize, usize), Vec<usize>>> =
        repeat_call(HashMap::new).take(4).collect();
    for (index, point) in points.iter().enumerate() {
        for (map, hash_function) in squares.iter_mut().zip(hash_functions.iter()) {
            let key = hash_function(point);
            map.entry(key).or_insert_with(Vec::new).push(index);
        }
    }
    squares
}
