use std::collections::HashSet;

use advent_of_code::XYWorld;

advent_of_code::solution!(6);

fn rotate_velocity(x: isize, y: isize) -> (isize, isize) {
    match (x, y) {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let world = XYWorld::from_str(input);
    let (mut x, mut y) = world.find_first(&'^').expect("start point not found");
    let (mut v_x, mut v_y) = (0_isize, -1_isize);
    let mut path: HashSet<(usize, usize)> = HashSet::new();
    path.insert((x, y));
    loop {
        let p_x = x.checked_add_signed(v_x).unwrap();
        let p_y = y.checked_add_signed(v_y).unwrap();
        if let Some(peek_ch) = world.get(p_x, p_y) {
            if *peek_ch == '#' {
                (v_x, v_y) = rotate_velocity(v_x, v_y);
            } else {
                x = p_x as usize;
                y = p_y as usize;
                path.insert((x.clone(), y.clone()));
            }
        } else {
            break;
        }
    }
    Some(path.len() as u32)
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Guard {
    x: usize,
    y: usize,
    x_v: isize,
    y_v: isize,
}

impl Guard {
    fn new(x: usize, y: usize, x_v: isize, y_v: isize) -> Guard {
        Guard { x, y, x_v, y_v }
    }

    fn rotate(&mut self) {
        (self.x_v, self.y_v) = rotate_velocity(self.x_v, self.y_v);
    }

    fn update(&mut self) -> Option<()> {
        self.x = self.x.checked_add_signed(self.x_v)?;
        self.y = self.y.checked_add_signed(self.y_v)?;
        Some(())
    }
}

//todo to increase the speed of the algorithm we simply need to on search along the path where a
//the 90 degree position would hit another blocker.
fn find_path(world: &XYWorld) -> HashSet<(usize, usize)> {
    let (mut x, mut y) = world.find_first(&'^').expect("start point not found");
    let (mut v_x, mut v_y) = (0_isize, -1_isize);
    let mut path: HashSet<(usize, usize)> = HashSet::new();
    path.insert((x, y));
    loop {
        let p_x = x.checked_add_signed(v_x).unwrap();
        let p_y = y.checked_add_signed(v_y).unwrap();
        if let Some(peek_ch) = world.get(p_x, p_y) {
            if *peek_ch == '#' {
                (v_x, v_y) = rotate_velocity(v_x, v_y);
            } else {
                x = p_x as usize;
                y = p_y as usize;
                path.insert((x.clone(), y.clone()));
            }
        } else {
            break;
        }
    }
    return path;
}

fn has_cycle(world: &XYWorld) -> bool {
    let (x, y) = world.find_first(&'^').expect("start point not found");
    let mut guard = Guard::new(x, y, 0_isize, -1_isize);
    let mut path: Vec<Guard> = Vec::new();
    loop {
        let mut next_position = guard.clone();
        if next_position.update().is_none() {
            return false;
        }
        if path.contains(&next_position) {
            break;
        }
        if let Some(peek_ch) = world.get(next_position.x, next_position.y) {
            if *peek_ch == '#' {
                guard.rotate();
            } else {
                path.push(next_position.clone());
                guard = next_position;
            }
        } else {
            return false;
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut world = XYWorld::from_str(input);
    let mut path = find_path(&world);
    let start = world.find_first(&'^').expect("start point not found");
    path.remove(&start);
    let mut total = 0;
    for (x, y) in path {
        world.update_unsafe(x, y, '#');
        if has_cycle(&world) {
            total += 1;
        }
        world.update_unsafe(x, y, '.');
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
