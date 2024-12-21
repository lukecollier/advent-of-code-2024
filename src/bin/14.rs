use std::collections::{HashMap, HashSet};

use advent_of_code::Rect;

advent_of_code::solution!(14);

#[derive(Debug)]
pub struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Robot {
    pub fn tick(&mut self, width: isize, height: isize) {
        let mut new_x = self.pos.0 + self.vel.0;
        let mut new_y = self.pos.1 + self.vel.1;
        if new_x < 0 {
            new_x = width + new_x;
        }
        if new_y < 0 {
            new_y = height + new_y;
        }

        if new_x >= width {
            new_x -= width;
        }
        if new_y >= height {
            new_y -= height;
        }

        self.pos.0 = new_x;
        self.pos.1 = new_y;
    }
}

pub fn parse(input: &str) -> Vec<Robot> {
    let mut robots = vec![];
    for line in input.lines() {
        if let Some((left, right)) = line.split_once(' ') {
            if let (Some((x_str, y_str)), Some((dx_str, dy_str))) = (
                left.strip_prefix("p=")
                    .and_then(|p_str| p_str.split_once(",")),
                right
                    .strip_prefix("v=")
                    .and_then(|v_str| v_str.split_once(",")),
            ) {
                let x = x_str.parse::<isize>().unwrap();
                let y = y_str.parse::<isize>().unwrap();
                let dx = dx_str.parse::<isize>().unwrap();
                let dy = dy_str.parse::<isize>().unwrap();
                let robot = Robot {
                    pos: (x, y),
                    vel: (dx, dy),
                };
                robots.push(robot);
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }
    robots
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots = parse(input);
    #[cfg(test)]
    let dimensions = (11, 7);
    #[cfg(not(test))]
    let dimensions = (101, 103);

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.tick(dimensions.0, dimensions.1);
        }
    }
    let top_right = Rect::new((0, 0), (dimensions.0 / 2 - 1, dimensions.1 / 2 - 1));
    let top_left = Rect::new(
        (dimensions.0 / 2 + 1, 0),
        (dimensions.0, dimensions.1 / 2 - 1),
    );
    let bot_left = Rect::new(
        (0, dimensions.1 / 2 + 1),
        (dimensions.0 / 2 - 1, dimensions.1 - 1),
    );
    let bot_right = Rect::new(
        (dimensions.0 / 2 + 1, dimensions.1 / 2 + 1),
        (dimensions.0, dimensions.1 - 1),
    );
    let mut top_left_quadrant = 0;
    let mut top_right_quadrant = 0;
    let mut bot_left_quadrant = 0;
    let mut bot_right_quadrant = 0;
    for robot in &robots {
        let (x, y) = robot.pos;
        if top_left.includes(x, y) {
            top_left_quadrant += 1;
        }
        if top_right.includes(x, y) {
            top_right_quadrant += 1;
        }
        if bot_left.includes(x, y) {
            bot_left_quadrant += 1;
        }
        if bot_right.includes(x, y) {
            bot_right_quadrant += 1;
        }
    }
    Some(top_left_quadrant * top_right_quadrant * bot_left_quadrant * bot_right_quadrant)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = parse(input);
    #[cfg(test)]
    let dimensions = (11, 7);
    #[cfg(not(test))]
    let dimensions = (101, 103);

    let mut second = 0;
    loop {
        second += 1;
        for robot in robots.iter_mut() {
            robot.tick(dimensions.0, dimensions.1);
        }
        let mut seen: HashSet<(isize, isize)> = HashSet::new();
        let mut robot_iter = robots.iter();
        let mut connected: HashMap<(isize, isize), usize> = HashMap::new();
        while let Some(robot) = robot_iter.next() {
            if !seen.contains(&robot.pos) {
                let mut agents: Vec<(isize, isize)> = vec![robot.pos];
                while let Some((x, y)) = agents.pop() {
                    if !seen.contains(&(x, y)) {
                        seen.insert((x, y));
                        *connected.entry(robot.pos).or_insert(0) += 1;
                        if let Some(_) = robots.iter().find(|other_robot| {
                            other_robot.pos.0 - 1 == x && other_robot.pos.1 == y
                        }) {
                            agents.push((x - 1, y));
                        }
                        if let Some(_) = robots.iter().find(|other_robot| {
                            other_robot.pos.0 + 1 == x && other_robot.pos.1 == y
                        }) {
                            agents.push((x + 1, y));
                        }
                        if let Some(_) = robots.iter().find(|other_robot| {
                            other_robot.pos.0 == x && other_robot.pos.1 - 1 == y
                        }) {
                            agents.push((x, y - 1));
                        }
                        if let Some(_) = robots.iter().find(|other_robot| {
                            other_robot.pos.0 == x && other_robot.pos.1 + 1 == y
                        }) {
                            agents.push((x, y + 1));
                        }
                    }
                }
            }
        }

        //todo: split robots by X coordinates, if the robot has a partner robot across the line of
        //symmetry print the result
        for y in 0..dimensions.1 {
            for x in 0..dimensions.0 {
                if robots.iter().filter(|robot| robot.pos == (x, y)).count() > 0 {
                    print!(
                        "{}",
                        robots.iter().filter(|robot| robot.pos == (x, y)).count()
                    );
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
        if connected.values().find(|connect| **connect > 40).is_some() {
            break;
        }
    }
    Some(second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
