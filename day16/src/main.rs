use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use aocutil::coord::Coord;
use aocutil::direction::{COMPASS, Direction, Directions};
use aocutil::grid::Grid;

const DAY: u8 = 16;

type Position = (Coord, &'static Direction);

#[derive(Clone, Eq, PartialEq)]
struct Node {
    position: Position,
    score: i64,
    path: Vec<Coord>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = aocutil::load_input(DAY);
    let grid = Grid::parse(&input);

    let solutions = get_solutions(&grid);

    println!("Part 1: {}", solutions[0].score);

    let solution_coords: HashSet<_> = solutions.iter()
        .flat_map(|Node { path: coords, .. }| coords)
        .collect();
    println!("Part 2: {}", solution_coords.len());
}

fn get_solutions(grid: &Grid) -> Vec<Node> {
    let start = grid.find_cell('S').unwrap();
    let end = grid.find_cell('E').unwrap();
    let start_position = (start, COMPASS.east());

    let mut best_scores: HashMap<Position, i64> = HashMap::new();
    best_scores.insert(start_position, 0);

    let mut priority_queue: BinaryHeap<Node> = BinaryHeap::new();
    priority_queue.push(Node { position: start_position, score: 0, path: vec![start] });

    let mut solutions: Vec<Node> = vec![];
    while let Some(node) = priority_queue.pop() {
        if !is_best_score(&best_scores, &solutions, &node) {
            continue;
        } else if node.position.0 == end {
            solutions.push(node.clone());
        } else {
            let next_nodes: Vec<Node> = get_next_nodes(grid, &node).into_iter()
                .filter(|n| is_best_score(&best_scores, &solutions, n))
                .collect();
            for &Node { position, score, .. } in &next_nodes {
                best_scores.insert(position, score);
            }
            priority_queue.extend(next_nodes);
        }
    }

    solutions
}

fn is_best_score(best_scores: &HashMap<Position, i64>, solutions: &[Node], node: &Node) -> bool {
    if !&solutions.is_empty() && node.score > solutions[0].score {
        false
    } else {
        match best_scores.get(&node.position) {
            None => true,
            Some(&best_score) => node.score <= best_score
        }
    }
}

fn get_next_nodes(grid: &Grid, node: &Node) -> Vec<Node> {
    let mut result = vec![];
    let (coord, direction) = node.position;
    if can_step(grid, coord, direction) {
        let next_coord = direction.step(coord);
        let mut new_path = node.path.clone();
        new_path.push(next_coord);
        result.push(Node { position: (next_coord, direction), score: node.score + 1, path: new_path });
    }
    let left = COMPASS.left(direction);
    if can_step(grid, coord, left) {
        result.push(Node { position: (coord, left), score: node.score + 1000, path: node.path.clone() });
    }
    let right = COMPASS.right(direction);
    if can_step(grid, coord, right) {
        result.push(Node { position: (coord, right), score: node.score + 1000, path: node.path.clone() });
    }
    result
}

fn can_step(grid: &Grid, coord: Coord, direction: &Direction) -> bool {
    grid.get_or(direction.step(coord), '#') != '#'
}
