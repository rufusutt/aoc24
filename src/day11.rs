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

fn blink(stones: &mut Vec<u64>) {
    let mut new_stones = Vec::with_capacity(stones.len() * 2);

    for stone in stones.iter() {
        if *stone == 0 {
            new_stones.push(1);
        } else if count_digits(*stone) % 2 == 0 {
            let (a, b) = split_stone(*stone);
            new_stones.push(a);
            new_stones.push(b);
        } else {
            new_stones.push(stone * 2024);
        }
    }

    *stones = new_stones;
}

pub fn solution(input: &str) {
    let mut stones: Vec<_> = input
        .trim()
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect();

    for _ in 1..=25 {
        blink(&mut stones);
    }

    let part1 = stones.len();
    println!("Part 1: {}", part1);

    for i in 25..=75 {
        println!("Blink {}: {}", i, stones.len());
        blink(&mut stones);
    }

    let part2 = stones.len();
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
