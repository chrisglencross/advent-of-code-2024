use std::collections::HashMap;

const DAY: u8 = 22;

type DeltaPattern = (i64, i64, i64, i64);

fn main() {
    let input = aocutil::load_input(DAY);
    let values = parse_input(&input);

    println!("Part 1: {}", values.iter().map(|&n| sequence(n)[2000]).sum::<i64>());

    let total_pattern_scores = values.iter()
        .map(|&n| pattern_prices_for_monkey(n))
        .fold(HashMap::new(), |acc, m| accumulate(acc, &m));
    println!("Part 2: {}", *total_pattern_scores.values().max().unwrap());
}

fn accumulate(mut acc: HashMap<DeltaPattern, i64>, n: &HashMap<DeltaPattern, i64>) -> HashMap<DeltaPattern, i64> {
    for (&pattern, &price) in n {
        *acc.entry(pattern).or_default() += price;
    }
    acc
}

fn pattern_prices_for_monkey(seed: i64) -> HashMap<DeltaPattern, i64> {
    let mut result: HashMap<DeltaPattern, i64> = HashMap::new();
    let prices: Vec<i64> = sequence(seed).iter().map(|n| n % 10).collect();
    prices.windows(5)
        .map(|p| ((p[1] - p[0], p[2] - p[1], p[3] - p[2], p[4] - p[3]), p[4]))
        .for_each(|(pattern, price)| {
            if !result.contains_key(&pattern) {
                result.insert(pattern, price);
            }
        });
    result
}
fn sequence(seed: i64) -> Vec<i64> {
    (0..2000).fold(vec![seed], |mut v, _| { v.push(next(v[v.len()-1])); v})
}

fn next(mut n: i64) -> i64 {
    n = ((n * 64) ^ n) % 16777216;
    n = ((n / 32) ^ n) % 16777216;
    n = ((n * 2048) ^ n) % 16777216;
    n
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}