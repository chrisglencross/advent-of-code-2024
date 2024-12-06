use std::fmt;
use crate::Coord;

#[derive(Clone, Debug)]
pub struct Direction {
    name: String,
    delta: (i64, i64),
}

impl Direction {
    fn new(name: &str, delta: (i64, i64)) -> Direction {
        Direction{name: name.to_string(), delta}
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn delta(&self) -> (i64, i64) {
        self.delta
    }
    pub fn step_from(&self, coord: &Coord) -> Coord {
        self.forward_from(coord, 1)
    }
    pub fn forward_from(&self, coord: &Coord, distance: i64) -> Coord {
        let (x, y) = self.delta;
        (coord.0 + x * distance, coord.1 + y * distance)
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
    pub n: Direction,
    pub e: Direction,
    pub s: Direction,
    pub w: Direction,
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
}

pub struct Compass8 {
    n: Direction,
    pub ne: Direction,
    e: Direction,
    se: Direction,
    s: Direction,
    sw: Direction,
    w: Direction,
    pub nw: Direction,
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
}