use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").sorted().collect_vec();
    assert!(lines.next().is_some_and(|line| line.is_empty()));
    let mut successes = 0;
    for line in lines {
        let mut ranges: HashMap<usize, Vec<usize>> = HashMap::new();
        for pattern in &patterns {
            let mut idx = 0;
            for chunk in line.chars().collect_vec().windows(pattern.len()) {
                let str = chunk.iter().collect::<String>();
                if &str == pattern {
                    ranges
                        .entry(idx)
                        .and_modify(|m| m.push(idx + pattern.len()))
                        .or_insert(vec![idx + pattern.len()]);
                }
                idx += 1;
            }
        }
        // If no range starts at 0 we've done nothing
        let mut seen: HashSet<usize> = HashSet::new();
        if let Some(next_up) = ranges.get(&0) {
            let mut next_up = next_up.into_iter().collect::<BinaryHeap<_>>();
            while let Some(start_at) = next_up.pop() {
                if seen.insert(*start_at) {
                    if *start_at == line.len() {
                        successes += 1;
                        next_up.clear();
                        break;
                    }
                    if let Some(range) = ranges.get(&start_at) {
                        for next in range {
                            next_up.push(next);
                        }
                    }
                }
            }
        }
    }
    Some(successes)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").sorted().collect_vec();
    assert!(lines.next().is_some_and(|line| line.is_empty()));
    let mut successes = 0;
    for line in lines {
        let mut tree: HashMap<usize, VecDeque<usize>> = HashMap::new();
        for pattern in &patterns {
            let mut idx = 0;
            for chunk in line.chars().collect_vec().windows(pattern.len()) {
                let str = chunk.iter().collect::<String>();
                if &str == pattern {
                    tree.entry(idx + pattern.len())
                        .and_modify(|m| {
                            m.push_back(idx);
                        })
                        .or_insert(VecDeque::from_iter(vec![idx]));
                }
                idx += 1;
            }
        }

        let mut strengths: HashMap<usize, usize> = HashMap::new();
        // By using a binary heap we can _almost_ gaurentee we've visited every parent node
        // **BEFORE** we visit a child, neat!
        let mut next_up = BinaryHeap::from_iter(vec![line.len()]);
        while let Some(current) = next_up.pop() {
            let curr_strength = strengths.get(&current).unwrap_or(&1).clone();
            if let Some(range) = tree.get(&current) {
                for value in range {
                    if let Some(freq) = strengths.get_mut(value) {
                        *freq += curr_strength;
                    } else {
                        strengths.insert(*value, curr_strength);
                        next_up.push(*value);
                    }
                }
            }
        }
        successes += strengths.get(&0).unwrap_or(&0);
    }
    Some(successes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
