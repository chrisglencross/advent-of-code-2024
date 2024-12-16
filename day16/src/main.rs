use std::collections::HashMap;

use aocutil::coord::Coord;
use aocutil::direction::{COMPASS, Direction, Directions};
use aocutil::grid::Grid;

const DAY: u8 = 16;

type Location = (Coord, &'static Direction);

fn main() {
    let input = aocutil::load_input(DAY);
    let grid = Grid::parse(&input);

    let start = grid.find_cell('S').unwrap();
    let end = grid.find_cell('E').unwrap();

    let start_state = (start, COMPASS.east());
    let mut bests: HashMap<Location, i64> = HashMap::new();
    bests.insert(start_state, 0);

    let mut result = -1;
    let mut queue: Vec<(Location, i64)> = vec![(start_state, 0)];
    loop {
        if let Some((location, score)) = queue.pop() {
            let (coord, _) = location;
            if coord == end {
                result = score;
                break;
            }
            let next_items: Vec<(Location, i64)> = next_locations(&grid, location, score)
                .into_iter()
                .filter(|(l, score)| match bests.get(l) {
                    None => true,
                    Some(prev) => score < prev
                })
                .collect();
            next_items.iter().for_each(|&(location, score)| {
                bests.insert(location, score);
            });
            queue.extend(next_items);
        } else {
            break;
        }
        queue.sort_by_key(|(location, score)| 0 - score);
    }

    let part1: i64 = result;
    println!("Part 1: {part1}");

    let part2: i64 = 0; // TODO
    println!("Part 2: {part2}");
}

fn next_locations(grid: &Grid, (coord, direction): Location, score: i64) -> Vec<(Location, i64)> {
    let mut result = vec![];
    if can_step(grid, coord, direction) {
        result.push(((direction.step(coord), direction), score + 1));
    }
    let left = COMPASS.left(direction);
    if can_step(grid, coord, left) {
        result.push(((coord, left), score + 1000));
    }
    let right = COMPASS.right(direction);
    if can_step(grid, coord, right) {
        result.push(((coord, right), score + 1000));
    }
    result
}

fn can_step(grid: &Grid, coord: Coord, direction: &Direction) -> bool {
    grid.get_or(direction.step(coord), '#') != '#'
}

