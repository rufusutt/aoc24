use std::collections::HashMap;

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn step(secret: u64) -> u64 {
    let step1 = prune(mix(secret, secret * 64));
    let step2 = prune(mix(step1, step1 / 32));
    prune(mix(step2, step2 * 2048))
}

pub fn solution(input: &str) {
    let part1 = input
        .lines()
        .map(|line| {
            let initial = line.parse::<u64>().unwrap();
            // Find 2000th step value
            (0..2000).fold(initial, |secret, _| step(secret))
        })
        .sum::<u64>();
    println!("Part 1: {}", part1);

    // Pre-allocate the vector with expected capacity
    let mut maps: Vec<HashMap<[i64; 4], u64>> = Vec::with_capacity(input.lines().count());
    // Reuse prices vector across iterations
    let mut prices = Vec::with_capacity(2000);

    for line in input.lines() {
        let initial = line.parse::<u64>().unwrap();
        let mut secret = initial;

        prices.clear();
        prices.push(initial % 10);
        prices.extend((0..1999).map(|_| {
            secret = step(secret);
            secret % 10
        }));

        // Create map for this sequence
        let mut map = HashMap::new();

        let mut window = [0i64; 4];
        for chunk in prices.windows(5) {
            for i in 0..4 {
                window[i] = chunk[i + 1] as i64 - chunk[i] as i64;
            }
            map.entry(window).or_insert(chunk[4]);
        }

        maps.push(map);
    }

    // Create a unified map to track maximum sums
    let mut max_sums: HashMap<[i64; 4], u64> = HashMap::new();

    // Calculate sums for each sequence
    for map in &maps {
        for (&sequence, &value) in map {
            max_sums
                .entry(sequence)
                .and_modify(|sum| *sum += value)
                .or_insert(value);
        }
    }

    // Find maximum sum
    let part2 = max_sums.values().max().copied().unwrap_or(0);

    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37)
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920)
    }

    #[test]
    fn test_step() {
        let sequence = [
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254,
        ];

        let mut secret = sequence[0];
        for &value in sequence.iter().skip(1) {
            secret = step(secret);
            assert_eq!(secret, value);
        }
    }
}
