use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use regex::Regex;
use aocutil::coord::Coord;
use aocutil::direction::{COMPASS, Direction, Directions};
use aocutil::grid::Grid;

const DAY: u8 = 18;

#[derive(Clone, Eq, PartialEq)]
struct Node {
    position: Coord,
    ticks: i64,
    path: Vec<Coord>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.ticks.cmp(&self.ticks)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = aocutil::load_input(DAY);
    let byte_coords = parse_input(&input);

    let part1_ticks = 1024;
    let start = (0, 0);
    let end = (70, 70);

    let mut grid = Grid::new_with_coords(byte_coords.iter().take(part1_ticks), '#');
    let part1= shortest_route(&grid, start, end).unwrap();
    println!("Part 1: {}", part1.ticks);

    let part2 = part2(&mut grid, start, end, byte_coords.clone().into_iter().skip(part1_ticks).collect());
    println!("Part 2: {},{}", part2.0, part2.1);
}

fn part2(grid: &mut Grid, start: Coord, end: Coord, remaining_coords: Vec<Coord>) -> Coord {
    let mut route = shortest_route(&grid, start, end).unwrap().path;
    for coord in remaining_coords {
        grid.set(coord, '#');
        if route.contains(&coord) {
            match shortest_route(&grid, (0, 0), end) {
                Some(node) => route = node.path,
                None => return coord
            }
        }
    }
    panic!()
}

fn shortest_route(grid: &Grid, start: Coord, end: Coord) -> Option<Node> {
    let mut best_scores: HashMap<Coord, i64> = HashMap::new();

    let mut priority_queue: BinaryHeap<Node> = BinaryHeap::new();
    priority_queue.push(Node { position: start, ticks: 0, path: vec![start] });

    let mut best_solution: Option<Node> = None;
    while let Some(node) = priority_queue.pop() {
        if best_solution.as_ref().map(|best_solution| best_solution.ticks < node.ticks).unwrap_or(false) {
            continue;
        } else if node.position == end {
            best_solution = Some(node);
        } else {
            let next_nodes: Vec<Node> = get_next_nodes(&grid, &node).into_iter()
                .filter(|n| is_best_score(&best_scores, &best_solution, n))
                .collect();
            for &Node { position, ticks, .. } in &next_nodes {
                best_scores.insert(position, ticks);
            }
            priority_queue.extend(next_nodes);
        }
    }

    best_solution
}

fn is_best_score(best_scores: &HashMap<Coord, i64>, best_solution: &Option<Node>, node: &Node) -> bool {
    if !&best_solution.is_none() && node.ticks > best_solution.as_ref().map(|b| b.ticks).unwrap() {
        false
    } else {
        match best_scores.get(&node.position) {
            None => true,
            Some(&best_score) => node.ticks < best_score
        }
    }
}

fn get_next_nodes(grid: &Grid, node: &Node) -> Vec<Node> {
    let mut result = vec![];
    let coord = node.position;
    for direction in COMPASS.directions() {
        if can_step(grid, coord, direction) {
            let next_coord = direction.step(coord);
            let mut new_path = node.path.clone();
            new_path.push(next_coord);
            result.push(Node { position: next_coord, ticks: node.ticks + 1, path: new_path });
        }
    }
    result
}

fn can_step(grid: &Grid, coord: Coord, direction: &Direction) -> bool {
    grid.is_in_bounds(coord) && grid.get_or(direction.step(coord), '.') != '#'
}

fn parse_input(input: &str) -> Vec<Coord> {
    let mut result = vec![];
    let re = Regex::new(r"^(\d+),+(\d+)$").unwrap();
    for line in input.lines() {
        let (_, [c1, c2]) = re.captures(line).unwrap().extract();
        result.push((c1.parse().unwrap(), c2.parse().unwrap()));
    }
    result
}