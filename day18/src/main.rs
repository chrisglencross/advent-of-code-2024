use regex::Regex;
use aocutil::coord::Coord;
use aocutil::direction::{COMPASS, Directions};
use aocutil::grid::Grid;
use pathfinding::prelude::bfs;

const DAY: u8 = 18;

fn main() {
    let input = aocutil::load_input(DAY);
    let byte_coords = parse_input(&input);

    let part1_ticks = 1024;
    let start = (0, 0);
    let end = (70, 70);

    let mut grid = Grid::new_with_coords(byte_coords.iter().take(part1_ticks), '#');
    let part1= shortest_route(&grid, start, end).unwrap();
    println!("Part 1: {}", part1.len());

    let part2 = add_coords_until_no_route(&mut grid, start, end, byte_coords.clone().into_iter().skip(part1_ticks).collect());
    println!("Part 2: {},{}", part2.0, part2.1);
}

fn add_coords_until_no_route(grid: &mut Grid, start: Coord, end: Coord, remaining_coords: Vec<Coord>) -> Coord {
    let mut current_route = shortest_route(&grid, start, end).unwrap();
    for coord in remaining_coords {
        grid.set(coord, '#');
        if current_route.contains(&coord) {
            match shortest_route(&grid, (0, 0), end) {
                Some(route) => current_route = route,
                None => return coord
            }
        }
    }
    panic!()
}

fn shortest_route(grid: &Grid, start: Coord, end: Coord) -> Option<Vec<Coord>> {
    bfs(&start, |p| get_next_nodes(grid, *p), |p| *p == end)
}

fn get_next_nodes(grid: &Grid, coord: Coord) -> Vec<Coord> {
    COMPASS.directions().iter()
        .map(|d| d.step(coord))
        .filter(|&c| grid.is_in_bounds(c) && grid.get_or(c, '.') != '#')
        .collect()
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