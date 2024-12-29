use std::cmp::Ordering;
use std::collections::HashSet;

use itertools::Itertools;

const DAY: u8 = 5;

fn main() {
    let input = aocutil::load_input(DAY);
    let (rules, updates) = parse_input(&input);

    let mut part1 = 0;
    let mut part2 = 0;
    for update in updates {
        let sorted_update = sort_update(&update, &rules);
        let mid = sorted_update[sorted_update.len() / 2];
        if sorted_update == update {
            part1 += mid;
        } else {
            part2 += mid;
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn sort_update(update: &[i64], rules: &HashSet<(i64, i64)>) -> Vec<i64> {
    update.iter()
        .sorted_by(|&a, &b|
            if rules.contains(&(*a, *b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        )
        .copied()
        .collect()
}

fn parse_input(input: &str) -> (HashSet<(i64, i64)>, Vec<Vec<i64>>) {
    let (block1, block2) = input.split_once("\n\n").unwrap();

    let rules: HashSet<(i64, i64)> = block1.lines()
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();
            (before.parse().unwrap(), after.parse().unwrap())
        })
        .collect();

    let updates: Vec<Vec<i64>> = block2.lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}