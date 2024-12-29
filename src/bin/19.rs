use itertools::Itertools;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").sorted().collect_vec();
    assert!(lines.next().is_some_and(|line| line.is_empty()));
    let mut successes = 0;
    for line in lines {
        let mut start = 0;
        while start < line.len() {
            let mut found = false;
            for end in (start..=line.len()).rev() {
                if let Some(pattern) = line.get(start..end) {
                    println!("searching for: {}", pattern);
                    if patterns.contains(&pattern) {
                        println!("found at: {}", start);
                        start = end;
                        if start == line.len() {
                            successes += 1;
                        }
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                // if we haven't found, we're finished
                break;
            }
        }
    }
    Some(successes)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
