use aocutil::Coord;
use aocutil::direction::{compass8, Direction};
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
    let word = "XMAS".to_string();
    let compass8 = compass8::directions();
    grid.all_coords().iter()
        .map(|&start|
            compass8.values().iter()
                .filter(|&direction| is_word_in_line(grid, &word, start, direction))
                .count())
        .sum()
}

fn is_word_in_line(grid: &Grid, word: &String, start: &Coord, direction: &Direction) -> bool {
    word.chars().enumerate().all(|(index, letter)| {
        let steps = i64::try_from(index).unwrap();
        let grid_cell = grid.get_or(direction.forward_from(start, steps), ' ');
        letter == grid_cell
    })
}

/// Counts 'A's in the grid which have 'M' and 'S' in both adjacent NE/SW and NE/SE directions.
fn count_x(grid: &Grid) -> usize {
    let compass8 = compass8::directions();
    let ne = compass8.parse("NE");
    let nw = compass8.parse("NW");
    grid.find_cells('A').iter()
        .filter(|&start|
            [&ne, &nw].iter().all(|d| {
                let c0 = grid.get(d.step_from(&start)).unwrap_or(' ');
                let c1 = grid.get(d.reverse().step_from(&start)).unwrap_or(' ');
                (c0 == 'M' && c1 == 'S') || (c0 == 'S' && c1 == 'M')
            })
        ).count()
}
