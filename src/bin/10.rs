use std::collections::{HashMap, HashSet};

use advent_of_code::XYWorld;
use itertools::Itertools;

advent_of_code::solution!(10);

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug)]
struct Agent {
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    path: Vec<(usize, usize)>,
}

impl Agent {
    fn new(x: usize, y: usize, dx: isize, dy: isize) -> Agent {
        Agent {
            x,
            y,
            dx,
            dy,
            path: vec![],
        }
    }

    fn with_path(x: usize, y: usize, dx: isize, dy: isize, path: Vec<(usize, usize)>) -> Agent {
        Agent { x, y, dx, dy, path }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let world: XYWorld<usize> = XYWorld::<usize>::from_str(input);
    let mut paths: HashSet<Vec<(usize, usize)>> = HashSet::new();
    for x in 0..world.width {
        for y in 0..world.height {
            let height = world.get_unsafe(x, y);
            if *height == 0 {
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                let mut agents: Vec<Agent> = vec![];
                for (dx, dy) in DIRS {
                    let mut agent = Agent::new(x, y, dx, dy);
                    agent.path.push((x, y));
                    agents.push(agent);
                }
                while let Some(mut agent) = agents.pop() {
                    let Agent {
                        x,
                        y,
                        dx,
                        dy,
                        ref mut path,
                    } = agent;
                    if let (Some(nx), Some(ny)) =
                        (x.checked_add_signed(dx), y.checked_add_signed(dy))
                    {
                        if world.contains(nx as isize, ny as isize) && !visited.contains(&(nx, ny))
                        {
                            let current = world.get_unsafe(x, y);
                            let height = world.get_unsafe(nx, ny);
                            if height.checked_sub(*current).is_some_and(|diff| diff == 1) {
                                path.push((nx, ny));
                                visited.insert((nx, ny));
                                if path.len() == 10 {
                                    paths.insert(path.to_vec());
                                } else {
                                    for (dx, dy) in DIRS {
                                        let agent = Agent::with_path(nx, ny, dx, dy, path.clone());
                                        agents.push(agent);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Some(paths.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let world: XYWorld<usize> = XYWorld::<usize>::from_str(input);
    let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
    for x in 0..world.width {
        for y in 0..world.height {
            let height = world.get_unsafe(x, y);
            if *height == 0 {
                let mut agents: Vec<Agent> = vec![];
                for (dx, dy) in DIRS {
                    let agent = Agent::with_path(x, y, dx, dy, vec![(x, y)]);
                    agents.push(agent);
                }
                while let Some(mut agent) = agents.pop() {
                    let Agent {
                        x,
                        y,
                        dx,
                        dy,
                        ref mut path,
                    } = agent;
                    if let (Some(nx), Some(ny)) =
                        (x.checked_add_signed(dx), y.checked_add_signed(dy))
                    {
                        if world.contains(nx as isize, ny as isize) {
                            let current = world.get_unsafe(x, y);
                            let height = world.get_unsafe(nx, ny);
                            if height.checked_sub(*current).is_some_and(|diff| diff == 1) {
                                path.push((nx, ny));
                                if path.len() == 10 {
                                    paths.push(path.to_vec());
                                } else {
                                    for (dx, dy) in DIRS {
                                        let agent = Agent::with_path(nx, ny, dx, dy, path.clone());
                                        agents.push(agent);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut grouped: HashMap<(usize, usize), HashSet<Vec<(usize, usize)>>> = HashMap::new();
    for path in paths.iter() {
        let id = path.first().unwrap();

        let found = grouped.entry(*id).or_insert(HashSet::new());
        found.insert(path.to_vec());
    }
    let mut len = 0;
    for (_, v) in &grouped {
        len += v.len();
    }
    Some(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
