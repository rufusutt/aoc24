use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

const DOOR: [&str; 4] = ["789", "456", "123", "#0A"];
const DPAD: [&str; 2] = ["#^A", "<v>"];

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    fn to_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }

    fn to_delta(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

fn search(coords: &HashMap<(isize, isize), char>, a: char, b: char) -> Vec<String> {
    let inv: HashMap<char, (isize, isize)> = coords.iter().map(|(&pos, &ch)| (ch, pos)).collect();

    let start = inv[&a];
    let end = inv[&b];

    let mut q = VecDeque::new();
    q.push_back((start, String::new()));

    let mut accum = Vec::new();
    let mut dist = usize::MAX;

    while let Some((pos, steps)) = q.pop_front() {
        if pos == end && steps.len() <= dist {
            accum.push(steps.clone());
            dist = steps.len();
            continue;
        }

        if (steps.len() + 1) < dist {
            for direction in Direction::ALL {
                let (dy, dx) = direction.to_delta();
                let new_pos = (pos.0 + dy, pos.1 + dx);
                if coords.contains_key(&new_pos) && coords[&new_pos] != '#' {
                    let mut new_steps = steps.clone();
                    new_steps.push(direction.to_char());
                    q.push_back((new_pos, new_steps));
                }
            }
        }
    }

    accum
}

fn steps(s: &str) -> Vec<(char, char)> {
    s.chars().tuple_windows().collect()
}

fn best_path(
    start: char,
    end: char,
    n: usize,
    moves: &HashMap<(char, char), Vec<String>>,
    dpad_moves: &HashMap<(char, char), Vec<String>>,
    memo: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    if let Some(dist) = memo.get(&(start, end, n)) {
        return *dist;
    }

    let mut dist = usize::MAX;

    for path in moves.get(&(start, end)).unwrap_or(&vec!["".to_string()]) {
        if n == 0 {
            dist = dist.min(path.len());
        } else {
            let path_length: usize = steps(&format!("A{}A", path))
                .iter()
                .map(|&(s2, e2)| best_path(s2, e2, n - 1, dpad_moves, dpad_moves, memo))
                .sum();
            dist = dist.min(path_length);
        }
    }

    memo.insert((start, end, n), dist);
    dist
}

fn solve(
    input: &str,
    n: usize,
    door_moves: &HashMap<(char, char), Vec<String>>,
    dpad_moves: &HashMap<(char, char), Vec<String>>,
    memo: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    input
        .lines()
        .map(|sequence| {
            let length: usize = steps(&format!("A{}", sequence))
                .iter()
                .map(|&(s, e)| best_path(s, e, n, door_moves, dpad_moves, memo))
                .sum();

            let numeric_string = sequence
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>();
            let numeric = numeric_string.parse::<usize>().unwrap();

            length * numeric
        })
        .sum::<usize>()
}

pub fn solution(input: &str) {
    // Create coordinate maps
    let mut door_c = HashMap::new();
    for (y, line) in DOOR.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '#' {
                door_c.insert((y as isize, x as isize), ch);
            }
        }
    }
    let mut dpad_c = HashMap::new();
    for (y, line) in DPAD.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '#' {
                dpad_c.insert((y as isize, x as isize), ch);
            }
        }
    }

    // Initialise moves hashmaps
    let mut door_moves = HashMap::new();
    let mut dpad_moves = HashMap::new();

    // Generate all moves
    for (a, b) in door_c.values().tuple_combinations() {
        door_moves.insert((*b, *a), search(&door_c, *b, *a));
        door_moves.insert((*a, *b), search(&door_c, *a, *b));
    }
    for (a, b) in dpad_c.values().tuple_combinations() {
        dpad_moves.insert((*b, *a), search(&dpad_c, *b, *a));
        dpad_moves.insert((*a, *b), search(&dpad_c, *a, *b));
    }

    let mut memo = HashMap::new();

    let part1 = solve(input, 2, &door_moves, &dpad_moves, &mut memo);
    println!("Part 1: {}", part1);

    let part2 = solve(input, 25, &door_moves, &dpad_moves, &mut memo);
    println!("Part 2: {}", part2);
}
