use std::collections::HashMap;

use itertools::Itertools;

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

    let mut prices = Vec::new();

    // For each initial value, create a map of sequences to bananas
    let maps: Vec<_> = input
        .lines()
        .map(|line| {
            let initial = line.parse::<u64>().unwrap();

            prices.clear();
            prices.push(initial % 10);
            (0..2000).fold(initial, |secret, _| {
                let secret = step(secret);
                prices.push(secret % 10);
                secret
            });

            let mut map: HashMap<[i64; 4], u64> = HashMap::new();

            for window in prices.windows(5) {
                // Get the sequence
                let mut sequence = [0; 4];
                for (i, values) in window.windows(2).enumerate() {
                    sequence[i] = values[1] as i64 - values[0] as i64;
                }

                // Get the banana value
                let banana = window[4];

                // If this is a new sequence, add it to the map
                map.entry(sequence).or_insert(banana);
            }

            map
        })
        .collect();

    // For every unique key in all maps, find the maximum value by summing accross all maps.
    let part2 = maps
        .iter()
        .flat_map(|map| map.keys())
        .unique()
        .map(|key| {
            maps.iter()
                .map(|map| map.get(key).unwrap_or(&0))
                .sum::<u64>()
        })
        .max()
        .unwrap();
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
