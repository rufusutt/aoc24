use regex::Regex;

pub fn solution(input: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let part1: i64 = re
        .captures_iter(input)
        .map(|captures| {
            let first: i64 = captures[1].parse().unwrap();
            let second: i64 = captures[2].parse().unwrap();
            first * second
        })
        .sum();
    println!("Part 1: {}", part1);

    let re = Regex::new(r"((?:mul|do|don't))\(((?:\d+,\d+)?)\)").unwrap();

    let mut enabled = true;
    let part2: i64 = re
        .captures_iter(input)
        .filter_map(|captures| {
            let command = &captures[1];
            let args = &captures[2];

            match command {
                "mul" if enabled => {
                    let mut numbers = args.split(',').filter_map(|n| n.parse::<i64>().ok());
                    let first = numbers.next().unwrap();
                    let second = numbers.next().unwrap();
                    Some(first * second)
                }
                "do" => {
                    enabled = true;
                    None
                }
                "don't" => {
                    enabled = false;
                    None
                }
                _ => None,
            }
        })
        .sum();
    println!("Part 2: {}", part2);
}
