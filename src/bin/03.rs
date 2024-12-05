use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one_regex(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut total = 0;
    for group in re.captures_iter(input) {
        let (_, [first, second]) = group.extract();
        total += first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap();
    }
    Some(total)
}
const TERMS: [char; 4] = ['m', 'u', 'l', '('];

pub fn part_one(input: &str) -> Option<u32> {
    part_one_regex(input)
}

pub fn part_one_parser(input: &str) -> Option<u32> {
    let mut characters = input.chars().peekable();
    let mut total = 0;
    let mut pos = 0;
    let mut buf = String::new();
    while let Some(ch) = characters.next() {
        if pos == TERMS.len() {
            if ch == ')' {
                if let Some((first, last)) = buf.split_once(',') {
                    total += first.parse::<u32>().unwrap() * last.parse::<u32>().unwrap();
                    buf.clear();
                    pos = 0;
                } else {
                    buf.clear();
                    pos = 0;
                }
            } else if ch.is_ascii_digit() || ch == ',' {
                buf.push(ch);
            } else {
                buf.clear();
                pos = 0;
            }
        }
        if pos != TERMS.len() && ch == TERMS[pos] {
            pos += 1;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"don't\(\)((.|\n)*?)(do\(\)|\z)").unwrap();
    let replaced = re.replace_all(input, "");
    part_one(&replaced)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }

    #[test]
    fn test_part_two_dont_start() {
        let result = part_two("don't()mul(1,1)");
        assert_eq!(result, Some(0));
    }
    #[test]
    fn test_part_two_do() {
        let result = part_two("don't()mul(1,1)do()mul(1,1)");
        assert_eq!(result, Some(1));
    }
    #[test]
    fn test_part_two_do_2() {
        let result = part_two("mul(1,1)don't()mul(1,1)do()mul(1,1)");
        assert_eq!(result, Some(2));
    }
    #[test]
    fn test_part_two_handle_newline() {
        let result = part_two("don't()\nmul(1,2)");
        assert_eq!(result, Some(0));
    }
}
