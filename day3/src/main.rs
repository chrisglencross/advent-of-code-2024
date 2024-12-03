use regex::Regex;

const DAY: u8 = 3;

fn main() {
    let input = aocutil::load_input(DAY);

    let part1 = process(&input, true);
    println!("Part 1: {part1}");

    let part2 = process(&input, false);
    println!("Part 2: {part2}");
}

fn process(input: &str, part1: bool) -> i64 {
    let re = Regex::new(r"(mul)\((\d+,+\d+)\)|(do)\(()\)|(don't)\(()\)").unwrap();

    let mut result = 0i64;
    let mut on = true;
    for (_, [op, args]) in re.captures_iter(input).map(|c| c.extract()) {
        match op {
            "do" => on = true,
            "don't" => on = false,
            "mul" => if part1 || on {
                result += args.split(",")
                    .filter_map(|n|n.parse::<i64>().ok())
                    .fold(1, |x,y|x*y)
            }
            _ => panic!("Unknown operator {}", op)
        }
    }
    result
}