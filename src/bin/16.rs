use std::{
    collections::{HashMap, HashSet},
    usize,
};

use advent_of_code::XYWorld;
use itertools::Itertools;

advent_of_code::solution!(16);

fn raycast(
    world: &XYWorld<char>,
    pos: (isize, isize),
    direction: (isize, isize),
) -> Option<(isize, isize)> {
    let (mut x, mut y) = pos;
    let (dx, dy) = direction;
    let (rx, ry) = (dy, dx);
    let (lx, ly) = (rx * -1, ry * -1);
    while let Some(next) = world.get_isize(x, y) {
        // this is a dead end what do we do?
        if *next == 'E' {
            break;
        }
        if let Some(left) = world.get_isize(x + lx, y + ly) {
            if *left == '.' || *left == 'E' {
                break;
            }
        }
        if let Some(right) = world.get_isize(x + rx, y + ry) {
            if *right == '.' || *right == 'E' {
                break;
            }
        }
        if *next != '.' {
            return None;
        }
        x += dx;
        y += dy;
    }
    Some((x, y))
}

fn raycast_all(
    world: &XYWorld<char>,
    pos: (isize, isize),
    direction: (isize, isize),
) -> Vec<(isize, isize)> {
    let (mut x, mut y) = pos;
    let (dx, dy) = direction;
    let (rx, ry) = (dy, dx);
    let (lx, ly) = (rx * -1, ry * -1);
    let mut result = Vec::new();
    while let Some(next) = world.get_isize(x, y) {
        // this is a dead end what do we do?
        if *next == 'E' {
            result.push((x, y));
            return result;
        }
        if world
            .get_isize(x + rx, y + ry)
            .is_some_and(|o| *o == '.' || *o == 'E')
            || world
                .get_isize(x + lx, y + ly)
                .is_some_and(|o| *o == '.' || *o == 'E')
        {
            result.push((x, y))
        }
        if *next != '.' {
            return result;
        }
        x += dx;
        y += dy;
    }
    panic!()
}

pub fn part_one(input: &str) -> Option<usize> {
    let world = XYWorld::<char>::from_str::<char>(input);
    let (mut cx, mut cy) = (0_isize, 0_isize);
    let (mut gx, mut gy) = (0_isize, 0_isize);
    for x in 0..world.width {
        for y in 0..world.height {
            if *world.get_unsafe(x, y) == 'S' {
                (cx, cy) = (x as isize, y as isize);
            }
            if *world.get_unsafe(x, y) == 'E' {
                (gx, gy) = (x as isize, y as isize);
            }
        }
    }
    // find a best guess via simple
    let mut distances: HashMap<((isize, isize), (isize, isize)), usize> = HashMap::new();
    let mut visited: HashSet<((isize, isize), (isize, isize))> = HashSet::new();
    // unvisited nodes, the position with the direction it was found at;
    let (mut dx, mut dy) = (1_isize, 0_isize);
    // problem we don't know what angle, do we treat the direction we approach the node at as a new
    // dimension for the node distances?
    while let Some(next) = world.get_isize(cx + dx, cy + dy) {
        // I guess we need to follow each line for as long as possible from the current x
        let (rx, ry) = (dy, dx);
        let (lx, ly) = (rx * -1, ry * -1);
        if *next == '.' {
            if let Some((nx, ny)) = raycast(&world, (cx + dx, cy + dy), (dx, dy)) {
                let mut score = *distances.get(&((cx, cy), (dx, dy))).unwrap_or(&0);
                score += nx.abs_diff(cx) + ny.abs_diff(cy);
                *distances.entry(((nx, ny), (dx, dy))).or_insert(0) = score;
            }
        }
        if let Some(right) = world.get_isize(cx + rx, cy + ry) {
            // we're at a left node
            if *right == '.' {
                let mut score = *distances.get(&((cx, cy), (dx, dy))).unwrap_or(&0);
                score += 1000;
                if let Some((nx, ny)) = raycast(&world, (cx + rx, cy + ry), (rx, ry)) {
                    score += nx.abs_diff(cx) + ny.abs_diff(cy);
                    *distances.entry(((nx, ny), (rx, ry))).or_insert(0) = score;
                }
            }
        }
        if let Some(left) = world.get_isize(cx + lx, cy + ly) {
            if *left == '.' {
                let mut score = *distances.get(&((cx, cy), (dx, dy))).unwrap_or(&0);
                // we had to turn so we add the score to these paths
                score += 1000;
                if let Some((nx, ny)) = raycast(&world, (cx + lx, cy + ly), (lx, ly)) {
                    score += nx.abs_diff(cx) + ny.abs_diff(cy);
                    *distances.entry(((nx, ny), (lx, ly))).or_insert(0) = score;
                }
            }
        }
        visited.insert(((cx, cy), (dx, dy)));
        if let Some((pos, _)) = distances
            .iter()
            .filter(|(other, _)| !visited.contains(other))
            .sorted_by(|(_, value_one), (_, value_two)| value_one.cmp(value_two))
            .next()
        {
            ((cx, cy), (dx, dy)) = *pos;
        } else {
            break;
        }
    }

    distances
        .iter()
        .sorted_by(|(_, value_one), (_, value_two)| value_one.cmp(value_two))
        .find_map(|((pos, _), value)| if *pos == (gx, gy) { Some(value) } else { None })
        .copied()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut world = XYWorld::<char>::from_str::<char>(input);
    let (mut cx, mut cy) = (0_isize, 0_isize);
    let (mut ix, mut iy) = (0_isize, 0_isize);
    let (mut gx, mut gy) = (0_isize, 0_isize);
    for x in 0..world.width {
        for y in 0..world.height {
            if *world.get_unsafe(x, y) == 'S' {
                (cx, cy) = (x as isize, y as isize);
                (ix, iy) = (x as isize, y as isize);
            }
            if *world.get_unsafe(x, y) == 'E' {
                (gx, gy) = (x as isize, y as isize);
            }
        }
    }
    // find a best guess via simple
    let mut distances: HashMap<((isize, isize), (isize, isize)), usize> = HashMap::new();
    let mut connected: HashMap<(isize, isize, isize, isize), HashSet<(isize, isize, usize)>> =
        HashMap::new();
    let mut visited: HashSet<((isize, isize), (isize, isize))> = HashSet::new();
    let (mut dx, mut dy) = (1_isize, 0_isize);
    while let Some(next) = world.get_isize(cx + dx, cy + dy) {
        // I guess we need to follow each line for as long as possible from the current x
        let (rx, ry) = (dy, dx);
        let (lx, ly) = (rx * -1, ry * -1);
        if *next == '.' {
            let important_points = raycast_all(&world, (cx + dx, cy + dy), (dx, dy));
            for (nx, ny) in &important_points {
                connected
                    .entry((*nx, *ny, dx, dy))
                    .and_modify(|connected| {
                        connected.insert((cx, cy, nx.abs_diff(cx) + ny.abs_diff(cy)));
                    })
                    .or_insert(HashSet::from([(cx, cy, nx.abs_diff(cx) + ny.abs_diff(cy))]));
            }
            if let Some((nx, ny)) = important_points.first() {
                let mut score = *distances.get(&((cx, cy), (dx, dy))).unwrap_or(&0);
                score += nx.abs_diff(cx) + ny.abs_diff(cy);
                let old_score = distances
                    .get(&((*nx, *ny), (dx, dy)))
                    .unwrap_or(&usize::MAX);
                *distances.entry(((*nx, *ny), (dx, dy))).or_insert(0) = score.min(*old_score);
            }
        }
        if let Some(right) = world.get_isize(cx + rx, cy + ry) {
            if *right == '.' {
                let important_points = raycast_all(&world, (cx + rx, cy + ry), (rx, ry));
                for (nx, ny) in &important_points {
                    connected
                        .entry((*nx, *ny, rx, ry))
                        .and_modify(|connected| {
                            connected.insert((cx, cy, 1000 + nx.abs_diff(cx) + ny.abs_diff(cy)));
                        })
                        .or_insert(HashSet::from([(
                            cx,
                            cy,
                            1000 + nx.abs_diff(cx) + ny.abs_diff(cy),
                        )]));
                }
                if let Some((nx, ny)) = important_points.first() {
                    let mut score = *distances.get(&((cx, cy), (dx, dy))).unwrap_or(&0);
                    score += 1000;
                    score += nx.abs_diff(cx) + ny.abs_diff(cy);
                    let old_score = distances
                        .get(&((*nx, *ny), (rx, ry)))
                        .unwrap_or(&usize::MAX);
                    *distances.entry(((*nx, *ny), (rx, ry))).or_insert(0) = score.min(*old_score);
                }
            }
        }
        if let Some(left) = world.get_isize(cx + lx, cy + ly) {
            if *left == '.' {
                let important_points = raycast_all(&world, (cx + lx, cy + ly), (lx, ly));
                for (nx, ny) in &important_points {
                    connected
                        .entry((*nx, *ny, lx, ly))
                        .and_modify(|connected| {
                            connected.insert((cx, cy, 1000 + nx.abs_diff(cx) + ny.abs_diff(cy)));
                        })
                        .or_insert(HashSet::from([(
                            cx,
                            cy,
                            1000 + nx.abs_diff(cx) + ny.abs_diff(cy),
                        )]));
                }
                if let Some((nx, ny)) = important_points.first() {
                    let mut score = *distances.get(&((cx, cy), (dx, dy))).unwrap_or(&0);
                    score += 1000;
                    score += nx.abs_diff(cx) + ny.abs_diff(cy);
                    let old_score = distances
                        .get(&((*nx, *ny), (lx, ly)))
                        .unwrap_or(&usize::MAX);
                    *distances.entry(((*nx, *ny), (lx, ly))).or_insert(0) = score.min(*old_score);
                }
            }
        }
        visited.insert(((cx, cy), (dx, dy)));
        if let Some((pos, _)) = distances
            .iter()
            .filter(|(other, _)| !visited.contains(other))
            .sorted_by(|(_, value_one), (_, value_two)| value_one.cmp(value_two))
            .next()
        {
            ((cx, cy), (dx, dy)) = *pos;
        } else {
            break;
        }
    }
    let shortest_distance = distances
        .iter()
        .sorted_by(|(_, value_one), (_, value_two)| value_one.cmp(value_two))
        .find_map(|((pos, _), value)| if *pos == (gx, gy) { Some(value) } else { None })
        .copied()
        .unwrap();

    let mut points = distances
        .iter()
        .sorted_by(|(_, value_one), (_, value_two)| value_one.cmp(value_two))
        .filter_map(|((pos, dir), score)| {
            if *pos == (gx, gy) && *score == shortest_distance {
                Some((gx, gy, dir.0, dir.1))
            } else {
                None
            }
        })
        .collect_vec();
    let mut seen: HashSet<(isize, isize, isize, isize)> = HashSet::new();
    let mut path: XYWorld<char> = XYWorld::<char>::blank(world.width, world.height);
    while let Some((x, y, dx, dy)) = points.pop() {
        let direction = (dx, dy);
        if let Some(distance) = distances.get(&((x, y), direction)) {
            if let Some(connections) = connected.get(&(x, y, dx, dy)) {
                for (cx, cy, weight) in connections {
                    let right_dir = (dy, dx);
                    let left_dir = (-dy, -dx);
                    for direction in [direction, right_dir, left_dir] {
                        if let Some(other_distance) = distances.get(&((*cx, *cy), direction)) {
                            if (*distance as isize - *weight as isize) == *other_distance as isize {
                                path.draw_between(
                                    (x as usize, y as usize),
                                    (*cx as usize, *cy as usize),
                                    'O',
                                );
                                if seen.insert((*cx, *cy, direction.0, direction.1)) {
                                    points.push((*cx, *cy, direction.0, direction.1));
                                }
                            } else if (*distance as isize - *weight as isize) == 0 {
                                path.draw_between(
                                    (x as usize, y as usize),
                                    (*cx as usize, *cy as usize),
                                    'O',
                                );
                            }
                        } else {
                            if (*distance as isize - *weight as isize) == 0 {
                                path.draw_between(
                                    (x as usize, y as usize),
                                    (*cx as usize, *cy as usize),
                                    'O',
                                );
                            }
                        };
                    }
                }
            }
        }
    }
    let mut result = 0;
    for x in 0..world.width {
        for y in 0..world.height {
            if *path.get_unsafe(x, y) == 'O' {
                result += 1;
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
