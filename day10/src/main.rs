use itertools::Itertools;

use aocutil::coord::Coord;
use aocutil::direction::{COMPASS, Directions};
use aocutil::grid::Grid;

const DAY: u8 = 10;

fn main() {
    let input = aocutil::load_input(DAY);
    let grid = Grid::parse(&input);

    let trailheads = grid.find_cells('0');

    let part1: usize = trailheads.iter()
        .map(|&trailhead| search_summits(&grid, '0', trailhead))
        .map(|summits| summits.iter().unique().count())
        .sum();
    println!("Part 1: {part1}");

    let part2: usize = trailheads.iter()
        .map(|&trailhead| search_summits(&grid, '0', trailhead))
        .map(|summits| summits.len())
        .sum();
    println!("Part 2: {part2}");
}

fn search_summits(grid: &Grid, c: char, coord: Coord) -> Vec<Coord> {
    if c == '9' {
        vec![coord]
    } else {
        let next_char = char::from_digit(c.to_digit(10).unwrap() + 1u32, 10).unwrap();
        COMPASS.directions().iter()
            .map(|d| d.step(coord))
            .filter(|&next_coord| grid.get_or(next_coord, '.') == next_char)
            .flat_map(|next_coord| search_summits(grid, next_char, next_coord))
            .collect()
    }
}
