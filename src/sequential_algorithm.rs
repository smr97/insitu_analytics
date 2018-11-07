use grouille::{tycat::colored_display, Point, Segment};
use itertools::repeat_call;
use itertools::Itertools;
use rand::random;
use std::collections::HashMap;
const THRESHOLD_DISTANCE: f64 = 0.0032;
const TESTS_NUMBER: u64 = 100;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_hashing() {
        (0..100).for_each(|_| {
            let points: Vec<_> = repeat_call(|| Point::new(random(), random()))
                .take(200)
                .collect();
            let squares = hash_points(&points);
            let graphs: Vec<Vec<Vec<usize>>> = squares
                .iter()
                .map(|square| make_graph(&square, &points))
                .collect();
            let final_graph = fuse_graphs(&graphs, &points);
            points
                .iter()
                .enumerate()
                .cartesian_product(points.iter().enumerate())
                .filter(
                    |((source_index, source), (destination_index, destination))| {
                        source.distance_to(&destination) <= THRESHOLD_DISTANCE
                            && source_index != destination_index
                    },
                ).for_each(
                    |((source_index, source), (destination_index, destination))| {
                        assert!(
                            match final_graph[source_index]
                                .iter()
                                .position(|some_point| *some_point == destination_index)
                            {
                                Some(num) => true,
                                None => false,
                            },
                            "points are {:?}, {:?}",
                            source,
                            destination
                        );
                    },
                );
        });
    }
}

pub fn display_graph(points: &[Point], graph: &[Vec<usize>]) {
    let segments: Vec<Vec<Segment>> = graph
        .iter()
        .enumerate()
        .map(|(point_index, neighbours_indices)| {
            neighbours_indices
                .iter()
                .map(|neighbour_index| Segment::new(points[point_index], points[*neighbour_index]))
                .collect()
        }).collect();
    //colored_display(&segments).expect("displaying graph failed");
    //for s in &segments {
    if !segments.is_empty() {
        tycat!(points, segments);
    }
    //}
}

pub fn make_graph(grid: &HashMap<(usize, usize), Vec<usize>>, points: &[Point]) -> Vec<Vec<usize>> {
    let mut graph: Vec<Vec<usize>> = repeat_call(Vec::new).take(points.len()).collect();
    for square in grid.values() {
        for point in square {
            graph[*point] = Vec::with_capacity(points.len() / 10000);
            graph[*point].extend(
                square
                    .iter()
                    .filter(|&p| {
                        p != point && points[*point].distance_to(&points[*p]) <= THRESHOLD_DISTANCE
                    }).cloned(),
            );
        }
    }
    graph
}

pub fn fuse_graphs(graphs: &Vec<Vec<Vec<usize>>>, points: &[Point]) -> Vec<Vec<usize>> {
    let mut final_graph: Vec<Vec<usize>> = Vec::with_capacity(points.len() + 1);
    final_graph.extend({
        points.iter().enumerate().map(|(point_index, _)| {
            let mut row: Vec<usize> = Vec::with_capacity(points.len() / 10000);
            row.extend(
                graphs
                    .iter()
                    .map(|graph| graph[point_index].clone())
                    .kmerge()
                    .dedup(),
            );
            row
        })
    });
    final_graph
}

fn traverse(
    point_index: usize,
    graph: &Vec<Vec<usize>>,
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
            graph[point_index]
                .iter()
                .filter(|neighbour_index| visited[**neighbour_index] == false),
        );
    }
}

pub fn compute_connected_components(graph: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut visited: Vec<bool> = (0..graph.len()).map(|_| false).collect();
    (0..graph.len())
        .filter_map(|point_index| {
            if visited[point_index] == false {
                let mut temp = Vec::new();
                traverse(point_index, graph, &mut visited, &mut temp);
                Some(temp)
            } else {
                None
            }
        }).collect::<Vec<Vec<usize>>>()
}

pub fn hash_points(points: &[Point]) -> Vec<HashMap<(usize, usize), Vec<usize>>> {
    let SIDE: f64 = THRESHOLD_DISTANCE * 2.0f64;
    let hash_functions = [
        Box::new(|p: &Point| ((p.x / SIDE).floor() as usize, (p.y / SIDE).floor() as usize))
            as Box<Fn(&Point) -> (usize, usize)>,
        Box::new(|p: &Point| {
            (
                ((p.x + SIDE / 2.0) / SIDE).floor() as usize,
                (p.y / SIDE).floor() as usize,
            )
        }),
        Box::new(|p: &Point| {
            (
                (p.x / SIDE).floor() as usize,
                ((p.y + SIDE / 2.0) / SIDE).floor() as usize,
            )
        }),
        Box::new(|p: &Point| {
            (
                ((p.x + SIDE / 2.0) / SIDE).floor() as usize,
                ((p.y + SIDE / 2.0) / SIDE).floor() as usize,
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
