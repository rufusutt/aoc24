fn valid_equation(target: u64, result: u64, numbers: &[u64], cat: bool) -> bool {
    // Base case: no more numbers to process
    if numbers.is_empty() {
        return result == target;
    }
    // If result exceeds target, no need to continue
    if result > target {
        return false;
    }

    let next = numbers[0];

    // Try addition
    if valid_equation(target, result + next, &numbers[1..], cat) {
        return true;
    }

    // Try multiplication
    if valid_equation(target, result * next, &numbers[1..], cat) {
        return true;
    }

    // Try concatenation
    if cat {
        // Concat the base 10 representation of the numbers
        let concat = format!("{}{}", result, next).parse::<u64>().unwrap();
        if valid_equation(target, concat, &numbers[1..], cat) {
            return true;
        }
    }

    false
}

pub fn solution(input: &str) {
    let mut numbers = Vec::new();

    let (part1, part2) = input
        .lines()
        .map(|line| {
            let (target, num_str) = line.split_once(':').unwrap();

            // Parse the target and numbers
            let target = target.parse::<u64>().unwrap();
            numbers.clear();
            numbers.extend(
                num_str
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().unwrap()),
            );

            let part1 = if valid_equation(target, 0, &numbers, false) {
                target
            } else {
                0
            };
            let part2 = if valid_equation(target, 0, &numbers, true) {
                target
            } else {
                0
            };

            (part1, part2)
        })
        .fold((0, 0), |(acc1, acc2), (p1, p2)| (acc1 + p1, acc2 + p2));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
