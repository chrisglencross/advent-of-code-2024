use std::collections::{HashMap, HashSet};

use aocutil::coord::Coord;
use aocutil::direction::{Compass4, Direction};
use aocutil::grid::Grid;

const DAY: u8 = 15;

fn main() {
    let input = aocutil::load_input(DAY);
    let (grid, directions) = parse_input(&input);

    part1(&grid, &directions);
    part2(&convert_grid_to_part2(&grid), &directions);
}

fn part1(grid: &Grid, directions: &String) {
    let compass4 = Compass4::new();
    let mut robot = grid.find_cell('@').unwrap();
    let mut boxes: HashSet<Coord> = grid.find_cells('O').into_iter().collect();

    for a in directions.chars() {
        let d = get_direction(&compass4, a);
        let robot_target = d.step(robot);

        let mut box_stack: Vec<Coord> = Vec::new();
        let mut pusher_target = robot_target;
        while boxes.contains(&pusher_target) {
            box_stack.push(pusher_target);
            pusher_target = d.step(pusher_target);
        }
        if grid.get(pusher_target).unwrap() != '#' {
            if !box_stack.is_empty() {
                boxes.remove(&robot_target);
                boxes.insert(pusher_target);
            }
            robot = robot_target;
        }
        // println!("{}", d.name());
        // print(&grid, robot, &boxes);
    }

    let part1: i64 = boxes.iter().map(|&(x, y)| x + 100 * y).sum();
    println!("Part 1: {part1}");
}


fn part2(grid: &Grid, directions: &String) {
    let compass4 = Compass4::new();
    let mut robot = grid.find_cell('@').unwrap();
    let mut left_boxes: HashSet<Coord> = grid.find_cells('[').into_iter().collect();
    let mut right_boxes: HashSet<Coord> = grid.find_cells(']').into_iter().collect();

    for a in directions.chars() {
        let d = get_direction(&compass4, a);
        let robot_target = d.step(robot);

        let mut left_boxes_to_move: Vec<Coord> = Vec::new();
        let mut right_boxes_to_move: Vec<Coord> = Vec::new();

        let mut pusher_targets = vec![robot_target];
        let mut pushed_boxes = get_pushed_boxes(&grid, &compass4, &left_boxes, &right_boxes, &pusher_targets);
        while !pushed_boxes.is_empty() {
            pusher_targets = Vec::new();
            for (left, right) in pushed_boxes {
                left_boxes_to_move.push(left);
                right_boxes_to_move.push(right);
                if d != compass4.east() {
                    pusher_targets.push(d.step(left));
                }
                if d != compass4.west() {
                    pusher_targets.push(d.step(right));
                }
            }
            pushed_boxes = get_pushed_boxes(&grid, &compass4, &left_boxes, &right_boxes, &pusher_targets);
        }
        if !pusher_targets.iter().any(|&t| grid.get_or(t, '.') == '#') {
            left_boxes_to_move.iter().for_each(|b| { left_boxes.remove(b); });
            left_boxes_to_move.iter().for_each(|&b| { left_boxes.insert(d.step(b)); });
            right_boxes_to_move.iter().for_each(|b| { right_boxes.remove(b); });
            right_boxes_to_move.iter().for_each(|&b| { right_boxes.insert(d.step(b)); });
            robot = robot_target;
        }
    }

    let part2: i64 = left_boxes.iter().map(|&(x, y)| x + 100 * y).sum();
    println!("Part 2: {part2}");
}

fn get_pushed_boxes(grid: &Grid, compass4: &Compass4, box_left: &HashSet<Coord>, box_right: &HashSet<Coord>, pusher_targets: &Vec<Coord>) -> Vec<(Coord, Coord)> {
    if pusher_targets.iter().any(|&t| grid.get_or(t, '.') == '#') {
        Vec::new()
    } else {
        pusher_targets.iter()
            .filter_map(|&t| get_pushed_box(compass4, box_left, box_right, t))
            .collect()
    }
}

fn get_pushed_box(compass4: &Compass4, box_left: &HashSet<Coord>, box_right: &HashSet<Coord>, pusher_target: Coord) -> Option<(Coord, Coord)> {
    if box_left.contains(&pusher_target) {
        Some((pusher_target, compass4.east().step(pusher_target)))
    } else if box_right.contains(&pusher_target) {
        Some((compass4.west().step(pusher_target), pusher_target))
    } else {
        None
    }
}


fn get_direction(compass4: &Compass4, a: char) -> &Direction {
    match a {
        '^' => compass4.north(),
        '>' => compass4.east(),
        'v' => compass4.south(),
        '<' => compass4.west(),
        _ => panic!("not a valid direction")
    }
}

fn parse_input(input: &str) -> (Grid, String) {
    let (block1, block2): (&str, &str) = input.split_once("\n\n").unwrap();
    (Grid::parse(block1), String::from(block2.replace("\n", "").trim()))
}

fn convert_grid_to_part2(grid: &Grid) -> Grid {
    let mut data: HashMap<Coord, char> = HashMap::new();
    let (x, y) = grid.find_cell('@').unwrap();
    data.insert((x * 2, y), '@');
    for (x, y) in grid.find_cells('#') {
        data.insert((x * 2, y), '#');
        data.insert((x * 2 + 1, y), '#');
    }
    for (x, y) in grid.find_cells('O') {
        data.insert((x * 2, y), '[');
        data.insert((x * 2 + 1, y), ']');
    }
    Grid::new_with_data(data)
}
