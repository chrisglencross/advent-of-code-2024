use std::collections::HashSet;

const DAY: u8 = 19;

fn main() {
    let input = aocutil::load_input(DAY);
    let (patterns, designs) = parse_input(&input);

    let mut negative_cache = HashSet::new();
    let part1 = designs.iter()
        .filter(|d| is_possible(&patterns, d, &mut negative_cache))
        .count();
    println!("Part 1: {part1}");

    let part2 : i64 = 0; // TODO
    println!("Part 2: {part2}");
}

fn is_possible<'a>(patterns: &Vec<&str>, design: &'a str, negative_cache: &mut HashSet<&'a str>) -> bool {
    if negative_cache.contains(design) {
        false
    } else if design.is_empty() {
        true
    } else {
        for pattern in patterns {
            if design.starts_with(pattern) && is_possible(patterns, &design[pattern.len()..], negative_cache) {
                return true
            }
        }
        negative_cache.insert(design);
        false
    }
}


fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (block1, block2) = input.split_once("\n\n").unwrap();
    (
        block1.split(", ").collect(),
        block2.lines().collect())
}