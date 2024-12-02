pub fn solution(input: &str) {
    // Parse input into left and right columns
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace().map(|p| p.parse::<i64>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .unzip();

    // Sort each column independently
    left.sort_unstable();
    right.sort_unstable();

    // Sum of differences between sorted columns
    let sum: i64 = left.iter().zip(&right).map(|(l, r)| (l - r).abs()).sum();
    println!("Part 1: {}", sum);

    // Map of occurrences of each number in the right column
    let right_map = right
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &r| {
            *acc.entry(r).or_insert(0) += 1;
            acc
        });

    // "Similarity score"
    // The number in left column multiplied by occurrences in right column
    let score: i64 = left
        .iter()
        .map(|l| l * right_map.get(l).unwrap_or(&0))
        .sum();
    println!("Part 2: {}", score);
}
