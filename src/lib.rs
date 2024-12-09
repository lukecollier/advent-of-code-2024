#![feature(iter_intersperse)]

pub mod template;

use std::fmt::{Debug, Display, Write};

use itertools::Itertools;

pub struct XYWorld {
    world: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
}

impl Display for XYWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_char(*self.get(x, y).unwrap())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Debug for XYWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl XYWorld {
    pub fn contains(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
    }

    pub fn is_outside(&self, x: isize, y: isize) -> bool {
        x < 0 || y < 0 || x > self.width as isize || y > self.height as isize
    }

    pub fn find_first(&self, ch: &char) -> Option<(usize, usize)> {
        for (y, row) in self.world.iter().enumerate() {
            for (x, el) in row.iter().enumerate() {
                if el == ch {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn draw_points(&mut self, path: Vec<(usize, usize)>, value: char) {
        for (x, y) in path {
            if let Some(row) = self.world.get_mut(y) {
                if let Some(ch) = row.get_mut(x) {
                    *ch = value;
                }
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.world.get(y).and_then(|column| column.get(x))
    }

    pub fn get_unsafe(&self, x: usize, y: usize) -> char {
        self.world[y][x]
    }

    pub fn update_unsafe(&mut self, x: usize, y: usize, ch: char) {
        self.world[y][x] = ch;
    }

    pub fn from_str(str: &str) -> XYWorld {
        let mut world: Vec<Vec<char>> = vec![];
        let mut width = 0;
        for line in str.lines() {
            let chars = line.chars().collect::<Vec<_>>();
            width = chars.len();
            world.push(chars);
        }
        XYWorld {
            world: world.clone(),
            height: world.len(),
            width,
        }
    }
}

// Use this file to add helper functions and additional modules.
