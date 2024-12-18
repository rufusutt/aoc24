mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;

const SOLUTIONS: &[fn(&str)] = &[
    day01::solution,
    day02::solution,
    day03::solution,
    day04::solution,
    day05::solution,
    day06::solution,
    day07::solution,
    day08::solution,
    day09::solution,
    day10::solution,
    day11::solution,
    day12::solution,
    day13::solution,
    day14::solution,
    day15::solution,
    day16::solution,
    day17::solution,
    day18::solution,
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <day> <input>", args[0]);
        std::process::exit(1);
    }

    let day = match args[1].parse::<usize>() {
        Ok(d) if d > 0 && d <= SOLUTIONS.len() => d,
        _ => {
            eprintln!("Invalid day: {}", args[1]);
            std::process::exit(1);
        }
    };

    let input = match std::fs::read_to_string(&args[2]) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            std::process::exit(1);
        }
    };

    SOLUTIONS[day - 1](&input);
}
