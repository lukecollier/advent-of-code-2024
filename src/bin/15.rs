use std::fs::File;
use std::io::Write;

use advent_of_code::XYWorld;

advent_of_code::solution!(15);

fn direction_to_vector(dir: char) -> (isize, isize) {
    match dir {
        '^' => (0, -1),
        '<' => (-1, 0),
        '>' => (1, 0),
        'v' => (0, 1),
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut buffer = String::new();
    let mut line_iter = input.lines().into_iter();
    let mut robot_pos = (0_isize, 0_isize);
    let mut y: isize = 0;
    while let Some(line) = line_iter.next() {
        if !line.is_empty() {
            if let Some((x, _)) = line.char_indices().find(|(_, ch)| *ch == '@') {
                robot_pos = (x as isize, y);
            }
            buffer.push_str(line);
            buffer.push_str("\n");
        } else {
            break;
        }
        y += 1;
    }
    let mut world = XYWorld::<char>::from_str::<char>(&buffer);
    buffer.clear();
    while let Some(line) = line_iter.next() {
        buffer.push_str(line);
    }
    for (_, (dx, dy)) in buffer.chars().map(|dir| (dir, direction_to_vector(dir))) {
        let (x, y) = robot_pos;
        let (new_x, new_y) = (x + dx, y + dy);
        if let Some(ch) = world.get_isize(new_x, new_y) {
            match ch {
                'O' => {
                    let (mut barrel_x, mut barrel_y) = (new_x + dx, new_y + dy);
                    while let Some(barrel) = world.get_isize(barrel_x, barrel_y) {
                        match barrel {
                            'O' => {
                                barrel_x += dx;
                                barrel_y += dy;
                            }
                            '.' => {
                                *world.get_mut(barrel_x, barrel_y).unwrap() = 'O';
                                *world.get_mut(x, y).unwrap() = '.';
                                *world.get_mut(new_x, new_y).unwrap() = '@';
                                robot_pos = (new_x, new_y);
                                break;
                            }
                            '#' => {
                                break;
                            }
                            _ => panic!(),
                        }
                    }
                }
                '.' => {
                    *world.get_mut(x, y).unwrap() = '.';
                    *world.get_mut(new_x, new_y).unwrap() = '@';
                    robot_pos = (new_x, new_y);
                }
                '#' => {}
                _ => panic!(),
            }
        }
    }

    let mut result = 0;
    for x in 0..world.width {
        for y in 0..world.height {
            let ch = world.get_unsafe(x, y);
            if *ch == 'O' {
                result += (y * 100) + x
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut buffer = String::new();
    let mut line_iter = input.lines().into_iter();
    let mut robot_pos = (0_isize, 0_isize);
    let mut y: isize = 0;
    while let Some(line) = line_iter.next() {
        if !line.is_empty() {
            for (x, ch) in line.char_indices() {
                match ch {
                    '@' => {
                        robot_pos = ((x as isize * 2), y);
                        buffer.push_str("@.")
                    }
                    '#' => buffer.push_str("##"),
                    'O' => buffer.push_str("[]"),
                    '.' => buffer.push_str(".."),
                    _ => panic!("{}", ch),
                }
            }
            buffer.push_str("\n");
        } else {
            break;
        }
        y += 1;
    }
    let mut world = XYWorld::<char>::from_str::<char>(&buffer);
    // for x in 0..world.width {
    //     for y in 0..world.height {
    //         let ch = world.get_unsafe(x, y);
    //         if *ch == '@' {
    //             robot_pos = (x as isize, y as isize);
    //         }
    //     }
    // }
    buffer.clear();
    while let Some(line) = line_iter.next() {
        buffer.push_str(line);
    }
    for (_, (dx, dy)) in buffer.chars().map(|dir| (dir, direction_to_vector(dir))) {
        let (x, y) = robot_pos;
        let (new_x, new_y) = (x + dx, y + dy);
        if let Some(ch) = world.get_isize(new_x, new_y) {
            match ch {
                '[' | ']' if dx == 0 => {
                    let mut selected = if *ch == '[' {
                        vec![(new_x, new_y), (new_x + 1, new_y)]
                    } else {
                        vec![(new_x - 1, new_y), (new_x, new_y)]
                    };
                    let mut boxes = if *ch == '[' {
                        vec![(new_x, new_y), (new_x + 1, new_y)]
                    } else {
                        vec![(new_x - 1, new_y), (new_x, new_y)]
                    };
                    while let Some((ox, oy)) = selected.pop() {
                        let nx = ox + dx;
                        let ny = oy + dy;
                        // check for any boxes in the way and store every position
                        if let Some(o_ch) = world.get_isize(nx, ny) {
                            match o_ch {
                                ']' => {
                                    if !boxes.contains(&(nx, ny)) {
                                        selected.push((nx, ny));
                                        boxes.push((nx, ny));
                                    }
                                    if !boxes.contains(&(nx - 1, ny)) {
                                        selected.push((nx - 1, ny));
                                        boxes.push((nx - 1, ny));
                                    }
                                }
                                '[' => {
                                    if !boxes.contains(&(nx, ny)) {
                                        selected.push((nx, ny));
                                        boxes.push((nx, ny));
                                    }
                                    if !boxes.contains(&(nx + 1, ny)) {
                                        selected.push((nx + 1, ny));
                                        boxes.push((nx + 1, ny));
                                    }
                                }
                                '.' => {}
                                '#' => {
                                    break;
                                }
                                _ => panic!("character {} not recognised", o_ch),
                            }
                        } else {
                            break;
                        }
                    }
                    // in theory the last two boxes will be the furthest away, we can check if
                    // there is a wall (if there is we do nothing!)
                    let is_touching_wall = boxes
                        .iter()
                        .rev()
                        .map(|(x, y)| (x + dx, y + dy))
                        .any(|(lx, ly)| world.get_isize(lx, ly) == Some(&'#'));

                    if !is_touching_wall {
                        let new_boxes = boxes
                            .iter()
                            .filter(|(bx, by)| *world.get_unsafe(*bx as usize, *by as usize) == '[')
                            .map(|(bx, by)| (bx + dx, by + dy))
                            .collect::<Vec<_>>();
                        for (bx, by) in boxes {
                            *world.get_mut(bx, by).unwrap() = '.';
                        }
                        for (bx, by) in &new_boxes {
                            *world.get_mut(*bx, *by).unwrap() = '[';
                            *world.get_mut(bx + 1, *by).unwrap() = ']';
                        }
                        *world.get_mut(x, y).unwrap() = '.';
                        *world.get_mut(new_x, new_y).unwrap() = '@';
                        robot_pos = (new_x, new_y);
                    }
                }
                '[' | ']' if dy == 0 => {
                    let (mut cx, mut cy) = (new_x, new_y);
                    let mut boxes = vec![];
                    while let Some(other_ch) = world.get_isize(cx, cy) {
                        match *other_ch {
                            '[' | ']' => {
                                boxes.push((cx, cy));
                                cx += dx;
                                cy += dy;
                            }
                            '.' => {
                                // move all them boxes
                                for (bx, by) in boxes.iter().rev() {
                                    let old_box = *world.get_mut(*bx, *by).unwrap();
                                    *world.get_mut(bx + dx, by + dy).unwrap() = old_box;
                                    *world.get_mut(*bx, *by).unwrap() = '.';
                                }
                                *world.get_mut(x, y).unwrap() = '.';
                                *world.get_mut(new_x, new_y).unwrap() = '@';
                                robot_pos = (new_x, new_y);
                                break;
                            }
                            '#' => {
                                break;
                            }
                            _ => panic!("found unrecognised character {}", other_ch),
                        }
                    }
                }
                '.' => {
                    *world.get_mut(x, y).unwrap() = '.';
                    *world.get_mut(new_x, new_y).unwrap() = '@';
                    robot_pos = (new_x, new_y);
                }
                '#' => {}
                _ => panic!("character not found {}", ch),
            }
        }
    }

    let mut result = 0;
    for x in 0..world.width {
        for y in 0..world.height {
            let ch = world.get_unsafe(x, y);
            if *ch == '[' {
                result += (y * 100) + x
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(10092));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
