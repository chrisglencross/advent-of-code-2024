use std::collections::HashMap;

use aocutil::coord::Coord;
use aocutil::grid::Grid;

const DAY: u8 = 21;

fn main() {
    let input: String = aocutil::load_input(DAY);

    let part1: i64 = input.lines()
        .map(|line| numeric_value(line) * button_presses_for(line, 2))
        .sum();
    println!("Part 1: {part1}");

    let part2: i64 = input.lines()
        .map(|line| numeric_value(line) * button_presses_for(line, 25))
        .sum();
    println!("Part 2: {part2}");
}

fn numeric_value(line: &str) -> i64 {
    line.replace("A", "").parse::<i64>().unwrap()
}

fn button_presses_for(code: &str, direction_keypads: i64) -> i64 {
    let mut movements = to_map_of_movements(&code.chars().collect());
    movements = numeric_keypad_presses(&movements);
    for _i in 0..direction_keypads {
        movements = direction_keypad_presses(&movements);
    }
    return movements.values().sum();
}

fn to_map_of_movements(code: &Vec<char>) -> HashMap<(char, char), i64> {
    let mut result = HashMap::new();
    for i in 0..code.len() {
        let from = if i == 0 { 'A' } else { code[i - 1] };
        let to = code[i];
        *result.entry((from, to)).or_default() += 1;
    }
    result
}

fn numeric_keypad_presses(movements: &HashMap<(char, char), i64>) -> HashMap<(char, char), i64> {
    let keypad = Grid::parse("789\n456\n123\n.0A");
    keypad_presses(&keypad, movements)
}

fn direction_keypad_presses(movements: &HashMap<(char, char), i64>) -> HashMap<(char, char), i64> {
    let keypad = Grid::parse(".^A\n<v>");
    keypad_presses(&keypad, movements)
}

fn keypad_presses(keypad: &Grid, movements: &HashMap<(char, char), i64>) -> HashMap<(char, char), i64> {
    let mut result = HashMap::new();
    let avoid = keypad.find_cell('.').unwrap();
    for (&(from, to), count) in movements {
        let from_coord = keypad.find_cell(from).unwrap();
        let to_coord = keypad.find_cell(to).unwrap();
        let key_presses = key_presses_to_move(from_coord, to_coord, avoid);
        for ((new_from, new_to), new_count) in to_map_of_movements(&key_presses) {
            *result.entry((new_from, new_to)).or_default() += count * new_count;
        }
    }
    result
}

fn key_presses_to_move(start: Coord, end: Coord, avoid: Coord) -> Vec<char> {

    let h = {
        let distance = (start.0 - end.0).abs() as usize;
        if start.0 > end.0 { ['<'].repeat(distance) } else { ['>'].repeat(distance) }
    };
    let v = {
        let distance = (start.1 - end.1).abs() as usize;
        if start.1 > end.1 { ['^'].repeat(distance) } else { ['v'].repeat(distance) }
    };

    if v.is_empty() || h.is_empty() {
        output(&h, &v)
    } else if start.1 == avoid.1 && end.0 == avoid.0 {
        output(&v, &h)
    } else if start.0 == avoid.0 && end.1 == avoid.1 {
        output(&h, &v)
    // preference for >/^/v/< arrows in that order, so we are near the buttons to reach 'A' in the top right
    } else if h.ends_with(&vec!['>']) {
        output(&v, &h)
    } else {
        output(&h, &v)
    }
}

fn output(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    let mut result = a.clone();
    result.extend(b);
    result.push('A');
    result
}