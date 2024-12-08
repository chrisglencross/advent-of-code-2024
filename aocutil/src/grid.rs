use std::{fmt, fs};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use crate::coord::Coord;

#[derive(Clone)]
pub struct Grid {
    data: HashMap<Coord, char>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid { data: HashMap::new() }
    }
    pub fn new_with_data(data: HashMap<Coord, char>) -> Grid {
        Grid { data }
    }

    pub fn load(filename: &str) -> Grid {
        let content = fs::read_to_string(filename)
            .expect(&format!("Unable to read file {}", filename));

        Self::parse(&content)
    }

    pub fn parse(content: &str) -> Grid {
        let rows = content.lines();
        let mut data = HashMap::new();
        for (y, row) in rows.enumerate() {
            for (x, cell) in row.chars().enumerate() {
                data.insert((i64::try_from(x).unwrap(), i64::try_from(y).unwrap()), cell);
            }
        }
        Grid::new_with_data(data)
    }

    pub fn min_x(&self) -> i64 {
        self.data.keys().min_by_key(|(x, _y)| x).expect("grid is empty").0
    }

    pub fn max_x(&self) -> i64 {
        self.data.keys().max_by_key(|(x, _y)| x).expect("grid is empty").0
    }

    pub fn min_y(&self) -> i64 {
        self.data.keys().min_by_key(|(_x, y)| y).expect("grid is empty").1
    }

    pub fn max_y(&self) -> i64 {
        self.data.keys().max_by_key(|(_x, y)| y).expect("grid is empty").1
    }

    pub fn get_bounds(&self) -> (Coord, Coord) {
        ((self.min_x(), self.min_y()), (self.max_x() + 1, self.max_y() + 1))
    }

    pub fn get_width(&self) -> i64 {
        self.max_x() - self.min_x() + 1
    }

    pub fn get_height(&self) -> i64 {
        self.max_y() - self.min_y() + 1
    }

    pub fn get_size(&self) -> Coord {
        (self.get_width(), self.get_height())
    }

    pub fn find_cell(&self, find: char) -> Option<Coord> {
        for (&coord, &cell) in self.data.iter().by_ref() {
            if cell == find {
                return Some(coord);
            }
        }
        None
    }

    pub fn find_cells(&self, find: char) -> Vec<Coord> {
        let mut result = vec![];
        for (&coord, &cell) in self.data.iter().by_ref() {
            if cell == find {
                result.push(coord)
            }
        }
        result.sort_by_key(|&(x, y)| (y, x));
        result
    }

    pub fn get(&self, coord: Coord) -> Option<char> {
        let &c = self.data.get(&coord)?;
        Some(c)
    }

    pub fn set(&mut self, coord: Coord, c: char) {
        self.data.insert(coord, c);
    }

    pub fn get_or(&self, coord: Coord, default: char) -> char {
        self.get(coord).unwrap_or(default)
    }

    pub fn all_coords(&self) -> HashSet<Coord> {
        self.data.keys().map(|&c| c).collect()
    }

    pub fn index_cells(&self, symbols: &str, not_symbols: &str) -> HashMap<char, Coord> {
        let mut result = HashMap::new();
        for (&coord, &symbol) in self.data.iter().by_ref() {
            if (symbols != "" && symbols.contains(symbol)) || (not_symbols != "" && !not_symbols.contains(symbol)) {
                if result.insert(symbol, coord).is_some() {
                    panic!("Symbol '{}' should not appear more than once in the grid. Use 'index_repeating_cells' to fine multiple instances.", symbol);
                }
            }
        }
        result
    }

    pub fn index_repeating_cells(&self, symbols: &str, not_symbols: &str) -> HashMap<char, Vec<Coord>> {
        let mut result = HashMap::new();
        for (&coord, &symbol) in self.data.iter().by_ref() {
            if (symbols != "" && symbols.contains(symbol)) || (not_symbols != "" && !not_symbols.contains(symbol)) {
                result.entry(symbol).or_insert(vec![]).push(coord);
            }
        }
        result
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ((x0, y0), (x1, y1)) = self.get_bounds();
        for y in y0..y1 {
            for x in x0..x1 {
                let c = self.get((x, y)).unwrap_or(' ');
                f.write_char(c)?;
            }
            f.write_char('\n')?;
        }
        f.write_str("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = Grid::load("tests/testgrid.txt");
        assert_eq!(grid.min_x(), 0);
        assert_eq!(grid.max_x(), 4);
        assert_eq!(grid.min_y(), 0);
        assert_eq!(grid.max_y(), 2);

        assert_eq!(grid.find_cell('?'), None);
        assert_eq!(grid.find_cell('*'), Some((3, 1)));
        assert_eq!(grid.find_cells('g'), vec![(1, 2), (2, 2), (3, 2)]);
    }
}
