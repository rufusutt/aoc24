use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (isize, isize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Note: reversed for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Map {
    tiles: Vec<char>,
    size: usize,
}

impl Map {
    fn new(size: usize) -> Option<Self> {
        Some(Self {
            tiles: vec!['.'; size * size],
            size,
        })
    }

    fn tile(&self, (x, y): (isize, isize)) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.size || y >= self.size {
            return None;
        }
        Some(self.tiles[y * self.size + x])
    }

    fn tile_mut(&mut self, (x, y): (isize, isize)) -> Option<&mut char> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.size || y >= self.size {
            return None;
        }
        Some(&mut self.tiles[y * self.size + x])
    }

    fn shortest_path(&self) -> Option<usize> {
        let mut costs: HashMap<(isize, isize), usize> = HashMap::new();
        let mut queue = BinaryHeap::new();

        queue.push(State {
            cost: 0,
            pos: (0, 0),
        });
        costs.insert((0, 0), 0);

        while let Some(State { cost, pos }) = queue.pop() {
            if cost > *costs.get(&pos).unwrap_or(&usize::MAX) {
                continue;
            }

            if pos == (self.size as isize - 1, self.size as isize - 1) {
                return Some(cost);
            }

            for &new_dir in &DIRECTIONS {
                let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);

                if let Some(tile) = self.tile(new_pos) {
                    if tile == '#' {
                        continue;
                    }

                    let new_cost = cost + 1;
                    if new_cost < *costs.get(&new_pos).unwrap_or(&usize::MAX) {
                        costs.insert(new_pos, new_cost);
                        queue.push(State {
                            cost: new_cost,
                            pos: new_pos,
                        });
                    }
                }
            }
        }

        None
    }
}

pub fn solution(input: &str) {
    let parse_line = |line: &str| -> (isize, isize) {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse::<isize>().unwrap();
        let y = parts.next().unwrap().parse::<isize>().unwrap();
        (x, y)
    };

    let mut map = Map::new(71).unwrap();
    input.lines().take(1024).for_each(|line| {
        let pos = parse_line(line);
        *map.tile_mut(pos).unwrap() = '#';
    });

    let part1 = map.shortest_path().unwrap();
    println!("Part 1: {}", part1);

    let mut map = Map::new(71).unwrap();
    let part2 = input.lines().map(parse_line).find(|pos| {
        *map.tile_mut(*pos).unwrap() = '#';
        map.shortest_path().is_none()
    });

    if let Some(part2) = part2 {
        println!("Part 2: {},{}", part2.0, part2.1);
    } else {
        println!("Part 2: No solution found");
    }
}
