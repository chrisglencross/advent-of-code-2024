use aocutil::grid::Grid;
use itertools;
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

fn is_compatible(lock: &Vec<i64>, key: &Vec<i64>) -> bool {
    lock.iter().zip(key).all(|(a, b)| a + b <= 5)
}

fn parse_input(input: &str) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let grids: Vec<Grid> = input.split("\n\n")
        .map(|b| Grid::parse(b))
        .collect();

    let locks = grids.iter()
        .filter(|grid| is_lock(grid))
        .map(|grid|  (0..grid.get_width())
            .map(|x| (0..grid.get_height())
                .filter(|&y| grid.get((x, y)).unwrap() == '#')
                .max().unwrap())
            .collect())
        .collect();

    let keys = grids.iter()
        .filter(|grid| !is_lock(grid))
        .map(|grid| (0..grid.get_width())
            .map(|x| (0..grid.get_height())
                .filter(|&y| grid.get((x, grid.get_height() - y - 1)).unwrap() == '#')
                .max().unwrap())
            .collect())
        .collect();

    (locks, keys)
}

fn is_lock(grid: &Grid) -> bool {
    (0..grid.get_width()).all(|x| grid.get((x, 0)).unwrap() == '#')
}