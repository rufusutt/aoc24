use std::collections::HashMap;

pub fn count_digits(n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    ((n as f64).log10() as u32) + 1
}

fn split_stone(stone: u64) -> (u64, u64) {
    let digits = count_digits(stone);
    let divisor = 10u64.pow(digits / 2);
    (stone / divisor, stone % divisor)
}

fn solve(stones: &mut [u64], steps: u32) -> usize {
    fn recursion(
        stone: u64,
        depth: u32,
        target: u32,
        memo: &mut HashMap<(u64, u32), usize>,
    ) -> usize {
        if let Some(&result) = memo.get(&(stone, depth)) {
            return result;
        }

        if depth == target {
            return 1;
        }

        let result = if stone == 0 {
            recursion(1, depth + 1, target, memo)
        } else if count_digits(stone) % 2 == 0 {
            let (a, b) = split_stone(stone);
            recursion(a, depth + 1, target, memo) + recursion(b, depth + 1, target, memo)
        } else {
            recursion(stone * 2024, depth + 1, target, memo)
        };

        memo.insert((stone, depth), result);
        return result;
    }

    let mut memo = HashMap::new();
    stones
        .iter()
        .map(|&stone| recursion(stone, 0, steps, &mut memo))
        .sum()
}

pub fn solution(input: &str) {
    let stones: Vec<_> = input
        .trim()
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect();

    let part1 = solve(&mut stones.clone(), 25);
    println!("Part 1: {}", part1);

    let part2 = solve(&mut stones.clone(), 75);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(123), 3);
        assert_eq!(count_digits(1234), 4);
        assert_eq!(count_digits(12345), 5);
    }

    #[test]
    fn test_split_stone() {
        assert_eq!(split_stone(1234), (12, 34));
        assert_eq!(split_stone(1001), (10, 1));
    }
}
