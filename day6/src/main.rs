use std::collections::HashSet;
use aocutil::Coord;
use aocutil::direction::{Compass4, Directions};
use aocutil::grid::Grid;

const DAY: u8 = 6;

fn main() {
    let input = aocutil::load_input(DAY);
    let grid = Grid::parse(&input);
    let start = grid.find_cell('^').unwrap();

    let (visited, _) = walk_grid(&grid, start);
    println!("Part 1: {}", visited.len());

    let part2 = visited.iter().filter(|&candidate| {
        let mut copy_grid = grid.clone();
        copy_grid.set(*candidate, '#');
        let (_, is_loop) = walk_grid(&copy_grid, start);
        is_loop
    }).count();
    println!("Part 2: {part2}");
}

fn walk_grid(grid: &Grid, start: Coord) -> (Vec<Coord>, bool) {
    let mut guard = start;
    let directions = Compass4::new();
    let mut direction = directions.parse("N");
    let mut visited = Vec::new();
    let mut states = HashSet::new();
    let mut looped = true;

    while !states.contains(&(guard, direction.name())) {
        visited.push(guard);
        let next = direction.step_from(&guard);
        let cell = grid.get(&next);
        match cell {
            Some('#') => {
                direction = directions.right(direction);
            },
            Some(_) => {
                states.insert((guard, direction.name()));
                guard = next;
            },
            None => {
                looped = false;  // walked off stage
                break;
            }
        }
    }
    (visited, looped)
}
