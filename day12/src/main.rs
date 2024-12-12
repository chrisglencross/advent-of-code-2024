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
        let cell = grid.get(coord).unwrap();

        let area = get_area(&grid, cell, coord, &directions);
        let perimeter = get_perimeter(&grid, cell, &area, &directions);
        let sides = count_perimeter_sides(&perimeter, &directions);

        part1 += area.len() * perimeter.len();
        part2 += area.len() * sides;

        remaining = remaining.difference(&area).cloned().collect();
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn get_area(grid: &Grid, cell: char, coord: Coord, directions: &Compass4) -> HashSet<Coord> {
    let mut area = HashSet::new();
    collect_area(grid, cell, coord, directions, &mut area);
    area
}

fn collect_area(grid: &Grid, cell: char, coord: Coord, directions: &Compass4, area: &mut HashSet<Coord>) {
    if grid.get_or(coord, '?') == cell && !area.contains(&coord) {
        area.insert(coord);
        directions.values().iter().map(|d| d.step(&coord))
            .for_each(|neighbour| collect_area(grid, cell, neighbour, directions, area));
    }
}

fn get_perimeter(grid: &Grid, cell: char, area: &HashSet<Coord>, directions: &Compass4) -> HashSet<(Coord, Direction)> {
    area.iter()
        .flat_map(|&c| directions.values().iter().map(|&d| (c, *d)).collect::<Vec<(Coord, Direction)>>())
        .filter(|(c, d): &(Coord, Direction)| grid.get_or(d.step(&c), '.') != cell)
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
    let mut next_coord = move_direction.step(&coord);
    while perimeter.remove(&(next_coord, direction)) {
        next_coord = move_direction.step(&next_coord);
    }
}
