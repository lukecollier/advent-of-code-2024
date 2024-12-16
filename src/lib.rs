pub mod template;

use std::{
    fmt::{Debug, Display, Write},
    str::FromStr,
};

pub struct XYWorld<A> {
    world: Vec<Vec<A>>,
    pub height: usize,
    pub width: usize,
}

impl<A: Display> Display for XYWorld<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", *self.get(x, y).unwrap())?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<A: Display> Debug for XYWorld<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl<A> XYWorld<A> {
    pub fn contains(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
    }

    pub fn is_outside(&self, x: isize, y: isize) -> bool {
        x < 0 || y < 0 || x > self.width as isize || y > self.height as isize
    }

    pub fn find_first(&self, ch: &A) -> Option<(usize, usize)>
    where
        A: PartialEq + Eq,
    {
        for (y, row) in self.world.iter().enumerate() {
            for (x, el) in row.iter().enumerate() {
                if el == ch {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn draw_points(&mut self, path: Vec<(usize, usize)>, value: A)
    where
        A: Copy,
    {
        for (x, y) in path {
            if let Some(row) = self.world.get_mut(y) {
                if let Some(ch) = row.get_mut(x) {
                    *ch = value;
                }
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&A> {
        self.world.get(y).and_then(|column| column.get(x))
    }

    pub fn get_unsafe(&self, x: usize, y: usize) -> &A {
        &self.world[y][x]
    }

    pub fn update_unsafe(&mut self, x: usize, y: usize, ch: A) {
        self.world[y][x] = ch;
    }

    pub fn blank(width: usize, height: usize) -> XYWorld<char> {
        let world = vec![vec!['.'; width]; height];
        XYWorld {
            world,
            height,
            width,
        }
    }

    pub fn from_str<B: FromStr>(str: &str) -> XYWorld<B> {
        let mut world: Vec<Vec<B>> = vec![];
        let mut width = 0;
        for line in str.lines() {
            let chars = line
                .chars()
                .map(|ch| ch.to_string())
                .filter_map(|ch| ch.parse::<B>().ok())
                .collect::<Vec<B>>();
            width = chars.len();
            world.push(chars);
        }
        let height = world.len();
        XYWorld {
            world,
            height,
            width,
        }
    }
}

// Use this file to add helper functions and additional modules.
