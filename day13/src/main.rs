use regex::Regex;

use aocutil::coord::Coord;

const DAY: u8 = 13;

fn main() {
    let input = aocutil::load_input(DAY);
    let games = parse_input(&input);

    let part1: i64 = games.iter()
        .filter_map(|&(a, b, t)| min_score1(a, b, t))
        .sum();
    println!("Part 1: {part1}");

    let part2: i64 = games.iter()
        .filter_map(|&(a, b, (tx, ty))| min_score2(a, b, (tx + 10000000000000, ty + 10000000000000)))
        .sum();
    println!("Part 2: {part2}");
}

fn min_score1((ax, ay): Coord, (bx, by): Coord, (tx, ty): Coord) -> Option<i64> {
    (0..=100)
        .flat_map(|b| (0..=100).map(|a| (a, b)).collect::<Vec<(i64, i64)>>())
        .filter(|(a, b)| (ax * a + bx * b, ay * a + by * b) == (tx, ty))
        .map(|(a, b)| 3 * a + b)
        .min()
}

fn min_score2((ax, ay): Coord, (bx, by): Coord, (tx, ty): Coord) -> Option<i64> {
    // Gradients of lines
    let ma = (ay as f64) / (ax as f64);
    let mb = (by as f64) / (bx as f64);

    // Solve equations of lines to find intersection I
    let ix = ((ty as f64) - mb * (tx as f64)) / (ma - mb);
    let iy = ma * ix;

    // Find rounded integer approximations of a and b (numbers of button presses)
    let a = (iy / (ay as f64)).round() as i64;
    let b = (((ty as f64) - iy) / (by as f64)).round() as i64;

    // Confirm that a and b integer solutions are correct, otherwise not possible
    if (ax * a + bx * b, ay * a + by * b) == (tx, ty) {
        Some(3 * a + b)
    } else {
        None
    }
}


fn parse_input(input: &str) -> Vec<(Coord, Coord, Coord)> {
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)").unwrap();
    let mut result = vec![];
    for (_, values) in re.captures_iter(input).map(|c| c.extract::<6>()) {
        let n: Vec<i64> = values.iter().map(|s| s.parse().unwrap()).collect();
        result.push(((n[0], n[1]), (n[2], n[3]), (n[4], n[5])))
    }
    result
}