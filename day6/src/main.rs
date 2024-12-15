use std::collections::HashSet;
use aocutil::coord::Coord;
use aocutil::direction::{COMPASS, Directions};
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

fn walk_grid(grid: &Grid, start: Coord) -> (HashSet<Coord>, bool) {
    let mut direction = COMPASS.north();
    let mut location = start;
    let mut locations = HashSet::new();
    let mut states = HashSet::new();
    let mut looped = true;

    while !states.contains(&(location, direction.name())) {
        locations.insert(location);
        let next_location = direction.step(location);
        let symbol = grid.get(next_location);
        match symbol {
            Some('#') => {
                direction = COMPASS.right(direction);
            },
            Some(_) => {
                states.insert((location, direction.name()));
                location = next_location;
            },
            None => {
                looped = false;  // walked off stage
                break;
            }
        }
    }
    (locations, looped)
}
