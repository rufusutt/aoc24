use regex::Regex;

pub fn solution(input: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let part1 = re
        .captures_iter(input)
        .map(|captures| {
            let (_, groups): (&str, [&str; 2]) = captures.extract();
            let mut numbers = groups.iter().map(|g| g.parse::<i64>().unwrap());
            let first = numbers.next().unwrap();
            let seconds = numbers.next().unwrap();

            first * seconds
        })
        .sum::<i64>();
    println!("Part 1: {}", part1);

    let re = Regex::new(r"((?:mul|do|don't))\(((?:\d+,\d+)?)\)").unwrap();

    let mut enabled = true;
    let part2 = re
        .captures_iter(input)
        .map(|captures| {
            let (_, [command, args]) = captures.extract();

            let mut numbers = args.split(',').map(|n| n.parse::<i64>().unwrap());

            match command {
                "mul" if enabled => {
                    let first = numbers.next().unwrap();
                    let second = numbers.next().unwrap();
                    return Some(first * second);
                }
                "do" => {
                    enabled = true;
                    return None;
                }
                "don't" => {
                    enabled = false;
                    return None;
                }
                _ => None,
            }
        })
        .flatten()
        .sum::<i64>();
    println!("Part 2: {}", part2);
}
