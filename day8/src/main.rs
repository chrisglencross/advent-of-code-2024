use std::collections::{HashMap, HashSet};

use itertools;
use itertools::Itertools;

use aocutil::coord;
use aocutil::coord::Coord;
use aocutil::grid::Grid;

const DAY: u8 = 8;

fn main() {
    let input = aocutil::load_input(DAY);
    let grid = Grid::parse(&input);

    let masts = grid.index_repeating_cells("", ".");
    let grid_coords: HashSet<Coord> = grid.all_coords().iter().map(|&c| *c).collect();

    let part1 = count_locations(&masts, locations_part_1, &grid_coords);
    println!("Part 1: {part1}");

    let part2 = count_locations(&masts, locations_part_2, &grid_coords);
    println!("Part 2: {part2}");
}

fn count_locations(masts: &HashMap<char, Vec<Coord>>,
                   antinode_locations: fn(Coord, Coord, &HashSet<Coord>) -> Vec<Coord>,
                   grid_coords: &HashSet<Coord>) -> usize {
    masts.values()
        .flat_map(|coords| coords.iter().permutations(2)
            .map(|pair| (pair[0], pair[1]))
            .flat_map(|(&c0, &c1)| antinode_locations(c0, c1, grid_coords))
        )
        .unique()
        .filter(|c| grid_coords.contains(c))
        .count()
}

fn locations_part_1(a: Coord, b: Coord, _grid_coords: &HashSet<Coord>) -> Vec<Coord> {
    let d = coord::sub(b, a);
    [coord::sub(a, d), coord::add(b, d)].iter()
        .map(|&c| c)
        .collect()
}

fn locations_part_2(a: Coord, b: Coord, grid_coords: &HashSet<Coord>) -> Vec<Coord> {
    let d = coord::sub(b, a);
    (0..)
        .map(|i| [coord::sub(a, coord::mul(d, i)), coord::add(b, coord::mul(d, i))])
        .take_while(|cs| cs.iter().any(|c| grid_coords.contains(c)))
        .flat_map(|cs| cs)
        .collect()
}
