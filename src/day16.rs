use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const EAST: (isize, isize) = DIRECTIONS[1];

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (isize, isize),
    dir: (isize, isize),
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

    fn rotation_cost(from: (isize, isize), to: (isize, isize)) -> usize {
        let from_idx = DIRECTIONS.iter().position(|&d| d == from).unwrap();
        let to_idx = DIRECTIONS.iter().position(|&d| d == to).unwrap();

        let clockwise = (4 + to_idx as isize - from_idx as isize) % 4;
        let counterclockwise = (4 - clockwise) % 4;

        let rotations = std::cmp::min(clockwise, counterclockwise) as usize;
        rotations * 1000
    }

    fn shortest_path(
        &self,
        start: (isize, isize),
        start_dir: (isize, isize),
        end: (isize, isize),
    ) -> Option<usize> {
        let mut costs: HashMap<((isize, isize), (isize, isize)), usize> = HashMap::new();

        let mut queue = BinaryHeap::new();
        queue.push(State {
            cost: 0,
            pos: start,
            dir: start_dir,
        });

        costs.insert((start, start_dir), 0);

        while let Some(State { cost, pos, dir }) = queue.pop() {
            // Skip if we've found a better path
            if let Some(&best) = costs.get(&(pos, dir)) {
                if cost > best {
                    continue;
                }
            }

            // Check end condition
            if pos == end {
                return Some(cost);
            }

            // Try each direction
            for &new_dir in DIRECTIONS.iter() {
                let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);

                match self.get_tile(new_pos) {
                    Some('#') => continue,
                    Some(_) => {
                        let rotation_cost = Self::rotation_cost(dir, new_dir);
                        let new_cost = cost + rotation_cost + 1;

                        // Only visit if we haven't found a better path to this state
                        if new_cost < *costs.get(&(new_pos, new_dir)).unwrap_or(&usize::MAX) {
                            costs.insert((new_pos, new_dir), new_cost);
                            queue.push(State {
                                cost: new_cost,
                                pos: new_pos,
                                dir: new_dir,
                            });
                        }
                    }
                    _ => panic!("Out of bounds"),
                }
            }
        }

        None
    }

    fn optimal_tiles(
        &self,
        start: (isize, isize),
        start_dir: (isize, isize),
        end: (isize, isize),
    ) -> Option<usize> {
        let mut costs: HashMap<((isize, isize), (isize, isize)), usize> = HashMap::new();
        let mut previous: HashMap<
            ((isize, isize), (isize, isize)),
            Vec<((isize, isize), (isize, isize))>,
        > = HashMap::new();
        let mut queue = BinaryHeap::new();

        queue.push(State {
            cost: 0,
            pos: start,
            dir: start_dir,
        });
        costs.insert((start, start_dir), 0);

        while let Some(State { cost, pos, dir }) = queue.pop() {
            if let Some(&best) = costs.get(&(pos, dir)) {
                if cost > best {
                    continue;
                }
            }

            for &new_dir in DIRECTIONS.iter() {
                let new_pos = (pos.0 + new_dir.0, pos.1 + new_dir.1);
                match self.get_tile(new_pos) {
                    Some('#') => continue,
                    Some(_) => {
                        let rotation_cost = Self::rotation_cost(dir, new_dir);
                        let new_cost = cost + rotation_cost + 1;

                        if new_cost <= *costs.get(&(new_pos, new_dir)).unwrap_or(&usize::MAX) {
                            costs.insert((new_pos, new_dir), new_cost);

                            // There can be multiple paths to the same state
                            let path = previous.entry((new_pos, new_dir)).or_default();
                            if !path.contains(&(pos, dir)) {
                                path.push((pos, dir));
                            }

                            queue.push(State {
                                cost: new_cost,
                                pos: new_pos,
                                dir: new_dir,
                            });
                        }
                    }
                    _ => panic!("Out of bounds"),
                }
            }
        }

        // Backtrack to find all tiles on all optimal paths
        let min_cost = costs
            .iter()
            .filter(|((pos, _), _)| *pos == end)
            .map(|(_, &cost)| cost)
            .min()?;

        let mut optimal_tiles: HashSet<(isize, isize)> = HashSet::new();
        let mut to_visit = Vec::new();

        // Collect all end states with minimal cost
        for ((pos, dir), &cost) in &costs {
            if *pos == end && cost == min_cost {
                to_visit.push((*pos, *dir));
            }
        }

        while let Some((pos, dir)) = to_visit.pop() {
            optimal_tiles.insert(pos);
            if let Some(prev_states) = previous.get(&(pos, dir)) {
                for &prev in prev_states {
                    if !optimal_tiles.contains(&prev.0) {
                        to_visit.push(prev);
                    }
                }
            }
        }

        Some(optimal_tiles.len())
    }
}

pub fn solution(input: &str) {
    let map = Map::new(input).unwrap();

    // Find start and end pos
    let start = map.find('S').next().unwrap();
    let end = map.find('E').next().unwrap();

    // Find shortest path
    let part1 = map.shortest_path(start, EAST, end).unwrap();
    println!("Part 1: {}", part1);

    let part2 = map.optimal_tiles(start, EAST, end).unwrap();
    println!("Part 2: {}", part2);
}
