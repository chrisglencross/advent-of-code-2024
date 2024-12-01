use std::fmt;
use std::fmt::{Debug, Display};
use crate::Coord;

pub struct DirectionScheme {
    name: String,
    values: Vec<&'static dyn Direction>,
}

impl DirectionScheme {
    pub fn parse(&self, value: &str) -> &dyn Direction {
        for &dir in &self.values {
            if dir.to_string() == value {
                return dir;
            }
        }
        panic!("Direction '{}' should be valid for scheme {}", value, self.name);
    }
    pub fn values(&self) -> &Vec<&dyn Direction> {
        &self.values
    }
}

pub trait Direction: Debug + Display + ToString {
    fn move_forward(&self, from: Coord, distance: i64) -> Coord;
    fn turn_left(&self) -> &dyn Direction;
    fn turn_right(&self) -> &dyn Direction;
    fn reverse(&self) -> &dyn Direction;
}

#[derive(Debug, PartialEq, Eq)]
pub enum UDLRDirection { U, R, D, L }

impl Display for UDLRDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Direction for UDLRDirection {
    fn move_forward(&self, from: Coord, distance: i64) -> Coord {
        match self {
            UDLRDirection::U => Coord(from.0, from.1 - distance),
            UDLRDirection::R => Coord(from.0 + distance, from.1),
            UDLRDirection::D => Coord(from.0, from.1 + distance),
            UDLRDirection::L => Coord(from.0 - distance, from.1),
        }
    }
    fn turn_left(&self) -> &dyn Direction {
        match self {
            UDLRDirection::U => &UDLRDirection::L,
            UDLRDirection::R => &UDLRDirection::U,
            UDLRDirection::D => &UDLRDirection::R,
            UDLRDirection::L => &UDLRDirection::D,
        }
    }
    fn turn_right(&self) -> &dyn Direction {
        match self {
            UDLRDirection::U => &UDLRDirection::R,
            UDLRDirection::R => &UDLRDirection::D,
            UDLRDirection::D => &UDLRDirection::L,
            UDLRDirection::L => &UDLRDirection::U,
        }
    }
    fn reverse(&self) -> &dyn Direction {
        match self {
            UDLRDirection::U => &UDLRDirection::D,
            UDLRDirection::R => &UDLRDirection::L,
            UDLRDirection::D => &UDLRDirection::U,
            UDLRDirection::L => &UDLRDirection::R,
        }
    }
}

pub fn scheme(name: &str) -> DirectionScheme {
    match name {
        "UDLR" => DirectionScheme {
            name: "UDLR".to_string(),
            values: vec![&UDLRDirection::U, &UDLRDirection::R, &UDLRDirection::D, &UDLRDirection::L],
        },
        "NSEW" => DirectionScheme {
            name: "NSEW".to_string(),
            values: vec![&NSEWDirection::N, &NSEWDirection::E, &NSEWDirection::S, &NSEWDirection::W],
        },
        _ => panic!("Direction scheme should be 'UDLR' or 'NSEW'")
    }
}

#[derive(Debug, PartialEq, Eq)]
enum NSEWDirection { N, E, S, W }

impl fmt::Display for NSEWDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Direction for NSEWDirection {
    fn move_forward(&self, from: Coord, distance: i64) -> Coord {
        match self {
            NSEWDirection::N => Coord(from.0, from.1 - distance),
            NSEWDirection::E => Coord(from.0 + distance, from.1),
            NSEWDirection::S => Coord(from.0, from.1 + distance),
            NSEWDirection::W => Coord(from.0 - distance, from.1),
        }
    }
    fn turn_left(&self) -> &dyn Direction {
        match self {
            NSEWDirection::N => &NSEWDirection::W,
            NSEWDirection::E => &NSEWDirection::N,
            NSEWDirection::S => &NSEWDirection::E,
            NSEWDirection::W => &NSEWDirection::S,
        }
    }
    fn turn_right(&self) -> &dyn Direction {
        match self {
            NSEWDirection::N => &NSEWDirection::E,
            NSEWDirection::E => &NSEWDirection::S,
            NSEWDirection::S => &NSEWDirection::W,
            NSEWDirection::W => &NSEWDirection::N,
        }
    }
    fn reverse(&self) -> &dyn Direction {
        match self {
            NSEWDirection::N => &NSEWDirection::S,
            NSEWDirection::E => &NSEWDirection::W,
            NSEWDirection::S => &NSEWDirection::N,
            NSEWDirection::W => &NSEWDirection::E,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(UDLRDirection::U.turn_left().to_string(), "L");
        assert_eq!(NSEWDirection::N.turn_left().to_string(), "W");
        assert_eq!(UDLRDirection::U.to_string(), "U");
        assert_eq!(UDLRDirection::U.move_forward(Coord(0, 0), 5), Coord(0, -5));
    }


    #[test]
    fn test_nsew_scheme() {
        assert_eq!(scheme("NSEW").parse("N").to_string(), "N");
    }
}
