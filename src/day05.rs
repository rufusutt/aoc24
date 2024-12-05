use std::collections::{HashMap, HashSet};

fn parse_rules(input: &str) -> HashMap<u8, HashSet<u8>> {
    let mut rules: HashMap<u8, HashSet<u8>> = HashMap::new();

    input.lines().for_each(|line| {
        let mut parts = line.split('|').map(|part| part.parse::<u8>().unwrap());
        let before = parts.next().unwrap();
        let page = parts.next().unwrap();

        rules.entry(page).or_default().insert(before);
    });

    rules
}

fn valid_update(rules: &HashMap<u8, HashSet<u8>>, pages: &[u8]) -> bool {
    let mut invalid = HashSet::new();

    for page in pages.iter() {
        // A previous rule has declared this page can't occur
        if invalid.contains(page) {
            return false;
        }

        if let Some(before) = rules.get(page) {
            // Page must be before all pages in before
            invalid.extend(before.iter().copied());
        }
    }

    true
}

pub fn solution(input: &str) {
    let (rules_str, updates) = input.split_once("\n\n").unwrap();
    let rules = parse_rules(rules_str);

    let part1: u32 = updates
        .lines()
        .filter_map(|line| {
            // Collect all pages
            let pages: Vec<_> = line
                .split(',')
                .map(|page| page.parse::<u8>().unwrap())
                .collect();

            if !valid_update(&rules, &pages) {
                return None;
            }

            // Return middle page
            Some(pages[pages.len() / 2] as u32)
        })
        .sum();
    println!("Part 1: {}", part1);

    let part2: u32 = updates
        .lines()
        .flat_map(|line| {
            let mut pages: Vec<_> = line
                .split(',')
                .map(|page| page.parse::<u8>().unwrap())
                .collect();

            // Already valid, ignore
            if valid_update(&rules, &pages) {
                return None;
            }

            // Custom ordering based on rules
            pages.sort_unstable_by(|a, b| match rules.get(a) {
                Some(before_a) => {
                    if before_a.contains(b) {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                }
                None => std::cmp::Ordering::Equal,
            });

            // Return middle page
            Some(pages[pages.len() / 2] as u32)
        })
        .sum();
    println!("Part 2: {}", part2);
}
