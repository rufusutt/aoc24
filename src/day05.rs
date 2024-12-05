use std::collections::HashSet;

pub fn solution(input: &str) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules: HashSet<(u32, u32)> = rules
        .lines()
        .map(|line| {
            let mut parts = line.split('|').map(|part| part.parse::<u32>().unwrap());
            let before = parts.next().unwrap();
            let page = parts.next().unwrap();
            (before, page)
        })
        .collect();

    let valid_update =
        |update: &[u32]| -> bool { update.is_sorted_by(|&a, &b| rules.contains(&(a, b))) };

    // Reuse vec
    let mut update = Vec::new();

    let part1: u32 = updates
        .lines()
        .filter_map(|line| {
            update.clear();
            update.extend(line.split(',').map(|page| page.parse::<u32>().unwrap()));

            // If sorted, return middle element
            if valid_update(&update) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum();
    println!("Part 1: {}", part1);

    let part2: u32 = updates
        .lines()
        .flat_map(|line| {
            update.clear();
            update.extend(line.split(',').map(|page| page.parse::<u32>().unwrap()));

            // Already valid, ignore
            if valid_update(&update) {
                return None;
            }

            let m = update.len() / 2;
            let (_, &mut m, _) = update.select_nth_unstable_by(m, |&a, &b| {
                if rules.contains(&(a, b)) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            });

            Some(m)
        })
        .sum();
    println!("Part 2: {}", part2);
}
