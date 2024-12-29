use std::collections::{HashMap, HashSet};

use advent_of_code::XYWorld;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
    #[cfg(test)]
    let exit = (6, 6);
    #[cfg(test)]
    let stop_at = 12;

    #[cfg(not(test))]
    let exit = (70, 70);
    #[cfg(not(test))]
    let stop_at = 1024;

    let mut world = XYWorld::<bool>::grid(exit.0 + 1, exit.1 + 1);
    for (x, y) in input
        .lines()
        .take(stop_at)
        .filter_map(|line| line.split_once(","))
        .map(|(x_str, y_str)| {
            (
                x_str.parse::<usize>().unwrap(),
                y_str.parse::<usize>().unwrap(),
            )
        })
    {
        world.update_unsafe(x, y, true);
    }

    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut next: Vec<(usize, usize, usize)> = vec![(0, 0, 0)];
    distances.insert((0, 0), 0);
    while let Some((x, y, _)) = next.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }
        let distance = distances.get(&(x, y)).unwrap().clone();
        for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if world.get(nx as usize, ny as usize).is_some_and(|v| !*v) && !world.is_outside(nx, ny)
            {
                let other_distance = distances
                    .entry((nx as usize, ny as usize))
                    .and_modify(|other_distance| {
                        if *other_distance > distance + 1 {
                            *other_distance = distance + 1;
                        }
                    })
                    .or_insert(distance + 1);
                next.push((nx as usize, ny as usize, *other_distance));
            }
        }
        visited.insert((x, y));
        next.sort_by(|(_, _, l), (_, _, r)| r.cmp(l));
    }
    distances.get(&(exit.0, exit.1)).copied()
}

pub fn part_two(input: &str) -> Option<String> {
    #[cfg(test)]
    let exit = (6, 6);
    #[cfg(not(test))]
    let exit = (70, 70);

    let mut world = XYWorld::<bool>::grid(exit.0 + 1, exit.1 + 1);
    let mut bytes = input
        .lines()
        .filter_map(|line| line.split_once(","))
        .map(|(x_str, y_str)| {
            (
                x_str.parse::<usize>().unwrap(),
                y_str.parse::<usize>().unwrap(),
            )
        });

    let mut first_run = true;
    let mut path = XYWorld::<bool>::grid(exit.0 + 1, exit.1 + 1);
    while let Some(coordinate) = bytes.next() {
        world.update_unsafe(coordinate.0, coordinate.1, true);
        if path
            .get(coordinate.0, coordinate.1)
            .is_some_and(|value| !*value)
            && !first_run
        {
            continue;
        }
        first_run = false;
        let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut next: Vec<(usize, usize, usize)> = vec![(0, 0, 0)];
        distances.insert((0, 0), 0);
        while let Some((x, y, _)) = next.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            let distance = distances.get(&(x, y)).unwrap().clone();
            for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if world.get(nx as usize, ny as usize).is_some_and(|v| !*v)
                    && !world.is_outside(nx, ny)
                {
                    let other_distance = distances
                        .entry((nx as usize, ny as usize))
                        .and_modify(|other_distance| {
                            if *other_distance > distance + 1 {
                                *other_distance = distance + 1;
                            }
                        })
                        .or_insert(distance + 1);
                    next.push((nx as usize, ny as usize, *other_distance));
                }
            }
            visited.insert((x, y));
            next.sort_by(|(_, _, l), (_, _, r)| r.cmp(l));
        }

        let mut up_next: Vec<(usize, usize)> = vec![exit];
        path.reset(false);
        visited.clear();
        while let Some((x, y)) = up_next.pop() {
            path.update_unsafe(x, y, true);
            if visited.contains(&(x, y)) {
                continue;
            }
            if let Some(distance) = distances.get(&(x, y)) {
                for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
                    let (nx, ny) = (x as isize + dx, y as isize + dy);
                    if world.get(nx as usize, ny as usize).is_some_and(|v| !*v)
                        && !world.is_outside(nx, ny)
                    {
                        let other_distance = distances.get(&(nx as usize, ny as usize)).unwrap();
                        if *distance as isize - *other_distance as isize == 1 {
                            up_next.push((nx as usize, ny as usize));
                            break;
                        }
                    }
                }
            } else {
                return Some(format!("{},{}", coordinate.0, coordinate.1));
            }
            visited.insert((x, y));
        }
        // debug paths
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
