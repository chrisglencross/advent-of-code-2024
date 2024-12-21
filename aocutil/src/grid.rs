use std::{fmt, fs};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use crate::coord::Coord;

#[derive(Clone)]
pub struct Grid {
    data: HashMap<Coord, char>,
    bounds: RefCell<Option<(Coord, Coord)>>
}

impl Grid {
    pub fn new() -> Grid {
        Grid { data: HashMap::new(), bounds: RefCell::new(None) }
    }
    pub fn new_with_data(data: HashMap<Coord, char>) -> Grid {
        Grid { data, bounds: RefCell::new(None) }
    }
    pub fn new_with_coords<'a, T: Iterator<Item=&'a Coord>>(data: T, c: char) -> Grid {
        Self::new_with_data(data.map(|&coord| (coord, c)).collect())
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
                data.insert((x as i64, y as i64), cell);
            }
        }
        Grid::new_with_data(data)
    }

    pub fn min_x(&self) -> i64 {
        let ((lo_x, _lo_y), (_hi_x, _hi_y)) = self.get_bounds();
        lo_x
    }

    pub fn max_x(&self) -> i64 {
        let ((_lo_x, _lo_y), (hi_x, _hi_y)) = self.get_bounds();
        hi_x - 1
    }

    pub fn min_y(&self) -> i64 {
        let ((_lo_x, lo_y), (_hi_x, _hi_y)) = self.get_bounds();
        lo_y
    }

    pub fn max_y(&self) -> i64 {
        let ((_lo_x, _lo_y), (_hi_x, hi_y)) = self.get_bounds();
        hi_y - 1
    }

    pub fn get_bounds(&self) -> (Coord, Coord) {
        self.get_cached_bounds()
    }

    pub fn is_in_bounds(&self, (x, y): Coord) -> bool {
        let ((lo_x, lo_y), (hi_x, hi_y)) = self.get_bounds();
        x >= lo_x && x < hi_x && y >= lo_y && y < hi_y
    }

    fn invalidate_cached_bounds(&self) {
        *self.bounds.borrow_mut() = None
    }

    fn get_cached_bounds(&self) -> (Coord, Coord){
        let mut cached_bounds = self.bounds.borrow_mut();
        match *cached_bounds {
            None => {
                let lo_x = *self.data.keys().map(|(x, _y)| x).min().unwrap_or(&0);
                let lo_y = *self.data.keys().map(|(_x, y)| y).min().unwrap_or(&0);
                let hi_x = *self.data.keys().map(|(x, _y)| x).max().unwrap_or(&(lo_x - 1)) + 1;
                let hi_y = *self.data.keys().map(|(_x, y)| y).max().unwrap_or(&(lo_y - 1)) + 1;
                let bounds = ((lo_x, lo_y), (hi_x, hi_y));
                *cached_bounds = Some(bounds);
                bounds
            }
            Some(bounds) => bounds
        }
    }

    pub fn get_width(&self) -> i64 {
        let ((lo_x, _lo_y), (hi_x, _hi_y)) = self.get_bounds();
        hi_x - lo_x
    }

    pub fn get_height(&self) -> i64 {
        let ((_lo_x, lo_y), (_hi_x, hi_y)) = self.get_bounds();
        hi_y - lo_y
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
        self.invalidate_cached_bounds();
    }

    pub fn get_or(&self, coord: Coord, default: char) -> char {
        self.get(coord).unwrap_or(default)
    }

    pub fn all_coords(&self) -> HashSet<Coord> {
        self.data.keys().map(|&c| c).collect()
    }

    pub fn contains_coord(&self, coord: Coord) -> bool {
        self.data.contains_key(&coord)
    }

    pub fn index_cells(&self, symbols: &str, not_symbols: &str) -> HashMap<char, Coord> {
        let mut result = HashMap::new();
        for (&coord, &symbol) in self.data.iter().by_ref() {
            if Self::index_symbol(symbol, symbols, not_symbols) {
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
            if Self::index_symbol(symbol, symbols, not_symbols) {
                result.entry(symbol).or_insert(vec![]).push(coord);
            }
        }
        result
    }

    fn index_symbol(symbol: char, symbols: &str, not_symbols: &str) -> bool {
        (symbols != "" && symbols.contains(symbol))
            || (not_symbols != "" && !not_symbols.contains(symbol))
            || (symbols == "" && not_symbols == "")
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ((x0, y0), (x1, y1)) = self.get_bounds();
        for y in y0..y1 {
            for x in x0..x1 {
                let c = self.get((x, y)).unwrap_or('.');
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
