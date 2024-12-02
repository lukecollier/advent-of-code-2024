advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut successes = 0;
    for numbers in input.lines().map(|line| {
        line.split_whitespace()
            .map(|number_str| number_str.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    }) {
        let differences = numbers
            .windows(2)
            .map(|window| window[0] as i64 - window[1] as i64)
            .collect::<Vec<_>>();
        if differences.iter().find(|num| num.abs() > 3).is_none()
            && (differences.iter().all(|num| *num > 0) || differences.iter().all(|num| *num < 0))
        {
            successes += 1
        }
    }
    Some(successes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut successes = 0;
    for numbers in input.lines().map(|line| {
        line.split_whitespace()
            .map(|number_str| number_str.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    }) {
        let differences = numbers
            .windows(2)
            .map(|window| window[0] as i64 - window[1] as i64)
            .collect::<Vec<_>>();
        if differences.iter().find(|num| num.abs() > 3).is_none()
            && (differences.iter().all(|num| *num > 0) || differences.iter().all(|num| *num < 0))
        {
            successes += 1
        } else {
            for idx in 0..numbers.len() {
                let mut removed = numbers.clone();
                removed.remove(idx);
                let differences = removed
                    .windows(2)
                    .map(|window| window[0] as i64 - window[1] as i64)
                    .collect::<Vec<_>>();
                if differences.iter().find(|num| num.abs() > 3).is_none()
                    && (differences.iter().all(|num| *num > 0)
                        || differences.iter().all(|num| *num < 0))
                {
                    successes += 1;
                    break;
                }
            }
        }
    }
    Some(successes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
