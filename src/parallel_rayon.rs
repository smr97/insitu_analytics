#[macro_use]
use grouille::Point;
use itertools::repeat_call;
use itertools::Itertools;
use rand::random;
use rayon::prelude::*;
use std::collections::HashMap;
const THRESHOLD_DISTANCE: f64 = 0.08;

fn make_graph(grid: &HashMap<(usize, usize), Vec<usize>>, points_number: usize) -> Vec<Vec<usize>> {
    let mut graph: Vec<Vec<usize>> = repeat_call(Vec::new).take(points_number).collect();
    for square in grid.values() {
        for point in square {
            graph[*point] = square.iter().filter(|&p| p != point).cloned().collect();
        }
    }
    graph
}

fn fuse_graphs(graphs: &Vec<Vec<Vec<usize>>>, points: &[Point]) -> Vec<Vec<usize>> {
    points
        .iter()
        .enumerate()
        .map(|(point_index, point)| {
            graphs
                .iter()
                .map(|graph| graph[point_index].clone())
                .kmerge()
                .dedup()
                .filter(|neighbour| point.distance_to(&points[*neighbour]) <= THRESHOLD_DISTANCE)
                .collect()
        }).collect()
}

fn traverse(
    point_index: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    connected_component: &mut Vec<usize>,
) {
    connected_component.push(point_index);
    visited[point_index] = true;
    graph[point_index].iter().for_each(|neighbour_index| {
        if visited[*neighbour_index] == false {
            traverse(*neighbour_index, graph, visited, connected_component)
        }
    });
}

fn compute_connected_components(graph: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
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

fn hash_points(points: &[Point]) -> Vec<HashMap<(usize, usize), Vec<usize>>> {
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
    points
        .par_iter()
        .enumerate()
        .fold(HashMap::new(), |mut state, (index, point)| {
            let key = hash_functions[0](point);
            state.entry(key).or_insert_with(Vec::new).push(index);
            state
        });
    unimplemented!()
}
