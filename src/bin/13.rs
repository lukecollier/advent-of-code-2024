use itertools::Itertools;

advent_of_code::solution!(13);

#[derive(Debug)]
struct ClawMachine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

impl ClawMachine {
    fn solve(&self) -> isize {
        let x1 = self.button_a.0;
        let x2 = self.button_a.1;
        let y1 = self.button_b.0;
        let y2 = self.button_b.1;
        let z1 = self.prize.0;
        let z2 = self.prize.1;
        let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
        let a = (z1 - b * y1) / x1;
        if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
            return 0;
        }
        a * 3 + b
    }

    fn solve_10000000000000(&self) -> isize {
        let x1 = self.button_a.0;
        let x2 = self.button_a.1;
        let y1 = self.button_b.0;
        let y2 = self.button_b.1;
        let z1 = self.prize.0 + 10000000000000;
        let z2 = self.prize.1 + 10000000000000;
        let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
        let a = (z1 - b * y1) / x1;
        if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
            return 0;
        }
        a * 3 + b
    }
}

fn parse(input: &str) -> Vec<ClawMachine> {
    let mut claw_machines = Vec::new();
    for line in &input.lines().filter(|line| !line.is_empty()).chunks(3) {
        if let Some((a_line, b_line, prize_line)) = line.collect_tuple::<(&str, &str, &str)>() {
            let (ax, ay) = a_line
                .strip_prefix("Button A: X+")
                .unwrap()
                .split_once(", Y+")
                .unwrap();
            let (bx, by) = b_line
                .strip_prefix("Button B: X+")
                .unwrap()
                .split_once(", Y+")
                .unwrap();

            let (px, py) = prize_line
                .strip_prefix("Prize: X=")
                .unwrap()
                .split_once(", Y=")
                .unwrap();

            let machine = ClawMachine {
                button_a: (ax.parse().unwrap(), ay.parse().unwrap()),
                button_b: (bx.parse().unwrap(), by.parse().unwrap()),
                prize: (px.parse().unwrap(), py.parse().unwrap()),
            };
            claw_machines.push(machine);
        }
    }

    claw_machines
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut result = 0;
    for machine in parse(input) {
        result += machine.solve();
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut result = 0;
    for machine in parse(input) {
        result += machine.solve_10000000000000();
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }
}
