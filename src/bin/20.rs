use std::collections::{BinaryHeap, HashMap, HashSet};

use advent_of_code::BinaryGrid;

advent_of_code::solution!(20);

type Point = (isize, isize);

pub fn get_cheats(input: &str) -> HashMap<usize, usize> {
    let (mut exit_x, mut exit_y): Point = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'E' {
                (exit_x, exit_y) = (x as isize, y as isize);
            }
        }
    }
    let grid = BinaryGrid::from_str(input, |ch| ch == '#');
    let mut distance_to_exit = HashMap::<Point, usize>::new();
    let mut next_up = BinaryHeap::<Point>::new();
    let mut visited: HashSet<Point> = HashSet::new();
    distance_to_exit.insert((exit_x, exit_y), 0);
    next_up.push((exit_x, exit_y));
    while let Some((x, y)) = next_up.pop() {
        let current_distance = distance_to_exit.get(&(x, y)).unwrap().clone();
        for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let (cx, cy) = (x + dx, y + dy);
            if let Some(false) = grid.get(cx, cy) {
                let candidate_distance = current_distance + 1;
                distance_to_exit
                    .entry((cx, cy))
                    .and_modify(|distance| {
                        if *distance > candidate_distance {
                            *distance = candidate_distance;
                        }
                    })
                    .or_insert(candidate_distance);
                if visited.insert((cx, cy)) {
                    next_up.push((cx, cy));
                }
            }
        }
    }

    let mut cheat_distances = HashMap::<(isize, isize, isize, isize), usize>::new();
    for ((x, y), distance) in &distance_to_exit {
        let candidate = distance + 2;
        for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let (fx, fy) = (x + dx, y + dy);
            if grid.get(fx, fy) == Some(true) {
                let u_pos = (fx + dx, fy + dy);
                let l_pos = (fx + dy, fy + dx);
                let r_pos = (fx + -dy, fy + -dx);
                for (sx, sy) in [l_pos, u_pos, r_pos] {
                    if let Some(false) = grid.get(sx, sy) {
                        cheat_distances
                            .entry((*x, *y, sx, sy))
                            .and_modify(|dist| {
                                if candidate < *dist {
                                    *dist = candidate;
                                }
                            })
                            .or_insert(candidate);
                    }
                }
            }
        }
    }
    let mut grouped = HashMap::<usize, usize>::new();
    for ((_, _, x, y), cheat_distance) in cheat_distances {
        if let Some(shortest) = distance_to_exit.get(&(x, y)) {
            if let Some(saving) = shortest.checked_sub(cheat_distance) {
                grouped
                    .entry(saving)
                    .and_modify(|dist| {
                        *dist += 1;
                    })
                    .or_insert(1);
            }
        }
    }
    grouped.remove(&0);
    grouped
}

pub fn part_one(input: &str) -> Option<usize> {
    let cheats = get_cheats(input);

    let mut result = 0;
    for (k, freq) in &cheats {
        if *k >= 100 {
            result += freq;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut exit_x, mut exit_y): Point = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'E' {
                (exit_x, exit_y) = (x as isize, y as isize);
            }
        }
    }
    let grid = BinaryGrid::from_str(input, |ch| ch == '#');
    let mut distance_to_exit = HashMap::<Point, usize>::new();
    let mut next_up = BinaryHeap::<Point>::new();
    let mut visited: HashSet<Point> = HashSet::new();
    distance_to_exit.insert((exit_x, exit_y), 0);
    next_up.push((exit_x, exit_y));
    while let Some((x, y)) = next_up.pop() {
        let current_distance = distance_to_exit.get(&(x, y)).unwrap().clone();
        for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let (cx, cy) = (x + dx, y + dy);
            if let Some(false) = grid.get(cx, cy) {
                let candidate_distance = current_distance + 1;
                distance_to_exit
                    .entry((cx, cy))
                    .and_modify(|distance| {
                        if *distance > candidate_distance {
                            *distance = candidate_distance;
                        }
                    })
                    .or_insert(candidate_distance);
                if visited.insert((cx, cy)) {
                    next_up.push((cx, cy));
                }
            }
        }
    }

    let mut cheat_distances = HashMap::<(isize, isize, isize, isize), usize>::new();
    for ((x, y), distance) in &distance_to_exit {
        let candidate = distance + 2;
        for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            let (fx, fy) = (x + dx, y + dy);
            if grid.get(fx, fy) == Some(true) {
                // we need to find the shortest path
                let u_pos = (fx + dx, fy + dy);
                let l_pos = (fx + dy, fy + dx);
                let r_pos = (fx + -dy, fy + -dx);
                for (sx, sy) in [l_pos, u_pos, r_pos] {
                    if let Some(false) = grid.get(sx, sy) {
                        cheat_distances
                            .entry((*x, *y, sx, sy))
                            .and_modify(|dist| {
                                if candidate < *dist {
                                    *dist = candidate;
                                }
                            })
                            .or_insert(candidate);
                    }
                }
            }
        }
    }
    let mut grouped = HashMap::<usize, usize>::new();
    for ((_, _, x, y), cheat_distance) in cheat_distances {
        if let Some(shortest) = distance_to_exit.get(&(x, y)) {
            if let Some(saving) = shortest.checked_sub(cheat_distance) {
                grouped
                    .entry(saving)
                    .and_modify(|dist| {
                        *dist += 1;
                    })
                    .or_insert(1);
            }
        }
    }
    grouped.remove(&0);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = get_cheats(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.iter().fold(0_usize, |acc, (_key, v)| acc + v), 44);
        assert_eq!(result.get(&2), Some(&14));
        assert_eq!(result.get(&4), Some(&14));
        assert_eq!(result.get(&6), Some(&2));
        assert_eq!(result.get(&8), Some(&4));
        assert_eq!(result.get(&10), Some(&2));
        assert_eq!(result.get(&12), Some(&3));
        assert_eq!(result.get(&20), Some(&1));
        assert_eq!(result.get(&36), Some(&1));
        assert_eq!(result.get(&38), Some(&1));
        assert_eq!(result.get(&40), Some(&1));
        assert_eq!(result.get(&64), Some(&1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
