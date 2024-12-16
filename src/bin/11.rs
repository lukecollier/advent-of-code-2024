use std::{collections::HashMap, time::Instant};

use itertools::Itertools;
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSlice,
};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut line: Vec<String> = input
        .split_ascii_whitespace()
        .map(|str| str.to_string())
        .collect();
    for _ in 0..25 {
        let mut next_line: Vec<String> = vec![];
        for num_str in line {
            let length = num_str.chars().count();
            if num_str == "0" {
                next_line.push("1".to_string());
            } else if length % 2 == 0 {
                let (left, right) = num_str.split_at(length / 2);
                next_line.push(left.parse::<usize>().unwrap().to_string());
                next_line.push(right.parse::<usize>().unwrap().to_string());
            } else {
                let parsed = num_str.parse::<usize>().unwrap() * 2024;
                next_line.push(parsed.to_string());
            }
        }
        line = next_line;
    }
    Some(line.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let init_line: Vec<&str> = input.split_ascii_whitespace().collect();
    let lines = init_line.par_iter().flat_map(|stone| {
        let mut line: HashMap<String, usize> = HashMap::new();
        line.insert(stone.to_string(), 1);
        let started_at = Instant::now();
        for _ in 0..75 {
            let line_iter = line.iter_mut();
            let mut next_line: HashMap<String, usize> = HashMap::new();
            for (next_stone, v) in line_iter {
                let length = next_stone.len();
                if *next_stone == "0".to_string() {
                    *next_line.entry("1".to_string()).or_insert(0) += *v;
                } else if length % 2 == 0 {
                    let (left, right) = next_stone.split_at(length / 2);
                    *next_line
                        .entry(left.parse::<usize>().unwrap().to_string())
                        .or_insert(0) += *v;
                    *next_line
                        .entry(right.parse::<usize>().unwrap().to_string())
                        .or_insert(0) += *v;
                } else {
                    let parsed = next_stone.parse::<usize>().unwrap() * 2024;
                    *next_line.entry(parsed.to_string()).or_insert(0) += *v;
                }
            }
            line = next_line;
        }
        line
    });
    let mut final_map: HashMap<String, usize> = HashMap::new();
    for (k, v) in lines.collect_vec_list().into_iter().flatten() {
        *final_map.entry(k).or_insert(0) += v;
    }
    final_map.values().sum1::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
