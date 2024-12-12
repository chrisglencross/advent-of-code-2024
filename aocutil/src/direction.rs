use std::fmt;
use std::hash::{Hash, Hasher};
use crate::coord::Coord;

#[derive(Clone, Debug, Copy, Eq)]
pub struct Direction {
    name: &'static str,
    delta: (i64, i64),
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
    }
}

impl Hash for Direction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl Direction {
    fn new(name: &'static str, delta: (i64, i64)) -> Direction {
        Direction{name, delta}
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn delta(&self) -> (i64, i64) {
        self.delta
    }
    pub fn step(&self, coord: Coord) -> Coord {
        self.forward(coord, 1)
    }
    pub fn forward(&self, (start_x, start_y): Coord, distance: i64) -> Coord {
        let (x, y) = self.delta;
        (start_x + x * distance, start_y + y * distance)
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub trait Directions {
    fn values(&self) -> Vec<&Direction>;
    fn parse(&self, name: &str) -> &Direction {
        self.values().iter().find(|&d| d.name == name).unwrap()
    }
    fn left(&self, from: &Direction) -> &Direction;
    fn right(&self, from: &Direction) -> &Direction;
    fn reverse(&self, from: &Direction) -> &Direction;

}

pub struct Compass4 {
    n: Direction,
    e: Direction,
    s: Direction,
    w: Direction,
}

impl Directions for Compass4 {
    fn values(&self) -> Vec<&Direction> {
        vec![&self.n, &self.e, &self.s, &self.w]
    }

    fn left(&self, from: &Direction) -> &Direction {
        match from.name() {
            "N" => &self.w,
            "E" => &self.n,
            "S" => &self.e,
            "W" => &self.s,
            _ => panic!(),
        }
    }

    fn right(&self, from: &Direction) -> &Direction {
        match from.name() {
            "N" => &self.e,
            "E" => &self.s,
            "S" => &self.w,
            "W" => &self.n,
            _ => panic!(),
        }
    }

    fn reverse(&self, from: &Direction) -> &Direction {
        match from.name() {
            "N" => &self.s,
            "E" => &self.w,
            "S" => &self.n,
            "W" => &self.e,
            _ => panic!(),
        }
    }
}

impl Compass4 {
    pub fn new() -> Compass4 {
        Compass4 {
            n: Direction::new("N", (0, -1)),
            e: Direction::new("E", (1, 0)),
            s: Direction::new("S", (0, 1)),
            w: Direction::new("W", (-1, 0)),
        }
    }
    pub fn north(&self) -> &Direction {
        &self.n
    }
    pub fn east(&self) -> &Direction {
        &self.e
    }
    pub fn south(&self) -> &Direction {
        &self.s
    }
    pub fn west(&self) -> &Direction {
        &self.w
    }
}

pub struct Compass8 {
    n: Direction,
    ne: Direction,
    e: Direction,
    se: Direction,
    s: Direction,
    sw: Direction,
    w: Direction,
    nw: Direction,
}

impl Directions for Compass8 {
    fn values(&self) -> Vec<&Direction> {
        vec![&self.n, &self.ne, &self.e, &self.se, &self.s, &self.sw, &self.w, &self.nw]
    }

    fn left(&self, from: &Direction) -> &Direction {
        match from.name() {
            "N" => &self.nw,
            "NE" => &self.n,
            "E" => &self.ne,
            "SE" => &self.e,
            "S" => &self.se,
            "SW" => &self.s,
            "W" => &self.sw,
            "NW" => &self.w,
            _ => panic!(),
        }
    }

    fn right(&self, from: &Direction) -> &Direction {
        match from.name() {
            "N" => &self.ne,
            "NE" => &self.e,
            "E" => &self.se,
            "SE" => &self.s,
            "S" => &self.sw,
            "SW" => &self.w,
            "W" => &self.nw,
            "NW" => &self.n,
            _ => panic!(),
        }
    }

    fn reverse(&self, from: &Direction) -> &Direction {
        match from.name() {
            "N" => &self.s,
            "NE" => &self.sw,
            "E" => &self.w,
            "SE" => &self.nw,
            "S" => &self.n,
            "SW" => &self.ne,
            "W" => &self.e,
            "NW" => &self.se,
            _ => panic!(),
        }
    }
}

impl Compass8 {
    pub fn new() -> Compass8 {
        Compass8 {
            n: Direction::new("N", (0, -1)),
            ne: Direction::new("NE", (1, -1)),
            e: Direction::new("E", (1, 0)),
            se: Direction::new("SE", (1, 1)),
            s: Direction::new("S", (0, 1)),
            sw: Direction::new("SW", (-1, 1)),
            w: Direction::new("W", (-1, 0)),
            nw: Direction::new("NW", (-1, -1)),
        }
    }
    pub fn north(&self) -> &Direction {
        &self.n
    }
    pub fn northeast(&self) -> &Direction {
        &self.ne
    }
    pub fn east(&self) -> &Direction {
        &self.e
    }
    pub fn southeast(&self) -> &Direction {
        &self.se
    }
    pub fn south(&self) -> &Direction {
        &self.s
    }
    pub fn southwest(&self) -> &Direction {
        &self.sw
    }
    pub fn west(&self) -> &Direction {
        &self.w
    }
    pub fn northwest(&self) -> &Direction {
        &self.nw
    }
}