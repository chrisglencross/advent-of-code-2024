use itertools;
use itertools::Itertools;

use aocutil::coord;
use aocutil::coord::Coord;
use aocutil::grid::Grid;

const DAY: u8 = 8;

fn main() {
    let input = aocutil::load_input(DAY);
    let grid = Grid::parse(&input);

    let part1 = count_unique_locations(locations_for_mast_pair_pt1, &grid);
    println!("Part 1: {part1}");

    let part2 = count_unique_locations(locations_for_mast_pair_pt2, &grid);
    println!("Part 2: {part2}");
}

fn count_unique_locations(locations_for_mast_pair: fn(Coord, Coord, &Grid) -> Vec<Coord>,
                          grid: &Grid) -> usize {
    let masts = grid.index_repeating_cells("", ".");
    masts.values()
        .flat_map(|coords| coords.iter()
            .permutations(2)
            .map(|pair| (pair[0], pair[1]))
            .flat_map(|(&c0, &c1)| locations_for_mast_pair(c0, c1, grid))
        )
        .unique()
        .filter(|&c| grid.contains_coord(c))
        .count()
}

fn locations_for_mast_pair_pt1(a: Coord, b: Coord, _grid: &Grid) -> Vec<Coord> {
    let d = coord::sub(b, a);
    [coord::sub(a, d), coord::add(b, d)].iter()
        .map(|&c| c)
        .collect()
}

fn locations_for_mast_pair_pt2(a: Coord, b: Coord, grid: &Grid) -> Vec<Coord> {
    let d = coord::sub(b, a);
    (0..)
        .map(|i| [coord::sub(a, coord::mul(d, i)), coord::add(b, coord::mul(d, i))])
        .take_while(|cs| cs.iter().any(|&c| grid.contains_coord(c)))
        .flat_map(|cs| cs)
        .collect()
}
