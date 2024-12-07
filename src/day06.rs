use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Map {
    tiles: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Option<Self> {
        let tiles = input.lines().flat_map(|line| line.chars()).collect();
        let width = input.lines().next()?.chars().count();
        let height = input.lines().count();

        Some(Self {
            tiles,
            width,
            height,
        })
    }

    fn get_tile(&self, (x, y): (isize, isize)) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.tiles[y * self.width + x])
    }

    fn find(&self, c: char) -> Option<(isize, isize)> {
        self.tiles
            .iter()
            .enumerate()
            .find_map(|(i, &tile)| {
                if tile == c {
                    Some((i % self.width, i / self.width))
                } else {
                    None
                }
            })
            .map(|(x, y)| (x as isize, y as isize))
    }

    fn obstructed(
        &self,
        (x, y): (isize, isize),
        (dx, dy): (isize, isize),
        obstruction: Option<(isize, isize)>,
    ) -> bool {
        let next_pos = (x + dx, y + dy);
        if let Some(obstruction_pos) = obstruction {
            if next_pos == obstruction_pos {
                return true;
            }
        }
        matches!(self.get_tile(next_pos), Some('#'))
    }

    fn guard_path(&self, start: (isize, isize)) -> HashSet<(isize, isize)> {
        let mut visited = HashSet::new();
        let mut pos = start;
        let mut direction = 0;

        while self.get_tile(pos).is_some() {
            // Mark the current position as visited
            visited.insert(pos);

            let (dx, dy) = DIRECTIONS[direction];

            if self.obstructed(pos, (dx, dy), None) {
                // Turn right
                direction = (direction + 1) % 4;
            } else {
                // Step forward
                pos = (pos.0 + dx, pos.1 + dy);
            }
        }

        visited
    }

    fn new_loop(&self, start: (isize, isize), obstruction: (isize, isize)) -> bool {
        let mut visited = HashSet::new();
        let mut pos = start;
        let mut direction = 0;

        while self.get_tile(pos).is_some() {
            // If we've visited this position and direction before, we're in a loop
            if !visited.insert((pos, direction)) {
                return true;
            }

            let (dx, dy) = DIRECTIONS[direction];

            if self.obstructed(pos, (dx, dy), Some(obstruction)) {
                // Turn right
                direction = (direction + 1) % 4;
            } else {
                // Step forward
                pos = (pos.0 + dx, pos.1 + dy);
            }
        }

        false
    }

    fn count_loops(&self, start: (isize, isize), visited: &HashSet<(isize, isize)>) -> usize {
        visited
            .iter()
            .filter(|&&pos| pos != start && self.new_loop(start, pos))
            .count()
    }
}

pub fn solution(input: &str) {
    let map = Map::new(input).unwrap();

    // Find the start square denoted by '^'
    let start = map.find('^').unwrap();

    let path = map.guard_path(start);

    println!("Part 1: {}", path.len());
    println!("Part 2: {}", map.count_loops(start, &path));
}
