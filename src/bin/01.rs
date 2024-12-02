advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_list: Vec<u32> = Vec::with_capacity(input.len());
    let mut right_list: Vec<u32> = Vec::with_capacity(input.len());
    for line in input.lines() {
        let numbers = line
            .split_whitespace()
            .filter_map(|str| str.parse::<u32>().ok())
            .collect::<Vec<_>>();
        if let (Some(left), Some(right)) = (numbers.get(0), numbers.get(1)) {
            left_list.push(*left);
            right_list.push(*right);
        } else {
            panic!("NO NUMBERS")
        }
    }
    left_list.sort();
    right_list.sort();
    let mut total_distance = 0;
    for (left, right) in left_list.iter().zip(right_list) {
        total_distance += left.abs_diff(right)
    }
    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_list: Vec<u32> = Vec::with_capacity(input.len());
    let mut right_list: Vec<u32> = Vec::with_capacity(input.len());
    for line in input.lines() {
        let numbers = line
            .split_whitespace()
            .filter_map(|str| str.parse::<u32>().ok())
            .collect::<Vec<_>>();
        if let (Some(left), Some(right)) = (numbers.get(0), numbers.get(1)) {
            left_list.push(*left);
            right_list.push(*right);
        } else {
            panic!("NO NUMBERS")
        }
    }
    left_list.sort();
    right_list.sort();
    let mut total_distance = 0;
    for left in left_list {
        let occurances = right_list.iter().filter(|right| left == **right).count() as u32;
        total_distance += left * occurances
    }
    Some(total_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
