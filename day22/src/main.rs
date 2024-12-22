use std::collections::HashMap;

const DAY: u8 = 22;

type ChangePattern = (i64, i64, i64, i64);

fn main() {
    let input = aocutil::load_input(DAY);
    let values = parse_input(&input);

    println!("Part 1: {}", values.iter().map(|&n| sequence(n)[2000]).sum());

    let total_pattern_scores = values.iter()
        .map(|&n| pattern_scores_for_monkey(n))
        .fold(HashMap::new(), |acc, m| accumulate(acc, &m));
    println!("Part 2: {}", *total_pattern_scores.values().max().unwrap());
}

fn accumulate(mut acc: HashMap<ChangePattern, i64>, n: &HashMap<ChangePattern, i64>) -> HashMap<ChangePattern, i64> {
    for (&key, &value) in n {
        *acc.entry(key).or_default() += value;
    }
    acc
}

fn pattern_scores_for_monkey(n: i64) -> HashMap<ChangePattern, i64> {
    let prices = sequence_mod_10(n);
    let mut result: HashMap<ChangePattern, i64> = HashMap::new();
    let delta_prices: Vec<(ChangePattern, i64)> = prices.windows(5)
        .map(|w| ((w[1] - w[0], w[2] - w[1], w[3] - w[2], w[4] - w[3]), w[4]))
        .collect();

    for (key, value) in delta_prices {
        if !result.contains_key(&key) {
            result.insert(key, value);
        }
    }
    result
}

fn sequence_mod_10(n: i64) -> Vec<i64> {
    sequence(n).iter().map(|n| n % 10).collect()
}

fn sequence(mut n: i64) -> Vec<i64> {
    let mut result = Vec::with_capacity(2000);
    result.push(n);
    for _ in 0..2000 {
        n = next(n);
        result.push(n)
    }
    result
}

fn next(n: i64) -> i64 {
    let a = ((n * 64) ^ n) % 16777216;
    let b = ((a / 32) ^ a) % 16777216;
    let c = ((b * 2048) ^ b) % 16777216;
    c
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}