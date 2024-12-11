use std::collections::HashMap;

const DAY: u8 = 11;

fn main() {
    let input = aocutil::load_input(DAY);
    let numbers = parse_input(&input);

    let mut cache: HashMap<(u64, u32), usize> = HashMap::new();
    println!("Part 1: {}", length_after_iterations(&numbers, 25, &mut cache));
    println!("Part 2: {}", length_after_iterations(&numbers, 75, &mut cache));
}

fn length_after_iterations(values: &Vec<u64>, iterations: u32, cache: &mut HashMap<(u64, u32), usize>) -> usize {
    values.iter()
        .map(|&value| length_after_iterations1(value, iterations, cache))
        .sum()
}

fn length_after_iterations1(value: u64, iterations: u32, cache: &mut HashMap<(u64, u32), usize>) -> usize {
    if iterations == 0 {
        1
    } else {
        match cache.get(&(value, iterations)) {
            Some(&length) => length,
            None => {
                let length = length_after_iterations(&expand(value), iterations - 1, cache);
                cache.insert((value, iterations), length);
                length
            }
        }
    }
}

fn expand(value: u64) -> Vec<u64> {
    if value == 0 {
        vec![1]
    } else {
        let digits = value.ilog10() + 1;
        if digits % 2 == 0 {
            let split = 10u64.pow(digits / 2);
            vec![value / split, value % split]
        } else {
            vec![value * 2024]
        }
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect()
}