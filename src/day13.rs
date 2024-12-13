use regex::Regex;

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn parse_machine(group: &str) -> Option<Machine> {
    // TODO: Static
    let button_regex = Regex::new(r"X\+(\d+), Y\+(\d+)").ok()?;
    let prize_regex = Regex::new(r"X=(\d+), Y=(\d+)").ok()?;

    let mut lines = group.lines();

    let parse_coords = |captures: regex::Captures| -> Option<(i64, i64)> {
        Some((
            captures.get(1)?.as_str().parse().ok()?,
            captures.get(2)?.as_str().parse().ok()?,
        ))
    };

    let a = parse_coords(button_regex.captures(lines.next()?)?)?;
    let b = parse_coords(button_regex.captures(lines.next()?)?)?;
    let prize = parse_coords(prize_regex.captures(lines.next()?)?)?;

    Some(Machine { a, b, prize })
}

fn calculate_gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (gcd, x1, y1) = extended_gcd(b, a % b);
    (gcd, y1, x1 - (a / b) * y1)
}

fn solve_diophantine(x: i64, y: i64, target: i64) -> Vec<(i64, i64)> {
    println!("Solving {}x + {}y = {}", x, y, target);

    // Calculate GCD and check if the equation has a solution
    let gcd = calculate_gcd(x, y);
    if target % gcd != 0 {
        println!("No solution");
        return Vec::new();
    }

    // Get Bézout's identity coefficients
    let (_, a0, b0) = extended_gcd(x, y);
    println!("Base solution: {}x + {}y = {}", a0, b0, gcd);
    println!();

    // Scale the coefficients to match the target
    let factor = target / gcd;
    let a0 = a0 * factor;
    let b0 = b0 * factor;

    // Equations:
    // a = a0 + k * step_a
    // b = b0 - k * step_b

    // Find the range of k values that make both a and b positive
    let step_a = y / gcd;
    let step_b = x / gcd;

    let k_min = if a0 >= 0 {
        (-a0 / step_a) - 1
    } else {
        -a0 / step_a
    };
    let k_max = b0 / step_b;

    println!("Searching k in [{}, {}]", k_min, k_max);

    let mut solutions = Vec::new();
    for k in k_min..=k_max {
        let a = a0 + k * step_a;
        let b = b0 - k * step_b;
        if a >= 0 && b >= 0 {
            solutions.push((a, b));
        }
        println!("Solution {}: a = {}, b = {}", k, a, b);
    }

    println!();

    solutions
}

pub fn solution(input: &str) {
    let machines = input
        .split("\n\n")
        .map(|group| parse_machine(group).unwrap())
        .collect::<Vec<_>>();

    let mut part1 = 0;

    machines.iter().for_each(|machine| {
        // Solve independently for x and y
        let solutions_x = solve_diophantine(machine.a.0, machine.b.0, machine.prize.0);
        let solutions_y = solve_diophantine(machine.a.1, machine.b.1, machine.prize.1);

        // Find cross-section of solutions
        let mut solutions = Vec::new();
        for (a_x, b_x) in solutions_x {
            for (a_y, b_y) in solutions_y.iter().copied() {
                if a_x == a_y && b_x == b_y {
                    solutions.push((a_x, b_x));
                }
            }
        }

        // Print solutions
        println!("Solutions:");
        for (a, b) in &solutions {
            println!("x = {}, y = {}", a, b);
        }

        if let Some(score) = solutions.iter().map(|(a, b)| a * 3 + b).min() {
            part1 += score;
        }

        println!();
        println!();

        // let target = machine.prize.0 % x_gcd == 0
    });

    println!("Part 1: {}", part1);
}
