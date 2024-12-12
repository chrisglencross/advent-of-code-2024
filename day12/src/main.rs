use std::collections::HashSet;

use aocutil::coord::Coord;
use aocutil::direction::{Compass4, Direction, Directions};
use aocutil::grid::Grid;

const DAY: u8 = 12;

fn main() {
    let input = aocutil::load_input(DAY);
    let grid = Grid::parse(&input);
    let directions = Compass4::new();

    let mut part1 = 0;
    let mut part2 = 0;

    let mut remaining = grid.all_coords().clone();
    while !remaining.is_empty() {
        let coord = *remaining.iter().next().unwrap();
        let symbol = grid.get(coord).unwrap();

        let area = get_area(&grid, symbol, coord, &directions);
        let perimeter = get_perimeter(&grid, symbol, &area, &directions);
        let sides = count_perimeter_sides(&perimeter, &directions);

        part1 += area.len() * perimeter.len();
        part2 += area.len() * sides;

        remaining = remaining.difference(&area).cloned().collect();
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn get_area(grid: &Grid, symbol: char, coord: Coord, directions: &Compass4) -> HashSet<Coord> {
    let mut area = HashSet::new();
    collect_area(grid, symbol, coord, directions, &mut area);
    area
}

fn collect_area(grid: &Grid, symbol: char, coord: Coord, directions: &Compass4, area: &mut HashSet<Coord>) {
    if grid.get(coord) == Some(symbol) && !area.contains(&coord) {
        area.insert(coord);
        for d in directions.values() {
            collect_area(grid, symbol, d.step(coord), directions, area);
        }
    }
}

fn get_perimeter(grid: &Grid, symbol: char, area: &HashSet<Coord>, directions: &Compass4) -> HashSet<(Coord, Direction)> {
    area.iter()
        .flat_map(|&coord| directions.values().iter().map(|&direction| (coord, *direction)).collect::<Vec<(Coord, Direction)>>())
        .filter(|(coord, direction)| grid.get(direction.step(*coord)) != Some(symbol))
        .collect()
}

fn count_perimeter_sides(perimeter: &HashSet<(Coord, Direction)>, directions: &Compass4) -> usize {
    let mut sides = 0;
    let mut perimeter_remaining = perimeter.clone();
    while !perimeter_remaining.is_empty() {
        sides += 1;
        let segment = perimeter_remaining.iter().next().unwrap().clone();
        remove_side_from_perimeter(&mut perimeter_remaining, segment, directions);
    }
    sides
}

fn remove_side_from_perimeter(mut perimeter: &mut HashSet<(Coord, Direction)>, segment: (Coord, Direction), directions: &Compass4) {
    perimeter.remove(&segment);
    remove_side_from_perimeter_in_direction(&mut perimeter, segment, directions.right(&segment.1));
    remove_side_from_perimeter_in_direction(&mut perimeter, segment, directions.left(&segment.1));
}

fn remove_side_from_perimeter_in_direction(perimeter: &mut HashSet<(Coord, Direction)>, (coord, direction): (Coord, Direction), move_direction: &Direction) {
    let mut next_coord = move_direction.step(coord);
    while perimeter.remove(&(next_coord, direction)) {
        next_coord = move_direction.step(next_coord);
    }
}
