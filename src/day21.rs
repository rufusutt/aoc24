use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    fn offset(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}

struct Keypad<'a> {
    tiles: &'a [Option<char>],
    width: usize,
    height: usize,
}

const NUMERIC: Keypad = Keypad {
    tiles: &[
        Some('7'),
        Some('8'),
        Some('9'),
        Some('4'),
        Some('5'),
        Some('6'),
        Some('1'),
        Some('2'),
        Some('3'),
        None,
        Some('0'),
        Some('A'),
    ],
    width: 3,
    height: 4,
};

const CONTROL: Keypad = Keypad {
    tiles: &[None, Some('^'), Some('A'), Some('<'), Some('v'), Some('>')],
    width: 3,
    height: 2,
};

impl<'a> Keypad<'a> {
    fn iter(&self) -> impl Iterator<Item = ((isize, isize), char)> + '_ {
        self.tiles.iter().enumerate().filter_map(move |(i, &tile)| {
            if let Some(tile) = tile {
                let x = i % self.width;
                let y = i / self.width;
                Some(((x as isize, y as isize), tile))
            } else {
                None
            }
        })
    }

    fn index(&self, (x, y): (isize, isize)) -> Option<usize> {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            Some((y as usize * self.width) + x as usize)
        } else {
            None
        }
    }

    fn coords(&self, index: usize) -> (isize, isize) {
        let x = index % self.width;
        let y = index / self.width;
        (x as isize, y as isize)
    }

    fn shortest_path(&self, start: (isize, isize), end: (isize, isize)) -> Vec<Direction> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.tiles.len()];
        let mut previous = vec![None; self.tiles.len()];

        let start_index = self.index(start).unwrap();
        let end_index = self.index(end).unwrap();

        queue.push_back(start_index);
        visited[start_index] = true;

        while let Some(current_index) = queue.pop_front() {
            if current_index == end_index {
                break;
            }

            for direction in &Direction::ALL {
                let (dx, dy) = direction.offset();
                let (x, y) = self.coords(current_index);
                let new_x = x + dx;
                let new_y = y + dy;

                if let Some(new_index) = self.index((new_x, new_y)) {
                    if !visited[new_index] && self.tiles[new_index].is_some() {
                        queue.push_back(new_index);
                        visited[new_index] = true;
                        previous[new_index] = Some(current_index);
                    }
                }
            }
        }

        let mut path = Vec::new();
        let mut current_index = end_index;
        while current_index != start_index {
            let previous_index = previous[current_index].unwrap();
            let (x, y) = self.coords(previous_index);
            let (new_x, new_y) = self.coords(current_index);
            let direction = match (new_x - x, new_y - y) {
                (0, -1) => Direction::Up,
                (0, 1) => Direction::Down,
                (-1, 0) => Direction::Left,
                (1, 0) => Direction::Right,
                _ => unreachable!(),
            };
            path.push(direction);
            current_index = previous_index;
        }

        path.reverse();
        path
    }
}

fn find_path(path_map: &HashMap<(char, char), Vec<Direction>>, sequence: &str) -> String {
    let mut current = 'A';
    let mut path = String::new();

    for next in sequence.chars() {
        let path_segment = path_map.get(&(current, next)).unwrap();
        for &direction in path_segment {
            path.push(direction.to_char());
        }
        path.push('A');
        current = next;
    }

    path
}

pub fn solution(input: &str) {
    // For every combination of start and end, find the shortest path
    let mut numeric_paths: HashMap<(char, char), Vec<Direction>> = HashMap::new();
    for (start_pos, start) in NUMERIC.iter() {
        for (end_pos, end) in NUMERIC.iter() {
            let path = NUMERIC.shortest_path(start_pos, end_pos);
            numeric_paths.insert((start, end), path);
        }
    }

    // For every combination of start and end, find the shortest path
    let mut control_paths: HashMap<(char, char), Vec<Direction>> = HashMap::new();
    for (start_pos, start) in CONTROL.iter() {
        for (end_pos, end) in CONTROL.iter() {
            let path = CONTROL.shortest_path(start_pos, end_pos);
            control_paths.insert((start, end), path);
        }
    }

    let sequence = "029A";
    let control_sequence = find_path(&numeric_paths, sequence);

}
