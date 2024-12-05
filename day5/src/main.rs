use std::collections::HashSet;

const DAY: u8 = 5;

fn main() {
    let input = aocutil::load_input(DAY);
    let (rules, updates) = parse_input(&input);

    let mut part1 = 0;
    let mut part2 = 0;
    for update in updates {
        if is_valid_update(&update, &rules) {
            part1 += update[update.len() / 2]
        } else {
            let valid_update = create_valid_update(&update, &rules);
            part2 += valid_update[valid_update.len() / 2];
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

/// Returns false if any page in the update has another page before where a rule states it must be
/// afterward.
fn is_valid_update(update: &Vec<i64>, rules: &HashSet<(i64, i64)>) -> bool {
    !update.iter().enumerate().any(|(i, &page)|
        update[0..i].iter().any(|&before_page| rules.contains(&(page, before_page)))
    )
}

fn create_valid_update(pages: &Vec<i64>, rules: &HashSet<(i64, i64)>) -> Vec<i64> {
    let mut result: Vec<i64> = vec![];
    for &page in pages {
        for i in 0..result.len() + 1 {
            let mut test = result.clone();
            test.insert(i, page);
            if is_valid_update(&test, rules) {
                result = test;
                break;
            }
        }
    }
    result
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

    return (rules, updates);
}