fn count_digits(mut n: u64) -> u32 {
    let mut digits = 0;
    while n > 0 {
        n /= 10;
        digits += 1;
    }
    digits
}

fn split_stone(stone: u64) -> (u64, u64) {
    let digits = count_digits(stone);
    let divisor = 10u64.pow(digits / 2);
    (stone / divisor, stone % divisor)
}

fn blink(stones: &mut Vec<u64>) {
    let mut i = 0;
    while i < stones.len() {
        let stone = &mut stones[i];

        if *stone == 0 {
            *stone = 1;
            i += 1;
        } else if count_digits(*stone) % 2 == 0 {
            let (a, b) = split_stone(*stone);
            stones[i] = a;
            stones.insert(i + 1, b);
            i += 2;
        } else {
            *stone *= 2024;
            i += 1;
        }
    }
}

pub fn solution(input: &str) {
    let mut stones: Vec<_> = input
        .trim()
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect();

    for _ in 0..25 {
        blink(&mut stones);
    }

    let part1 = stones.len();
    println!("Part 1: {}", part1);

    for i in 0..50 {
        println!("Blink {}", i + 25);
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
