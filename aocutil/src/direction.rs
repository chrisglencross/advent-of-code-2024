use std::collections::HashMap;
use std::fmt;

use crate::Coord;

#[derive(Clone)]
struct DirectionDef {
    name: String,
    delta: (i64, i64),
    left: String,
    right: String,
    reverse: String,
}

impl DirectionDef {
    fn new(name: &str, delta: (i64, i64), left: &str, right: &str, reverse: &str) -> Self {
        Self {
            name: String::from(name),
            delta,
            left: String::from(left),
            right: String::from(right),
            reverse: String::from(reverse),
        }
    }
}

pub struct Directions {
    direction_defs: HashMap<String, DirectionDef>,
}

#[derive(Clone)]
pub struct Direction<'a> {
    direction_def: &'a DirectionDef,
    directions: &'a Directions,
}

impl<'a> Direction<'a> {
    pub fn name(&self) -> String {
        self.direction_def.name.clone()
    }
    pub fn step_from(&self, coord: &Coord) -> Coord {
        self.forward_from(coord, 1)
    }
    pub fn forward_from(&self, coord: &Coord, distance: i64) -> Coord {
        let (x, y) = self.direction_def.delta;
        (coord.0 + x * distance, coord.1 + y * distance)
    }
    pub fn left(&self) -> Direction {
        self.directions.parse(&self.direction_def.left)
    }
    pub fn right(&self) -> Direction {
        self.directions.parse(&self.direction_def.right)
    }
    pub fn reverse(&self) -> Direction {
        self.directions.parse(&self.direction_def.reverse)
    }
}

impl fmt::Display for Direction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.direction_def.name)
    }
}

impl fmt::Debug for Direction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Direction")
            .field("name", &self.direction_def.name)
            .field("delta", &self.direction_def.delta)
            .finish()
    }
}

impl Directions {
    pub fn parse(&self, name: &str) -> Direction {
        let direction_def = self.direction_defs.get(&String::from(name)).unwrap();
        Direction { direction_def, directions: self }
    }
    pub fn values(&self) -> Vec<Direction> {
        self.direction_defs.values()
            .map(|d| Direction { direction_def: d, directions: self })
            .collect()
    }
}


pub mod compass4 {
    use crate::direction::{DirectionDef, Directions};

    pub fn directions() -> Directions {
        Directions {
            direction_defs: vec![
                DirectionDef::new("N", (0, -1), "W", "E", "S"),
                DirectionDef::new("E", (1, 0), "N", "S", "W"),
                DirectionDef::new("S", (0, 1), "E", "W", "N"),
                DirectionDef::new("W", (-1, 0), "S", "N", "E"),
            ].iter().map(|d| (d.name.clone(), d.clone())).collect()
        }
    }
}

pub mod compass8 {
    use crate::direction::{DirectionDef, Directions};

    pub fn directions() -> Directions {
        Directions {
            direction_defs: vec![
                DirectionDef::new("N", (0, -1), "W", "E", "S"),
                DirectionDef::new("NE", (1, -1), "W", "E", "SW"),
                DirectionDef::new("E", (1, 0), "N", "S", "W"),
                DirectionDef::new("SE", (1, 1), "W", "E", "NW"),
                DirectionDef::new("S", (0, 1), "E", "W", "N"),
                DirectionDef::new("SW", (-1, 1), "W", "E", "NE"),
                DirectionDef::new("W", (-1, 0), "S", "N", "E"),
                DirectionDef::new("NW", (-1, -1), "S", "N", "SE"),
            ].iter().map(|d| (d.name.clone(), d.clone())).collect()
        }
    }
}
