use std::collections::{HashMap, HashSet};
use regex::Regex;
use aocutil::coord::Coord;

const DAY: u8 = 14;
const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn main() {
    let input = aocutil::load_input(DAY);
    let robots = parse_input(&input);

    let part1 = quadrant_score_after(&robots, 100);
    println!("Part 1: {part1}");

    // This required a bit of experimentation to find the picture. For my test data
    // it happens to tbe the one with lowest score calculated in part 1 (robots least
    // evenly distributed across quadrants).
    // Now I know what the picture looks like, there are more reliable ways to identify it
    // e.g. maximum number of adjacent robots.
    let min_score_ticks = (0..WIDTH * HEIGHT)
        .min_by_key(|&i| quadrant_score_after(&robots, i))
        .unwrap();
    println!("Part 2: {min_score_ticks}");
    print(&robot_positions_after(&robots, min_score_ticks));
}

fn quadrant_score_after(start: &Vec<(Coord, Coord)>, ticks: i64) -> i64 {
    let coords = robot_positions_after(start, ticks);
    quadrant_score(&coords)
}

fn robot_positions_after(start: &Vec<(Coord, Coord)>, ticks: i64) -> Vec<Coord> {
    start.iter()
        .map(|&(p, v)| robot_position_after(p, v, ticks))
        .collect()
}

fn robot_position_after(p: Coord, v: Coord, ticks: i64) -> Coord {
    ((p.0 + v.0 * ticks).rem_euclid(WIDTH), (p.1 + v.1 * ticks).rem_euclid(HEIGHT))
}

fn quadrant_score(robots: &Vec<Coord>) -> i64 {
    let qs: HashMap<i64, i64> = robots.iter()
        .filter_map(|&c| quadrant(c))
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    qs.values().fold(1, |x, y| x * y)
}

fn quadrant(c: Coord) -> Option<i64> {
    let left = c.0 < WIDTH / 2;
    let right = c.0 > WIDTH / 2;
    let top = c.1 < HEIGHT / 2;
    let bottom = c.1 > HEIGHT / 2;

    if top && left { Some(0) }
    else if top && right { Some(1) }
    else if bottom && left { Some(2) }
    else if bottom && right { Some(3) }
    else { None }
}

fn print(robots: &Vec<Coord>) {
    let coords: HashSet<Coord> = robots.iter().map(|&(x, y)| (x, y)).collect();
    for y in 0..HEIGHT {
        let mut line = String::new();
        for x in 0..WIDTH {
            line += if coords.contains(&(x, y)) { "*" } else { " " };
        }
        println!("{}", line);
    }
    println!();
}

fn parse_input(input: &str) -> Vec<(Coord, Coord)> {
    let re = Regex::new(r"^p=(.+),(.+) +v=(.+),(.+)$").unwrap();
    input.lines()
        .map(|line| {
            let (_, values) = re.captures(line).unwrap().extract::<4>();
            let n: Vec<i64> = values.iter().map(|s| s.parse().unwrap()).collect();
            ((n[0], n[1]), (n[2], n[3]))
        })
        .collect()
}
