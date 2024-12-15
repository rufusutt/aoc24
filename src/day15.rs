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

    fn tile(&self, (x, y): (isize, isize)) -> Option<char> {
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

    fn tile_mut(&mut self, (x, y): (isize, isize)) -> Option<&mut char> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&mut self.tiles[y * self.width + x])
    }

    fn find(&self, c: char) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(move |(i, &tile)| {
                if tile == c {
                    Some((i % self.width, i / self.width))
                } else {
                    None
                }
            })
            .map(|(x, y)| (x as isize, y as isize))
    }

    fn can_push_box(&self, pos: (isize, isize), direction: (isize, isize)) -> bool {
        let next_pos = (pos.0 + direction.0, pos.1 + direction.1);
        match self.tile(next_pos) {
            Some('.') | None => true,
            Some('#') => false,
            Some('O') => self.can_push_box(next_pos, direction),
            _ => false,
        }
    }

    fn walk(&mut self, pos: &mut (isize, isize), direction: (isize, isize)) {
        let next_pos = (pos.0 + direction.0, pos.1 + direction.1);

        match self.tile(next_pos) {
            Some('#') => {
                // Wall, do nothing
            }
            Some('.') => {
                // Open space, move
                *self.tile_mut(*pos).unwrap() = '.';
                *self.tile_mut(next_pos).unwrap() = '@';
                *pos = next_pos;
            }
            Some('O') => {
                // Box, try to push
                if self.can_push_box(next_pos, direction) {
                    // Find all boxes in a row and move them
                    let mut boxes = vec![next_pos];
                    let mut current = next_pos;

                    // Find all consecutive boxes
                    while let Some('O') =
                        self.tile((current.0 + direction.0, current.1 + direction.1))
                    {
                        current = (current.0 + direction.0, current.1 + direction.1);
                        boxes.push(current);
                    }

                    // Move boxes starting from the last one
                    for &box_pos in boxes.iter().rev() {
                        let new_pos = (box_pos.0 + direction.0, box_pos.1 + direction.1);
                        *self.tile_mut(box_pos).unwrap() = '.';
                        *self.tile_mut(new_pos).unwrap() = 'O';
                    }

                    // Move player
                    *self.tile_mut(*pos).unwrap() = '.';
                    *self.tile_mut(next_pos).unwrap() = '@';
                    *pos = next_pos;
                }
            }
            _ => panic!("Invalid tile"),
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.tiles[y * self.width + x]);
            }
            println!();
        }
    }
}

pub fn solution(input: &str) {
    let (map, directions) = input.split_once("\n\n").unwrap();
    let mut map = Map::new(map).unwrap();

    // Find the starting position
    let mut pos = map.find('@').next().unwrap();

    map.print();
    println!();

    for char in directions.chars().filter(|&c| c != '\n') {
        let direction = match char {
            '^' => DIRECTIONS[0],
            '>' => DIRECTIONS[1],
            'v' => DIRECTIONS[2],
            '<' => DIRECTIONS[3],
            _ => panic!("Invalid direction"),
        };
        map.walk(&mut pos, direction);
    }

    map.print();
    println!();

    // Find all boxex to calculate the "GPS" score
    let part1 = map.find('O').map(|pos| pos.0 + pos.1 * 100).sum::<isize>();
    println!("Part 1: {}", part1);
}
