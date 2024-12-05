use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut page_ordering_rules_before: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut input_lines = input.lines();
    while let Some(line) = input_lines.next() {
        if line.is_empty() {
            break;
        } else {
            if let Some((must_be_before_str, number_str)) = line.split_once("|") {
                let must_be_after: u32 = number_str.parse().unwrap();
                let must_be_before: u32 = must_be_before_str.parse().unwrap();
                if let Some(before) = page_ordering_rules_before.get_mut(&must_be_after) {
                    before.push(must_be_before);
                } else {
                    page_ordering_rules_before.insert(must_be_after, vec![must_be_before]);
                }
            }
        }
    }
    let mut total: u32 = 0;
    while let Some(line) = input_lines.next() {
        let pages = line
            .split(",")
            .map(|page| page.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let mut valid = true;
        for (i, page) in pages.iter().enumerate() {
            if let Some(before_rules) = page_ordering_rules_before.get(&page) {
                for other in pages.get(0..i).unwrap() {
                    valid = valid && before_rules.contains(other);
                }
                for other in pages.get(i..pages.len()).unwrap() {
                    valid = valid && !before_rules.contains(other);
                }
            }
        }
        if valid {
            total += pages[pages.len() / 2];
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut page_ordering_rules_before: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut input_lines = input.lines();
    while let Some(line) = input_lines.next() {
        if line.is_empty() {
            break;
        } else {
            if let Some((must_be_before_str, number_str)) = line.split_once("|") {
                let must_be_after: u32 = number_str.parse().unwrap();
                let must_be_before: u32 = must_be_before_str.parse().unwrap();
                if let Some(before) = page_ordering_rules_before.get_mut(&must_be_after) {
                    before.push(must_be_before);
                } else {
                    page_ordering_rules_before.insert(must_be_after, vec![must_be_before]);
                }
            }
        }
    }
    let mut invalid: Vec<Vec<u32>> = Vec::new();
    while let Some(line) = input_lines.next() {
        let pages = line
            .split(",")
            .map(|page| page.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let mut valid = true;
        for (i, page) in pages.iter().enumerate() {
            if let Some(before_rules) = page_ordering_rules_before.get(&page) {
                for other in pages.get(0..i).unwrap() {
                    valid = valid && before_rules.contains(other);
                }
                for other in pages.get(i..pages.len()).unwrap() {
                    valid = valid && !before_rules.contains(other);
                }
            }
        }
        if !valid {
            invalid.push(pages);
        }
    }

    let mut total = 0;
    for mut pages in invalid {
        loop {
            let mut changed = false;
            for pos in 0..(pages.len() - 1) {
                let page = pages[pos];
                let page_two = pages[pos + 1];
                if let Some(before_rules) = page_ordering_rules_before.get(&page) {
                    if before_rules.contains(&page_two) {
                        pages.swap(pos, pos + 1);
                        changed = true;
                    }
                }
            }
            if !changed {
                total += pages[pages.len() / 2];
                break;
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
