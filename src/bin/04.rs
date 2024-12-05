advent_of_code::solution!(4);

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

pub fn part_one(input: &str) -> Option<u32> {
    let mut world: Vec<Vec<char>> = vec![];
    let mut width = 0;
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<_>>();
        width = chars.len();
        world.push(chars);
    }
    let height = world.len();
    let mut total = 0;
    // left / right (5 for example)
    for y in 0..height {
        //forward
        let mut row: Vec<char> = Vec::new();
        for x in 0..width {
            let cur = world[y][x];
            row.push(cur);
        }
        for window in row.windows(XMAS.len()) {
            if window == XMAS {
                total += 1;
            }
        }
        row.reverse();
        for window in row.windows(XMAS.len()) {
            if window == XMAS {
                total += 1;
            }
        }
    }
    // up/down
    for x in 0..width {
        let mut column: Vec<char> = Vec::new();
        for y in 0..height {
            let cur = world[y][x];
            column.push(cur);
        }
        for window in column.windows(XMAS.len()) {
            if window == XMAS {
                total += 1;
            }
        }
        column.reverse();
        for window in column.windows(XMAS.len()) {
            if window == XMAS {
                total += 1;
            }
        }
    }
    // diagonal \ (top right)
    for x in 0..=width {
        let mut diagonal_top_right: Vec<char> = Vec::with_capacity(width);
        let mut diagonal_bottom_left_reversed: Vec<char> = Vec::with_capacity(width);
        for y in 0..(height - x) {
            let cur = world[y][x + y];
            diagonal_top_right.push(cur);
            if x != 0 {
                let cur_2 = world[height - y - 1][width - (y + x) - 1];
                diagonal_bottom_left_reversed.push(cur_2);
            }
        }
        for window in diagonal_top_right.windows(XMAS.len()) {
            if window == XMAS {
                total += 1;
            }
        }
        diagonal_top_right.reverse();
        for window in diagonal_top_right.windows(XMAS.len()) {
            if window == XMAS {
                total += 1;
            }
        }
        for window in diagonal_bottom_left_reversed.windows(XMAS.len()) {
            if window == XMAS {
                total += 1;
            }
        }
        diagonal_bottom_left_reversed.reverse();
        for window in diagonal_bottom_left_reversed.windows(XMAS.len()) {
            if window == XMAS {
                total += 1;
            }
        }
    }

    // diagonal / (top left)
    for x in 0..=width {
        let mut diagonal_top_left: Vec<char> = Vec::with_capacity(width);
        let mut diagonal_bot_right: Vec<char> = Vec::with_capacity(width);
        for y in 0..(height - x) {
            let cur = world[y][width - (x + y) - 1];
            diagonal_top_left.push(cur);
            if x != 0 {
                let cur_other = world[height - y - 1][x + y];
                diagonal_bot_right.push(cur_other);
            }
        }
        for window in diagonal_top_left.windows(XMAS.len()) {
            if window == XMAS {
                total += 1;
            }
        }
        diagonal_top_left.reverse();
        for window in diagonal_top_left.windows(XMAS.len()) {
            if window == XMAS {
                total += 1;
            }
        }

        for window in diagonal_bot_right.windows(XMAS.len()) {
            if window == XMAS {
                total += 1;
            }
        }
        diagonal_bot_right.reverse();
        for window in diagonal_bot_right.windows(XMAS.len()) {
            if window == XMAS {
                total += 1;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut world: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<_>>();
        world.push(chars);
    }
    let mut total = 0;
    for (y, row) in world.iter().enumerate().skip(1).rev().skip(1).rev() {
        for (x, ch) in row.iter().enumerate().skip(1).rev().skip(1).rev() {
            if *ch == 'A' {
                let top_left = world[y - 1][x - 1];
                let top_right = world[y - 1][x + 1];
                let bot_left = world[y + 1][x - 1];
                let bot_right = world[y + 1][x + 1];
                let checks = [[top_left, bot_right], [top_right, bot_left]];
                let mut found = 0;

                for check in checks {
                    if check == ['M', 'S'] || check == ['S', 'M'] {
                        found += 1;
                    }
                }
                if found == 2 {
                    total += 1;
                }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
