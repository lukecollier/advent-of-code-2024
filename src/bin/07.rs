use itertools::Itertools;
use rayon::{iter::ParallelIterator, str::ParallelString};

advent_of_code::solution!(7);

pub fn solutions(numbers: &Vec<usize>, filter: usize) -> Vec<usize> {
    let mut answers: Vec<usize> = vec![];
    let mut remaining = numbers.iter();
    let first = remaining.next().unwrap();
    let second = remaining.next().unwrap();
    answers.push(first + second);
    answers.push(first * second);
    for number in remaining {
        let mut new_answers: Vec<usize> = vec![];
        while let Some(answer) = answers.pop() {
            let addition = answer + number;
            let multiply = answer * number;
            if addition <= filter {
                new_answers.push(addition);
            }
            if multiply <= filter {
                new_answers.push(multiply);
            }
        }
        answers = new_answers;
    }
    answers
}

pub fn part_one(input: &str) -> Option<usize> {
    let answer = input
        .par_lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(test_str, numbers_str)| {
            let test_value = test_str
                .parse::<usize>()
                .expect("could not parse test value");
            let numbers = numbers_str
                .split_ascii_whitespace()
                .map(|number_str| number_str.parse::<usize>().expect("could not parse number"))
                .collect_vec();
            (test_value, numbers)
        })
        .map(|(test_value, numbers)| {
            let solutions = solutions(&numbers, test_value);
            if solutions.contains(&test_value) {
                test_value as usize
            } else {
                0_usize
            }
        })
        .sum::<usize>();
    Some(answer)
}

pub fn solutions_three_operators(numbers: &Vec<usize>, filter: usize) -> Vec<usize> {
    let mut answers: Vec<usize> = vec![];
    let mut remaining = numbers.iter();
    let first = remaining.next().unwrap();
    let second = remaining.next().unwrap();
    answers.push(first + second);
    answers.push(first * second);
    answers.push(format!("{}{}", first, second).parse::<usize>().unwrap());
    for number in remaining {
        let mut new_answers: Vec<usize> = vec![];
        while let Some(answer) = answers.pop() {
            if answer <= filter {
                let addition = answer + number;
                let multiply = answer * number;
                let concat = format!("{}{}", answer, number).parse::<usize>().unwrap();
                new_answers.push(addition);
                new_answers.push(multiply);
                new_answers.push(concat);
            }
        }
        answers = new_answers;
    }
    answers
}

pub fn part_two(input: &str) -> Option<usize> {
    let answer = input
        .par_lines()
        .filter_map(|line| line.split_once(": "))
        .map(|(test_str, numbers_str)| {
            let test_value = test_str
                .parse::<usize>()
                .expect("could not parse test value");
            let numbers = numbers_str
                .split_ascii_whitespace()
                .map(|number_str| number_str.parse::<usize>().expect("could not parse number"))
                .collect_vec();
            (test_value, numbers)
        })
        .map(|(test_value, numbers)| {
            let solutions = solutions_three_operators(&numbers, test_value);
            if solutions.contains(&test_value) {
                test_value as usize
            } else {
                0_usize
            }
        })
        .sum::<usize>();
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_numbers_third_op() {
        let result = solutions_three_operators(&vec![10, 19], 1019);
        assert_eq!(result, vec![29, 190, 1019]);
    }

    #[test]
    fn two_numbers() {
        let result = solutions(&vec![10, 19], 190);
        assert_eq!(result, vec![29, 190]);
    }

    #[test]
    fn three_numbers() {
        let result = solutions(&vec![81, 40, 27], 87480);
        assert_eq!(result, vec![3267, 87480, 148, 3267]);
    }

    #[test]
    fn three_numbers_with_filter() {
        let result = solutions(&vec![81, 40, 27], 3267);
        assert_eq!(result, vec![3267, 148, 3267]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
