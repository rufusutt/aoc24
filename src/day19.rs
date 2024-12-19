use std::collections::HashMap;

fn design_possible(design: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|pat| {
        if !design.starts_with(pat) {
            return false;
        }

        let design = &design[pat.len()..];

        if design.is_empty() {
            return true;
        }

        design_possible(design, patterns)
    })
}

fn count_arrangements<'a>(
    design: &'a str,
    patterns: &[&'a str],
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&count) = memo.get(design) {
        return count;
    }

    // Try each pattern as a potential start
    let mut total = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            let remaining = &design[pattern.len()..];
            total += count_arrangements(remaining, patterns, memo);
        }
    }

    memo.insert(design, total);
    total
}

pub fn solution(input: &str) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();

    let patterns: Vec<_> = patterns.split(", ").collect();
    let designs: Vec<_> = designs.lines().collect();

    let part1 = designs
        .iter()
        .filter(|design| design_possible(design, &patterns))
        .count();
    println!("Part 1: {}", part1);

    let mut memo = HashMap::new();
    let part2 = designs
        .iter()
        .map(|design| count_arrangements(design, &patterns, &mut memo))
        .sum::<usize>();
    println!("Part 2: {}", part2);
}
