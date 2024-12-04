fn valid_delta(delta: i64) -> bool {
    (1..=3).contains(&delta.abs())
}

fn safe_report(levels: &[i64]) -> bool {
    // Shouldn't occur?
    if levels.len() < 2 {
        return true;
    }

    // Get the first delta to determine the desired direction
    let first_delta = levels[1] - levels[0];
    if !valid_delta(first_delta) {
        return false;
    }

    // Check all consecutive pairs
    levels.windows(2)
        .map(|pair| pair[1] - pair[0])
        .all(|delta| {
            valid_delta(delta) && 
            // Check if the delta is in the same direction as the first delta
            delta.signum() == first_delta.signum()
        })
}

fn dampened_safe_report(levels: &[i64]) -> bool {
    // Try to run the original safe_report
    if safe_report(levels) {
        return true;
    }

    // Try removing every item and running safe_report
    for (i, _) in levels.iter().enumerate() {
        let mut levels = levels.to_vec();
        levels.remove(i);
        if safe_report(&levels) {
            return true;
        }
    }

    false
}

pub fn solution(input: &str) {
    let (safe_count, dampened_safe_count) = input
        .lines()
        .map(|report| {
            // Parse report into levels
            let levels: Vec<i64> = report
                .split_whitespace()
                .map(|l| l.parse().unwrap())
                .collect();

            (
                safe_report(&levels) as i64,
                dampened_safe_report(&levels) as i64,
            )
        })
        .fold((0, 0), |(safe_acc, dampened_acc), (safe, dampened)| {
            (safe_acc + safe, dampened_acc + dampened)
        });

    println!("Part 1: {}", safe_count);
    println!("Part 2: {}", dampened_safe_count);
}
