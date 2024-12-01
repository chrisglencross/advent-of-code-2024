use regex::Regex;

const DAY: u8 = 2;

fn main() {
    let input = aocutil::load_test_input(DAY);
    let _ = parse_input(&input);

    let part1 : i64 = 0; // TODO
    println!("Part 1: {part1}");

    let part2 : i64 = 0; // TODO
    println!("Part 2: {part2}");
}

fn parse_input(input: &str) -> () {
    let re = Regex::new(r"^(\d+) +(\d+)$").unwrap();
    for line in input.lines() {
        let (_, [c1, c2]) = re.captures(line).unwrap().extract();
    }
    ()
}