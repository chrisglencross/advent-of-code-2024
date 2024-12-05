use std::collections::HashMap;

const DAY: u8 = 5;

fn main() {
    let input = aocutil::load_input(DAY);
    let (before_after, updates) = parse_input(&input);

    let mut part1 = 0;
    let mut part2 = 0;
    for update in updates {
        if is_update_valid(&update, &before_after)  {
            part1 += update[update.len() / 2]
        } else {
            let fixed_update = find_fixed_update(&update, &before_after);
            part2 += fixed_update[fixed_update.len() / 2];
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

/// Returns false if any page in the update has another page before it that is required after it
fn is_update_valid(update: &Vec<i64>, before_after: &HashMap<i64, Vec<i64>>) -> bool {
    !update.iter().enumerate().any(|(i, page)| {
        let before_pages = &update[0..i];
        match before_after.get(page) {
            Some(required_after) => before_pages.iter().any(|before_page| required_after.contains(before_page)),
            None => false
        }
    })
}

fn find_fixed_update(update: &Vec<i64>, before_after: &HashMap<i64, Vec<i64>>) -> Vec<i64> {
    let mut result: Vec<i64> = vec![];
    for page in update {
        for i in 0..result.len()+1 {
            let mut insert_test = result.clone();
            insert_test.insert(i, *page);
            if is_update_valid(&insert_test, before_after) {
                result.insert(i, *page);
                break;
            }
        }
    }
    result
}

fn parse_input(input: &str) -> (HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {
    let mut before_after: HashMap<i64, Vec<i64>> = HashMap::new();
    let (block1, block2) = input.split_once("\n\n").unwrap();
    for line in block1.lines() {
        let (bstr, astr) = line.split_once('|').unwrap();
        let before = bstr.parse().unwrap();
        let after = astr.parse().unwrap();
        before_after.entry(before).or_insert(vec![]).push(after);
    }
    let mut updates: Vec<Vec<i64>> = vec![];
    for line in block2.lines() {
        updates.push(line.split(',').map(|n|n.parse().unwrap()).collect());
    }
    return (before_after, updates)
}