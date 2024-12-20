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

    fn find(&self, c: char) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(move |(i, &tile)| {
                if tile == c {
                    Some((i % self.width, i / self.width))
                } else {
                    None
                }
            })
            .map(|(x, y)| (x as isize, y as isize))
    }

    fn shortest_path(
        &self,
        start: (isize, isize),
        end: (isize, isize),
        skip: Option<(isize, isize)>,
    ) -> Option<usize> {
        let mut costs: HashMap<(isize, isize), usize> = HashMap::new();
        let mut queue = BinaryHeap::new();

        queue.push(State {
            cost: 0,
            pos: start,
        });
        costs.insert(start, 0);

        while let Some(State { cost, pos }) = queue.pop() {
            if cost > *costs.get(&pos).unwrap_or(&usize::MAX) {
                continue;
            }

            if pos == end {
                return Some(cost);
            }

            for &new_dir in &DIRECTIONS {
                let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);

                if let Some(tile) = self.tile(new_pos) {
                    if tile == '#' {
                        if let Some(skip) = skip {
                            if new_pos != skip {
                                continue;
                            }
                        } else {
                            continue;
                        }
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
    let map = Map::new(input).unwrap();

    // Find the start and end positions
    let start = map.find('S').next().unwrap();
    let end = map.find('E').next().unwrap();

    // Find shortest path without 'cheating'
    let track_len = map.shortest_path(start, end, None).unwrap();

    // Try skipping each wall in the map to find time saved
    let mut count = 0;
    for skip in map.find('#') {
        // No point skipping walls
        if skip.0 == 0
            || skip.0 == map.width as isize - 1
            || skip.1 == 0
            || skip.1 == map.height as isize - 1
        {
            continue;
        }

        let skip_len = map.shortest_path(start, end, Some(skip)).unwrap();
        if track_len - skip_len >= 100 {
            count += 1;
        }
    }

    println!("Part 1: {}", count);
}
