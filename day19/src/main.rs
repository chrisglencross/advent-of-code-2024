use std::collections::HashMap;

const DAY: u8 = 19;

fn main() {
    let input = aocutil::load_input(DAY);
    let (patterns, designs) = parse_input(&input);

    let mut cache = HashMap::new();
    let design_counts: Vec<i64> = designs.iter()
        .map(|design| get_design_option_counts(design, &patterns, &mut cache))
        .filter(|&c| c > 0)
        .collect();

    println!("Part 1: {}", design_counts.len());
    println!("Part 2: {}", design_counts.iter().sum::<i64>());
}

fn get_design_option_counts<'a>(design: &'a str, patterns: &Vec<&str>, cache: &mut HashMap<&'a str, i64>) -> i64 {
    if design.is_empty() {
        1
    } else if let Some(&cached_value) = cache.get(design) {
        cached_value
    } else {
        let value: i64 = patterns.iter()
            .filter(|&pattern| design.starts_with(pattern))
            .map(|pattern| get_design_option_counts(&design[pattern.len()..], patterns, cache))
            .sum();
        cache.insert(design, value);
        value
    }
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (block1, block2) = input.split_once("\n\n").unwrap();
    (block1.split(", ").collect(), block2.lines().collect())
}