use std::iter::zip;

use itertools;
use itertools::Itertools;

const DAY: u8 = 7;

fn main() {
    let input = aocutil::load_input(DAY);
    let data = parse_input(&input);

    println!("Part 1: {}", get_solution(&data, &vec!["+", "*"]));
    println!("Part 2: {}", get_solution(&data, &vec!["+", "*", "||"]));
}

fn get_solution(data: &Vec<(i64, Vec<i64>)>, operators: &Vec<&str>) -> i64 {
    data.iter()
        .filter(|(target, values)| can_solve(*target, values, operators))
        .map(|(target, _values)| target)
        .sum()
}

fn can_solve(target: i64, values: &Vec<i64>, operators: &Vec<&str>) -> bool {
    itertools::repeat_n(operators.iter(), values.len() - 1)
        .multi_cartesian_product()
        .any(|ops| apply_ops(ops, values) == target)
}

fn apply_ops(ops: Vec<&&str>, values: &Vec<i64>) -> i64 {
    zip(ops, values[1..].iter())
        .fold(values[0], |result, (op, value)| match *op {
            "+" => result + value,
            "*" => result * value,
            "||" => (result.to_string() + &value.to_string()).parse::<i64>().unwrap(),
            _ => panic!()
        })
}

fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input.lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(v, rest)| (
            v.parse::<i64>().unwrap(),
            rest.split_whitespace().map(|v| v.parse::<i64>().unwrap()).collect()
        ))
        .collect()
}