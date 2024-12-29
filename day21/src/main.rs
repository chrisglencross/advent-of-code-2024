use std::collections::HashMap;

use aocutil::coord::Coord;
use aocutil::grid::Grid;

const DAY: u8 = 21;

fn main() {
    let input: String = aocutil::load_input(DAY);

    let part1: i64 = input.lines()
        .map(|line| numeric_value(line) * button_presses_for_code(line, 2))
        .sum();
    println!("Part 1: {part1}");

    let part2: i64 = input.lines()
        .map(|line| numeric_value(line) * button_presses_for_code(line, 25))
        .sum();
    println!("Part 2: {part2}");
}

fn numeric_value(line: &str) -> i64 {
    line.replace("A", "").parse::<i64>().unwrap()
}

fn button_presses_for_code(code: &str, directional_keypad_count: i64) -> i64 {

    let mut keypad_moves = chars_to_keypad_moves(&code.chars().collect::<Vec<char>>());

    let numerical_keypad =
        Grid::parse("789\n456\n123\n.0A").index_cells("", "");
    keypad_moves = use_keypad(&numerical_keypad, &keypad_moves);

    let directional_keypad =
        Grid::parse(".^A\n<v>").index_cells("", "");
    for _ in 0..directional_keypad_count {
        keypad_moves = use_keypad(&directional_keypad, &keypad_moves);
    }

    keypad_moves.values().sum()
}

fn chars_to_keypad_moves(code: &[char]) -> HashMap<(char, char), i64> {
    let mut result = HashMap::new();
    for (&from, &to) in ['A'].iter().chain(code.iter()).zip(code.iter()) {
        *result.entry((from, to)).or_default() += 1;
    }
    result
}

fn use_keypad(keypad: &HashMap<char, Coord>, moves_to_perform: &HashMap<(char, char), i64>) -> HashMap<(char, char), i64> {
    let mut new_keypad_moves = HashMap::new();
    let avoid = keypad.get(&'.').unwrap();
    for (&(from, to), count) in moves_to_perform {
        let from_coord = keypad.get(&from).unwrap();
        let to_coord = keypad.get(&to).unwrap();
        let button_presses = direction_buttons_to_move(from_coord, to_coord, avoid);
        for ((new_from, new_to), new_count) in chars_to_keypad_moves(&button_presses) {
            *new_keypad_moves.entry((new_from, new_to)).or_default() += count * new_count;
        }
    }
    new_keypad_moves
}

fn direction_buttons_to_move(start: &Coord, end: &Coord, avoid: &Coord) -> Vec<char> {

    let h = {
        let distance = (start.0 - end.0).unsigned_abs() as usize;
        if start.0 > end.0 { ['<'].repeat(distance) } else { ['>'].repeat(distance) }
    };
    let v = {
        let distance = (start.1 - end.1).unsigned_abs() as usize;
        if start.1 > end.1 { ['^'].repeat(distance) } else { ['v'].repeat(distance) }
    };

    // We can either move horizontally then vertically, or vertically then horizontally,
    // but we need to avoid the empty space.
    if v.is_empty() || h.is_empty() {
        output(&h, &v)
    } else if start.1 == avoid.1 && end.0 == avoid.0 {
        output(&v, &h)
    } else if start.0 == avoid.0 && end.1 == avoid.1 {
        output(&h, &v)
    // If we still have both possibilities, finish the sequence with one of '>', '^', 'v' or '<',
    // in that order of preference, so that we end up near the top-right to type the final 'A'.
    } else if h.ends_with(&['>']) {
        output(&v, &h)
    } else {
        output(&h, &v)
    }
}

fn output(a: &[char], b: &[char]) -> Vec<char> {
    let mut result = Vec::new();
    result.extend(a);
    result.extend(b);
    result.push('A');
    result
}