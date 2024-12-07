fn valid_equation(target: u64, numbers: &[u64]) -> bool {
    fn helper(target: u64, result: u64, numbers: &[u64]) -> bool {
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
        if helper(target, result + next, &numbers[1..]) {
            return true;
        }

        // Try multiplication
        if helper(target, result * next, &numbers[1..]) {
            return true;
        }
        false
    }

    // Start recursion with the first number
    helper(target, numbers[0], &numbers[1..])
}

fn valid_equation_cat(target: u64, numbers: &[u64]) -> bool {
    fn helper(target: u64, result: u64, numbers: &[u64]) -> bool {
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
        if helper(target, result + next, &numbers[1..]) {
            return true;
        }

        // Try multiplication
        if helper(target, result * next, &numbers[1..]) {
            return true;
        }

        // Try concatenation
        // Concat the base 10 representation of the numbers
        let concat = format!("{}{}", result, next).parse::<u64>().unwrap();
        if helper(target, concat, &numbers[1..]) {
            return true;
        }

        false
    }

    // Start recursion with the first number
    helper(target, numbers[0], &numbers[1..])
}

pub fn solution(input: &str) {
    let part1 = input
        .lines()
        .flat_map(|line| {
            let (target, numbers) = line.split_once(":").unwrap();

            // Parse the target and numbers
            let target = target.parse::<u64>().unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            // Check if the target can be reached
            if valid_equation(target, &numbers) {
                Some(target)
            } else {
                None
            }
        })
        .sum::<u64>();
    println!("Part 1: {}", part1);

    let part2 = input
    .lines()
    .flat_map(|line| {
        let (target, numbers) = line.split_once(":").unwrap();

        // Parse the target and numbers
        let target = target.parse::<u64>().unwrap();
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        // Check if the target can be reached
        if valid_equation_cat(target, &numbers) {
            Some(target)
        } else {
            None
        }
    })
    .sum::<u64>();
    println!("Part 2: {}", part2);
}
