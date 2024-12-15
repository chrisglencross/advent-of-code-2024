use std::collections::HashSet;

use aocutil::coord::Coord;
use aocutil::direction::{COMPASS, Direction, Directions};
use aocutil::grid::Grid;

pub type Edge = (Coord, Direction);

const DAY: u8 = 12;

fn main() {
    let input = aocutil::load_input(DAY);
    let grid = Grid::parse(&input);

    let mut part1 = 0;
    let mut part2 = 0;

    let mut remaining_coords = grid.all_coords().clone();
    while let Some(&coord) = remaining_coords.iter().next() {
        let symbol = grid.get(coord).unwrap();

        let area_coords = get_area(&grid, symbol, coord);
        let perimeter_edges = get_perimeter(&grid, symbol, &area_coords);
        let perimeter_sides = count_perimeter_sides(&perimeter_edges);

        part1 += area_coords.len() * perimeter_edges.len();
        part2 += area_coords.len() * perimeter_sides;

        area_coords.iter().for_each(|c| { remaining_coords.remove(c); } );
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn get_area(grid: &Grid, symbol: char, coord: Coord) -> HashSet<Coord> {
    let mut area = HashSet::new();
    collect_area(grid, symbol, coord, &mut area);
    area
}

fn collect_area(grid: &Grid, symbol: char, coord: Coord, area: &mut HashSet<Coord>) {
    if grid.get(coord) == Some(symbol) && !area.contains(&coord) {
        area.insert(coord);
        for d in COMPASS.directions() {
            collect_area(grid, symbol, d.step(coord), area);
        }
    }
}

fn get_perimeter(grid: &Grid, symbol: char, area: &HashSet<Coord>) -> HashSet<Edge> {
    area.iter()
        .flat_map(|&coord| COMPASS.directions().iter().map(|&direction| (coord, *direction)).collect::<Vec<Edge>>())
        .filter(|(coord, direction)| grid.get(direction.step(*coord)) != Some(symbol))
        .collect()
}

fn count_perimeter_sides(perimeter_edges: &HashSet<Edge>) -> usize {
    let mut side_count = 0;
    let mut perimeter_edges_remaining = perimeter_edges.clone();
    while let Some(&start_edge) = perimeter_edges_remaining.iter().next() {
        let side_edges = side_edges(perimeter_edges, start_edge);
        side_edges.iter().for_each(|e| { perimeter_edges_remaining.remove(e); } );
        side_count += 1;
    }
    side_count
}

fn side_edges(perimeter: &HashSet<Edge>, start_edge: Edge) -> HashSet<Edge> {
    let (_coord, direction) = start_edge;
    vec![start_edge].into_iter()
        .chain(side_edges_one_direction(perimeter, start_edge, COMPASS.left(&direction)))
        .chain(side_edges_one_direction(perimeter, start_edge, COMPASS.right(&direction)))
        .collect()
}

fn side_edges_one_direction(perimeter: &HashSet<Edge>, start_edge: Edge, move_direction: &Direction) -> Vec<Edge> {
    let (coord, direction) = start_edge;
    (1..)
        .map(|distance| (move_direction.forward(coord, distance), direction))
        .take_while(|edge| perimeter.contains(edge))
        .collect()
}
