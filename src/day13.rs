use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn new(group: &str) -> Option<Machine> {
        static BUTTON_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap());
        static PRIZE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"X=(\d+), Y=(\d+)").unwrap());

        let mut lines = group.lines();

        let parse_coords = |captures: regex::Captures| -> Option<(i64, i64)> {
            Some((
                captures.get(1)?.as_str().parse().ok()?,
                captures.get(2)?.as_str().parse().ok()?,
            ))
        };

        let a = parse_coords(BUTTON_REGEX.captures(lines.next()?)?)?;
        let b = parse_coords(BUTTON_REGEX.captures(lines.next()?)?)?;
        let prize = parse_coords(PRIZE_REGEX.captures(lines.next()?)?)?;

        Some(Machine { a, b, prize })
    }

    fn get_coefficients(&self, target: (i64, i64)) -> Option<(i64, i64)> {
        // Extract vectors and target based on whether we're doing part 2
        let v1 = self.a;
        let v2 = self.b;

        // Calculate determinant
        let det = v1.0 * v2.1 - v1.1 * v2.0;

        // Calculate coefficients using Cramer's rule
        let a = target.0 * v2.1 - target.1 * v2.0;
        let b = v1.0 * target.1 - v1.1 * target.0;

        // Check if we have integer solutions
        if a % det == 0 && b % det == 0 {
            Some((a / det, b / det))
        } else {
            None
        }
    }
}

pub fn solution(input: &str) {
    let mut part1 = 0;
    let mut part2 = 0;

    input
        .split("\n\n")
        .map(|group| Machine::new(group).unwrap())
        .for_each(|machine| {
            if let Some((a, b)) = machine.get_coefficients(machine.prize) {
                part1 += 3 * a + b;
            }

            let part2_prize = (
                machine.prize.0 + 10_i64.pow(13),
                machine.prize.1 + 10_i64.pow(13),
            );
            if let Some((a, b)) = machine.get_coefficients(part2_prize) {
                part2 += 3 * a + b;
            }
        });

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
