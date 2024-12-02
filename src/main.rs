mod day01;
mod day02;

const SOLUTIONS: &[fn(&str)] = &[day01::solution, day02::solution];

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
