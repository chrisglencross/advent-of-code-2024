use std::collections::HashMap;

const DAY: u8 = 11;

fn main() {
    let input = aocutil::load_input(DAY);
    let numbers = parse_input(&input);

    let mut cache: HashMap<(i64, i64), usize> = HashMap::new();
    println!("Part 1: {}", length_after_iterations(&numbers, 25, &mut cache));
    println!("Part 2: {}", length_after_iterations(&numbers, 75, &mut cache));
}

fn length_after_iterations(values: &Vec<i64>, iterations: i64, cache: &mut HashMap<(i64, i64), usize>) -> usize {
    values.iter()
        .map(|&value| length_after_iterations1(value, iterations - 1, cache))
        .sum()
}

fn length_after_iterations1(value: i64, iterations: i64, cache: &mut HashMap<(i64, i64), usize>) -> usize {
    if iterations == 0 {
        1
    } else {
        match cache.get(&(value, iterations)) {
            Some(&length) => length,
            None => {
                let length = length_after_iterations(&expand(value), iterations, cache);
                cache.insert((value, iterations), length);
                length
            }
        }
    }
}

fn expand(value: i64) -> Vec<i64> {
    if value == 0i64 {
        vec![1]
    } else {
        let digits = value.ilog10() + 1;
        if digits % 2 == 0 {
            let split = 10i64.pow(digits / 2);
            vec![value / split, value % split]
        } else {
            vec![value * 2024]
        }
    }
}

fn parse_input(input: &str) -> Vec<i64> {
    input.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect()
}