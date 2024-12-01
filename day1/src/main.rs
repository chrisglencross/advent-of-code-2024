use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use regex::Regex;

fn main() {

    let (col1, col2) = load_data("day1/input.txt");

    let part1: i64 = zip(&col1, &col2).map(|pair|(pair.0 - pair.1).abs()).sum();
    println!("Part 1: {part1}");

    let mut scores = HashMap::new();
    for c2 in &col2 {
        let score = scores.entry(*c2).or_insert(0);
        *score += c2;
    }
    let part2: i64 = col1.iter().map(|&c1|*scores.entry(c1).or_default()).sum();
    println!("Part 2: {part2}");

}

fn load_data(filename: &str) -> (Vec<i64>, Vec<i64>) {

    let input = fs::read_to_string(filename).expect("Unable to read file");

    let mut col1 = vec![];
    let mut col2 = vec![];
    let re = Regex::new(r"^(\d+) +(\d+)$").unwrap();
    for line in input.lines() {
        let (_, [c1, c2]) = re.captures(line).unwrap().extract();
        col1.push(c1.parse::<i64>().unwrap());
        col2.push(c2.parse::<i64>().unwrap());
    }
    col1.sort();
    col2.sort();
    (col1, col2)
}
