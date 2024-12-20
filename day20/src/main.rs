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

    let route = shortest_route(&grid, start, end);
    println!("Part 1: {}", cheats_saving_at_least_100(&route, 2));
    println!("Part 2: {}", cheats_saving_at_least_100(&route, 20));
}

fn cheats_saving_at_least_100(route: &Vec<Coord>, max_shortcut_distance: i64) -> i64 {
    let mut count = 0;
    for (d0, &c0) in route.iter().enumerate() {
        for (route_distance, &c1) in route[d0..].iter().enumerate() {
            let shortcut_distance = manhattan_distance(c0, c1);
            if shortcut_distance <= max_shortcut_distance && route_distance as i64 - shortcut_distance >= 100 {
                count += 1;
            }
        }
    }
    count
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