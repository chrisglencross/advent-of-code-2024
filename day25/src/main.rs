use aocutil::grid::Grid;
use itertools::iproduct;

const DAY: u8 = 25;

fn main() {
    let input = aocutil::load_input(DAY);
    let (locks, keys) = parse_input(&input);

    let part1 = iproduct!(locks, keys)
        .filter(|(lock, key)| is_compatible(lock, key))
        .count();
    println!("Part 1: {part1}");
}

fn is_compatible(lock: &[i64], key: &[i64]) -> bool {
    lock.iter().zip(key).all(|(a, b)| a + b <= 5)
}

fn parse_input(input: &str) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let grids: Vec<Grid> = input.split("\n\n")
        .map(Grid::parse)
        .collect();

    let locks = grids.iter()
        .filter(|grid| is_lock(grid))
        .map(|grid| parse_grid(grid, true))
        .collect();

    let keys = grids.iter()
        .filter(|grid| !is_lock(grid))
        .map(|grid| parse_grid(grid, false))
        .collect();

    (locks, keys)
}

fn parse_grid(grid: &Grid, is_lock: bool) -> Vec<i64> {
    let h = grid.get_height();
    (0..grid.get_width())
        .map(|x| (0..h)
            .filter(|&y| grid.get((x, if is_lock {y} else {h-y-1})).unwrap() == '#')
            .max().unwrap())
        .collect()
}

fn is_lock(grid: &Grid) -> bool {
    (0..grid.get_width()).all(|x| grid.get((x, 0)).unwrap() == '#')
}