use std::collections::HashMap;

use itertools;
use itertools::Itertools;
use pathfinding::prelude::bfs;

use aocutil::coord::{Coord, manhattan_distance};
use aocutil::direction::{COMPASS, Directions};
use aocutil::grid::Grid;

const DAY: u8 = 20;

fn main() {
    let input = aocutil::load_input(DAY);
    let grid = Grid::parse(&input);

    let start = grid.find_cell('S').unwrap();
    let end = grid.find_cell('E').unwrap();

    let baseline = shortest_route(&grid, start, end);
    let route_distances_from_start: HashMap<Coord, i64> = baseline.iter()
        .enumerate()
        .map(|(distance_from_start, coord)| (*coord, distance_from_start as i64))
        .collect();

    println!("Part 1: {}", cheats_saving_at_least_100(&route_distances_from_start, 2));
    println!("Part 2: {}", cheats_saving_at_least_100(&route_distances_from_start, 20));
}

fn cheats_saving_at_least_100(distances_from_start: &HashMap<Coord, i64>, max_shortcut_distance: i64) -> usize {
    distances_from_start.iter().permutations(2)
        .map(|pair| (pair[0], pair[1]))
        .map(|((&c0, &d0), (&c1, &d1))| (d1 - d0, manhattan_distance(c0, c1)))
        .filter(|&(_route_distance, shortcut_distance)| shortcut_distance <= max_shortcut_distance)
        .filter(|&(route_distance, shortcut_distance)| route_distance - shortcut_distance >= 100)
        .count()
}

fn shortest_route(grid: &Grid, start: Coord, end: Coord) -> Vec<Coord> {
    bfs(&start, |p| successors(grid, p), |p| *p == end).unwrap()
}

fn successors(grid: &Grid, coord: &Coord) -> Vec<Coord> {
    COMPASS.directions().iter()
        .map(|d| d.step(*coord))
        .filter(|&c| grid.is_in_bounds(c) && grid.get_or(c, '.') != '#')
        .collect()
}