use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign},
};

use advent_of_code::XYWorld;
use itertools::Itertools;

advent_of_code::solution!(12);

const DIRS: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

#[derive(Debug)]
struct Region {
    perimeter: usize,
    area: usize,
}

impl Region {
    fn empty() -> Region {
        Region {
            perimeter: 0,
            area: 0,
        }
    }

    fn new(perimeter: usize, area: usize) -> Region {
        Region { perimeter, area }
    }

    fn calculate(&self) -> usize {
        self.perimeter * self.area
    }
}

impl Add for Region {
    type Output = Region;

    fn add(self, rhs: Self) -> Self::Output {
        Region {
            perimeter: self.perimeter + rhs.perimeter,
            area: self.area + rhs.area,
        }
    }
}

impl AddAssign for Region {
    fn add_assign(&mut self, rhs: Self) {
        self.perimeter += rhs.perimeter;
        self.area += rhs.area;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let world = XYWorld::<char>::from_str::<char>(input);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<(char, Region)> = Vec::new();
    for x in 0..world.width {
        for y in 0..world.height {
            if !visited.contains(&(x, y)) {
                visited.insert((x, y));
                let region_label = world.get_unsafe(x, y);
                let mut current: Vec<(usize, usize)> = vec![(x, y)];
                let mut region = Region::empty();
                while let Some((cx, cy)) = current.pop() {
                    let mut perimeter = 0;
                    for (dx, dy) in DIRS {
                        if let (Some(nx), Some(ny)) =
                            (cx.checked_add_signed(dx), cy.checked_add_signed(dy))
                        {
                            if let Some(other_region) = world.get(nx, ny) {
                                if other_region == region_label {
                                    if !visited.contains(&(nx, ny)) {
                                        current.push((nx, ny));
                                        visited.insert((nx, ny));
                                    }
                                } else {
                                    perimeter += 1;
                                }
                            } else {
                                perimeter += 1;
                            }
                        } else {
                            perimeter += 1;
                        }
                    }
                    let new_region = Region::new(perimeter, 1);
                    region += new_region;
                }
                regions.push((*region_label, region));
            }
        }
    }
    regions.iter().map(|(_, region)| region.calculate()).sum1()
}

// I was close but googled a hint, basically count corners. To do this I'll use bitmasks to
// determine if we're on a corner.
//
type Bitmask = [[bool; 2]; 2];

#[rustfmt::skip]
const TOP_LEFT: [Bitmask; 3] = [
    [
        [false, false],
        [false, true]
    ],
    [
        [true, true],
        [true, false]
    ],
    [
        [true, false],
        [false, true]
    ],
];

#[rustfmt::skip]
const TOP_RIGHT: [Bitmask; 3] = [
    [
        [false, false],
        [true, false]
    ],
    [
        [true, true],
        [false, true]
    ],
    [
        [false, true],
        [true, false]
    ],
];

#[rustfmt::skip]
const BOT_RIGHT: [Bitmask; 3] = [
    [
        [true, false],
        [false, false]
    ],
    [
        [false, true],
        [true, true]
    ],
    [
        [true, false],
        [false, true]
    ],
];

#[rustfmt::skip]
const BOT_LEFT: [Bitmask; 3] = [
    [
        [false, true],
        [false, false]
    ],
    [
        [true, false],
        [true, true]
    ],
    [
        [false, true],
        [true, false]
    ],
];

pub fn part_two(input: &str) -> Option<usize> {
    let world = XYWorld::<char>::from_str::<char>(input);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    let mut region_id: usize = 0;
    for x in 0..world.width {
        for y in 0..world.height {
            if !visited.contains(&(x, y)) {
                visited.insert((x, y));
                let region_label = world.get_unsafe(x, y);
                regions
                    .entry(format!("{}-{}", *region_label, region_id))
                    .or_insert(vec![])
                    .push((x, y));
                let mut current: Vec<(usize, usize)> = vec![(x, y)];
                while let Some((cx, cy)) = current.pop() {
                    for (dx, dy) in DIRS {
                        if let (Some(nx), Some(ny)) =
                            (cx.checked_add_signed(dx), cy.checked_add_signed(dy))
                        {
                            if let Some(other_region) = world.get(nx, ny) {
                                if other_region == region_label && !visited.contains(&(nx, ny)) {
                                    current.push((nx, ny));
                                    visited.insert((nx, ny));
                                    regions
                                        .entry(format!("{}-{}", *region_label, region_id))
                                        .or_insert(vec![])
                                        .push((nx, ny));
                                }
                            }
                        }
                    }
                }
                region_id += 1;
            }
        }
    }
    let mut result = 0;
    for (label, region) in regions {
        let mut points: Vec<(usize, usize)> = Vec::new();
        let area = region.len();
        let mut sides = 0;
        let mut world = XYWorld::<char>::blank(world.width, world.height);
        world.draw_points(region, label.chars().next().unwrap());
        for x in 0..world.width {
            for y in 0..world.height {
                let top_left =
                    world.bitmask_top_left(x as isize, y as isize, label.chars().next().unwrap());
                let top_right =
                    world.bitmask_top_right(x as isize, y as isize, label.chars().next().unwrap());
                let bot_left =
                    world.bitmask_bot_left(x as isize, y as isize, label.chars().next().unwrap());
                let bot_right =
                    world.bitmask_bot_right(x as isize, y as isize, label.chars().next().unwrap());
                if TOP_LEFT.contains(&top_left) {
                    points.push((x as usize, y as usize));
                    sides += 1;
                }
                if TOP_RIGHT.contains(&top_right) {
                    points.push((x as usize, y as usize));
                    sides += 1;
                }
                if BOT_LEFT.contains(&bot_left) {
                    points.push((x as usize, y as usize));
                    sides += 1;
                }
                if BOT_RIGHT.contains(&bot_right) {
                    points.push((x as usize, y as usize));
                    sides += 1;
                }
            }
        }
        result += area * sides;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
