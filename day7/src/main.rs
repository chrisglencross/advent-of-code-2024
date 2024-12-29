use std::iter::zip;

use itertools::Itertools;

const DAY: u8 = 7;

fn main() {
    let input = aocutil::load_input(DAY);
    let data = parse_input(&input);

    println!("Part 1: {}", sum_solutions(&data, &vec!["+", "*"]));
    println!("Part 2: {}", sum_solutions(&data, &vec!["+", "*", "||"]));
}

fn sum_solutions(data: &[(i64, Vec<i64>)], operators: &Vec<&str>) -> i64 {
    data.iter()
        .filter(|(target, values)| can_solve(*target, values, operators))
        .map(|(target, _values)| target)
        .sum()
}

fn can_solve(target: i64, values: &[i64], operators: &[&str]) -> bool {
    itertools::repeat_n(operators.iter(), values.len() - 1)
        .multi_cartesian_product()
        .any(|ops| apply_ops(ops, values) == target)
}

fn apply_ops(ops: Vec<&&str>, values: &[i64]) -> i64 {
    zip(ops, values[1..].iter())
        .fold(values[0], |result, (op, value)| match *op {
            "+" => result + value,
            "*" => result * value,
            "||" => result * 10i64.pow(value.ilog10() + 1) + value,
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