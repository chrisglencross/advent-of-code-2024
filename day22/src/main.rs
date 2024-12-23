use std::collections::HashMap;

const DAY: u8 = 22;

type DeltaPattern = (i64, i64, i64, i64);

fn main() {
    let input = aocutil::load_input(DAY);
    let values = parse_input(&input);

    let part1: i64 = values.iter()
        .flat_map(|&n| random_sequence(n).skip(2000).take(1))
        .sum();
    println!("Part 1: {}", part1);

    let total_pattern_scores = values.iter()
        .map(|&n| pattern_prices_for_monkey(n))
        .fold(HashMap::new(), |acc, m| accumulate(acc, &m));
    println!("Part 2: {}", *total_pattern_scores.values().max().unwrap());
}

fn accumulate(mut acc: HashMap<DeltaPattern, i64>, n: &HashMap<DeltaPattern, i64>) -> HashMap<DeltaPattern, i64> {
    n.iter().for_each(|(&pattern, &price)| *acc.entry(pattern).or_default() += price);
    acc
}

fn pattern_prices_for_monkey(seed: i64) -> HashMap<DeltaPattern, i64> {
    let prices: Vec<i64> = random_sequence(seed).take(2001)
        .map(|n| n % 10)
        .collect();
    prices.windows(5)
        .map(|p| ((p[1] - p[0], p[2] - p[1], p[3] - p[2], p[4] - p[3]), p[4]))
        .rev()  // include earliest prices in HashMap
        .collect()
}

fn random_sequence(seed: i64) -> impl Iterator<Item = i64> {
    std::iter::successors(
        Some(seed),
        move |&n| {
            let n = ((n * 64) ^ n) % 16777216;
            let n = ((n / 32) ^ n) % 16777216;
            let n = ((n * 2048) ^ n) % 16777216;
            Some(n)
        },
    )
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}