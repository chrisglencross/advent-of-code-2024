use std::iter::zip;

const DAY: u8 = 2;

fn main() {
    let input = aocutil::load_input(DAY);
    let rows = parse_input(&input);

    let part1 = rows.iter()
        .filter(|r| is_safe(r))
        .count();
    println!("Part 1: {part1}");

    let part2 = rows.iter()
        .filter(|r| is_safe_with_dampener(r))
        .count();
    println!("Part 2: {part2}");
}

fn is_safe(row: &Vec<i64>) -> bool {
    (row.iter().is_sorted() || row.iter().rev().is_sorted()) &&
        zip(&row[0..row.len() - 1], &row[1..])
            .all(|(&n0, &n1)| (1..=3).contains(&(n1 - n0).abs()))
}

fn is_safe_with_dampener(row: &Vec<i64>) -> bool {
    is_safe(row) || (0..row.len())
        .map(|remove_index| row.iter().enumerate()
            .filter_map(|(i, r)| if i == remove_index { None } else { Some(*r) })
            .collect())
        .any(|dampened_row| is_safe(&dampened_row))
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|line| -> Vec<i64> {
        line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    }).collect()
}