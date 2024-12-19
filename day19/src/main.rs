use std::collections::HashMap;

const DAY: u8 = 19;

fn main() {
    let input = aocutil::load_input(DAY);
    let (patterns, designs) = parse_input(&input);

    let mut cache = HashMap::new();
    let part1 = designs.iter()
        .filter(|d| count_possible(&patterns, d, &mut cache) > 0)
        .count();
    println!("Part 1: {part1}");

    let part2: i64 = designs.iter()
        .map(|d| count_possible(&patterns, d, &mut cache))
        .sum();
    println!("Part 2: {part2}");
}

fn count_possible<'a>(patterns: &Vec<&str>, design: &'a str, cache: &mut HashMap<&'a str, i64>) -> i64 {
    if design.is_empty() {
        1
    } else if let Some(&cached_value) = cache.get(design) {
        cached_value
    } else {
        let value: i64 = patterns.iter()
            .filter(|&pattern| design.starts_with(pattern))
            .map(|pattern| count_possible(patterns, &design[pattern.len()..], cache))
            .sum();
        cache.insert(design, value);
        value
    }
}


fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (block1, block2) = input.split_once("\n\n").unwrap();
    (block1.split(", ").collect(), block2.lines().collect())
}