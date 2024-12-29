use std::collections::{HashMap, HashSet};
use regex::Regex;
use aocutil::coord::Coord;
use aocutil::grid::Grid;

const DAY: u8 = 14;
const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn main() {
    let input = aocutil::load_input(DAY);
    let robots = parse_input(&input);

    let part1 = quadrant_score_after(&robots, 100);
    println!("Part 1: {part1}");

    let max_interesting_ticks = (0..WIDTH * HEIGHT)
        .max_by_key(|&i| interesting_score_after(&robots, i))
        .unwrap();
    println!("Part 2: {max_interesting_ticks}");
    print(&robot_positions_after(&robots, max_interesting_ticks));
}

fn interesting_score_after(start: &[(Coord, Coord)], ticks: i64) -> usize {
    let robots = robot_positions_after(start, ticks);
    let coords: HashSet<_> = robots.iter().collect();
    robots.iter()
        .filter(|&(x, y)| coords.contains(&(x+1, *y)) )
        .count()
}

fn quadrant_score_after(start: &[(Coord, Coord)], ticks: i64) -> i64 {
    let coords = robot_positions_after(start, ticks);
    quadrant_score(&coords)
}

fn robot_positions_after(start: &[(Coord, Coord)], ticks: i64) -> Vec<Coord> {
    start.iter()
        .map(|&(p, v)| robot_position_after(p, v, ticks))
        .collect()
}

fn robot_position_after(p: Coord, v: Coord, ticks: i64) -> Coord {
    ((p.0 + v.0 * ticks).rem_euclid(WIDTH), (p.1 + v.1 * ticks).rem_euclid(HEIGHT))
}

fn quadrant_score(robots: &[Coord]) -> i64 {
    let qs: HashMap<(bool, bool), i64> = robots.iter()
        .filter_map(|&c| quadrant(c))
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    qs.values().product::<i64>()
}

fn quadrant(c: Coord) -> Option<(bool, bool)> {
    if c.0 == WIDTH / 2 || c.1 == HEIGHT / 2 {
        None
    } else {
        Some(( c.0 > WIDTH / 2, c.1 > HEIGHT / 2))
    }
}

fn print(robots: &[Coord]) {
    let grid = Grid::new_with_coords(robots.iter(), '*');
    grid.print();
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
