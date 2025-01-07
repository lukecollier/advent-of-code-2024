pub mod template;

use std::{
    fmt::{Debug, Display, Write},
    ops::Add,
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct BinaryGrid {
    world: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl BinaryGrid {
    pub fn from_str<F>(str: &str, f: F) -> BinaryGrid
    where
        F: Fn(char) -> bool,
    {
        let mut world: Vec<Vec<bool>> = Vec::new();
        let mut width = 0;
        for char in str.lines().map(|line| line.chars()) {
            let row = char.map(|ch| f(ch)).collect::<Vec<_>>();
            width = row.len();
            world.push(row);
        }
        let height = world.len();
        BinaryGrid {
            world,
            width,
            height,
        }
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut bool> {
        if x >= 0 && y >= 0 {
            let x = x as usize;
            let y = y as usize;
            if x < self.width && y < self.height {
                self.world
                    .get_mut(y as usize)
                    .and_then(|row| row.get_mut(x as usize))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<bool> {
        if x >= 0 && y >= 0 {
            let x = x as usize;
            let y = y as usize;
            if x < self.width && y < self.height {
                Some(self.world[y as usize][x as usize])
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Display for BinaryGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x as isize, y as isize).is_none_or(|res| res) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            if y < (self.height - 1) {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Rect<A> {
    bot_left: (A, A),
    top_right: (A, A),
}
impl<A> Rect<A> {
    pub fn contains(&self, x: A, y: A) -> bool
    where
        A: PartialOrd,
    {
        x < self.top_right.0 && x > self.bot_left.0 && y < self.top_right.1 && y > self.bot_left.1
    }

    pub fn includes(&self, x: A, y: A) -> bool
    where
        A: PartialOrd,
    {
        x <= self.top_right.0
            && x >= self.bot_left.0
            && y <= self.top_right.1
            && y >= self.bot_left.1
    }

    pub fn new(bot_left: (A, A), top_right: (A, A)) -> Rect<A> {
        Rect {
            bot_left,
            top_right,
        }
    }

    pub fn bounds(pos: (A, A), size: (A, A)) -> Rect<A>
    where
        A: Add<Output = A> + Clone,
    {
        let top_right = (size.0 + pos.0.clone(), size.1 + pos.1.clone());
        Rect {
            bot_left: pos,
            top_right,
        }
    }
}

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
            if y < (self.height - 1) {
                f.write_char('\n')?;
            }
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
    pub fn bitmask_top_left(&self, x: isize, y: isize, mask: A) -> [[bool; 2]; 2]
    where
        A: PartialEq,
    {
        self.neighbours_top_left(x, y)
            .map(|neighbours| neighbours.map(|v| v.is_some_and(|other| *other == mask)))
    }

    pub fn bitmask_top_right(&self, x: isize, y: isize, mask: A) -> [[bool; 2]; 2]
    where
        A: PartialEq,
    {
        self.neighbours_top_right(x, y)
            .map(|neighbours| neighbours.map(|v| v.is_some_and(|other| *other == mask)))
    }

    pub fn bitmask_bot_left(&self, x: isize, y: isize, mask: A) -> [[bool; 2]; 2]
    where
        A: PartialEq,
    {
        self.neighbours_bot_left(x, y)
            .map(|neighbours| neighbours.map(|v| v.is_some_and(|other| *other == mask)))
    }

    pub fn bitmask_bot_right(&self, x: isize, y: isize, mask: A) -> [[bool; 2]; 2]
    where
        A: PartialEq,
    {
        self.neighbours_bot_right(x, y)
            .map(|neighbours| neighbours.map(|v| v.is_some_and(|other| *other == mask)))
    }

    pub fn neighbours_top_left(&self, x: isize, y: isize) -> [[Option<&A>; 2]; 2] {
        [
            [self.get_isize(x - 1, y + 1), self.get_isize(x, y + 1)],
            [self.get_isize(x - 1, y), self.get_isize(x, y)],
        ]
    }

    pub fn neighbours_top_right(&self, x: isize, y: isize) -> [[Option<&A>; 2]; 2] {
        [
            [self.get_isize(x, y + 1), self.get_isize(x + 1, y + 1)],
            [self.get_isize(x, y), self.get_isize(x + 1, y)],
        ]
    }

    pub fn neighbours_bot_left(&self, x: isize, y: isize) -> [[Option<&A>; 2]; 2] {
        [
            [self.get_isize(x - 1, y), self.get_isize(x, y)],
            [self.get_isize(x - 1, y - 1), self.get_isize(x, y - 1)],
        ]
    }

    pub fn neighbours_bot_right(&self, x: isize, y: isize) -> [[Option<&A>; 2]; 2] {
        [
            [self.get_isize(x, y), self.get_isize(x + 1, y)],
            [self.get_isize(x, y - 1), self.get_isize(x + 1, y - 1)],
        ]
    }

    pub fn bitmask(&self, x: isize, y: isize, mask: A) -> [[bool; 3]; 3]
    where
        A: PartialEq,
    {
        self.neighbours(x, y)
            .map(|row| row.map(|v| v.is_some_and(|some| *some == mask)))
    }

    pub fn neighbours(&self, x: isize, y: isize) -> [[Option<&A>; 3]; 3] {
        [
            [
                self.get_isize(x - 1, y + 1),
                self.get_isize(x, y + 1),
                self.get_isize(x + 1, y + 1),
            ],
            [
                self.get_isize(x - 1, y),
                self.get_isize(x, y),
                self.get_isize(x + 1, y),
            ],
            [
                self.get_isize(x - 1, y - 1),
                self.get_isize(x, y - 1),
                self.get_isize(x + 1, y - 1),
            ],
        ]
    }

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

    pub fn draw_between(&mut self, from: (usize, usize), to: (usize, usize), value: A)
    where
        A: Copy,
    {
        let (lx, ly) = from;
        let (rx, ry) = to;
        for x in lx.min(rx)..=rx.max(lx) {
            self.update_unsafe(x, ry, value);
        }
        for y in ly.min(ry)..=ry.max(ly) {
            self.update_unsafe(rx, y, value);
        }
    }

    pub fn get_isize(&self, x: isize, y: isize) -> Option<&A> {
        let x_usize: usize = x.try_into().ok()?;
        let y_usize: usize = y.try_into().ok()?;
        self.get(x_usize, y_usize)
    }

    pub fn reset(&mut self, to: A)
    where
        A: Clone,
    {
        let world = vec![vec![to; self.width]; self.height];
        self.world.clear();
        self.world = world;
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut A> {
        let x_usize: usize = x.try_into().ok()?;
        let y_usize: usize = y.try_into().ok()?;
        self.world
            .get_mut(y_usize)
            .and_then(|column| column.get_mut(x_usize))
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

    pub fn grid(width: usize, height: usize) -> XYWorld<bool> {
        let world = vec![vec![false; width]; height];
        XYWorld {
            world,
            height,
            width,
        }
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
