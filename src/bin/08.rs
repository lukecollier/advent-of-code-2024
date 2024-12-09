use std::collections::{HashMap, HashSet};

use advent_of_code::XYWorld;
use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let world = XYWorld::from_str(input);
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for x in 0..world.width {
        for y in 0..world.height {
            let ch = world.get(x, y).unwrap();
            if *ch != '.' {
                let coords = antennas.entry(*ch).or_insert(vec![]);
                coords.push((x, y));
            }
        }
    }
    for coords in antennas.values() {
        for ((fx, fy), (sx, sy)) in coords.iter().tuple_combinations::<(_, _)>() {
            let fx_isize = *fx as isize;
            let fy_isize = *fy as isize;
            let sx_isize = *sx as isize;
            let sy_isize = *sy as isize;
            let fdx = fx_isize - sx_isize;
            let fdy = fy_isize - sy_isize;
            let sdx = sx_isize - fx_isize;
            let sdy = sy_isize - fy_isize;
            if world.contains(fx_isize + fdx, fy_isize + fdy) {
                points.insert(((fx_isize + fdx) as usize, (fy_isize + fdy) as usize));
            }
            if world.contains(sx_isize + sdx, sy_isize + sdy) {
                points.insert(((sx_isize + sdx) as usize, (sy_isize + sdy) as usize));
            }
        }
    }
    Some(points.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let world = XYWorld::from_str(input);
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for x in 0..world.width {
        for y in 0..world.height {
            let ch = world.get(x, y).unwrap();
            if *ch != '.' {
                let coords = antennas.entry(*ch).or_insert(vec![]);
                coords.push((x, y));
            }
        }
    }
    for coords in antennas.values() {
        for ((fx, fy), (sx, sy)) in coords.iter().tuple_combinations::<(_, _)>() {
            let (mut fx_isize, mut fy_isize) = (*fx as isize, *fy as isize);
            let (mut sx_isize, mut sy_isize) = (*sx as isize, *sy as isize);
            let fdx = fx_isize - sx_isize;
            let fdy = fy_isize - sy_isize;
            let sdx = sx_isize - fx_isize;
            let sdy = sy_isize - fy_isize;
            while world.contains(fx_isize, fy_isize) {
                points.insert((fx_isize as usize, fy_isize as usize));
                fx_isize += fdx;
                fy_isize += fdy;
            }
            while world.contains(sx_isize, sy_isize) {
                points.insert((sx_isize as usize, sy_isize as usize));
                sx_isize += sdx;
                sy_isize += sdy;
            }
        }
    }
    Some(points.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
