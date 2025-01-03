use aocutil::coord::Coord;
use aocutil::direction::{COMPASS8, Direction, Directions};
use aocutil::grid::Grid;

const DAY: u8 = 4;

fn main() {
    let input = aocutil::load_input(DAY);
    let grid = Grid::parse(&input);

    let part1 = count_xmas(&grid);
    println!("Part 1: {part1}");

    let part2 = count_x(&grid);
    println!("Part 2: {part2}");
}

fn count_xmas(grid: &Grid) -> usize {
    grid.all_coords().iter()
        .map(|&start|
            COMPASS8.directions().iter()
                .filter(|&direction| is_word_in_line(grid, "XMAS", start, direction))
                .count())
        .sum()
}

fn is_word_in_line(grid: &Grid, word: &str, start: Coord, direction: &Direction) -> bool {
    word.chars().enumerate().all(|(index, letter)| {
        let steps = index as i64;
        let grid_cell = grid.get_or(direction.forward(start, steps), ' ');
        letter == grid_cell
    })
}

/// Counts 'A's in the grid which have 'M' and 'S' in both adjacent NE/SW and NE/SE directions.
fn count_x(grid: &Grid) -> usize {
    grid.find_cells('A').iter()
        .filter(|&&start|
            [COMPASS8.northeast(), COMPASS8.northwest()].iter().all(|d| {
                let c0 = grid.get_or(d.step(start), ' ');
                let c1 = grid.get_or(COMPASS8.reverse(d).step(start), ' ');
                (c0 == 'M' && c1 == 'S') || (c0 == 'S' && c1 == 'M')
            })
        ).count()
}
