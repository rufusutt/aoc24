use std::collections::{HashSet, VecDeque};

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

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

    fn index(&self, (x, y): (isize, isize)) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(y * self.width + x)
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

    fn shortest_path(&self, start: (isize, isize), end: (isize, isize)) -> Vec<(isize, isize)> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut result = Vec::new();

        queue.push_back(start);

        while let Some((x, y)) = queue.pop_back() {
            if visited.contains(&(x, y)) {
                continue;
            }

            visited.insert((x, y));
            result.push((x, y));

            if (x, y) == end {
                return result;
            }

            for &(dx, dy) in &DIRECTIONS {
                let next = (x + dx, y + dy);
                if let Some(tile) = self.tile(next) {
                    if tile != '#' {
                        queue.push_front(next);
                    }
                }
            }
        }

        Vec::new()
    }

    fn cheats(&self, path: &[(isize, isize)], max_distance: isize, min_diff: isize) -> usize {
        let mut times = vec![-1; self.width * self.height];

        for (i, &pos) in path.iter().enumerate() {
            times[self.index(pos).unwrap()] = i as isize;
        }

        path.iter()
            .map(|&pos| {
                let mut count = 0;
                for dx in -max_distance..=max_distance {
                    for dy in -max_distance..=max_distance {
                        let distance = dx.abs() + dy.abs();
                        let new_pos = (pos.0 + dx, pos.1 + dy);

                        let Some(new_idx) = self.index(new_pos) else {
                            continue;
                        };

                        if distance > max_distance {
                            continue;
                        }

                        let current_time = times[self.index(pos).unwrap()];
                        let target_time = times[new_idx];

                        if target_time >= 0 && (current_time - target_time - distance) >= min_diff {
                            count += 1;
                        }
                    }
                }
                count
            })
            .sum()
    }
}

pub fn solution(input: &str) {
    let map = Map::new(input).unwrap();
    let start = map.find('S').unwrap();
    let end = map.find('E').unwrap();
    let shortest_path = map.shortest_path(start, end);

    println!("Part 1: {}", map.cheats(&shortest_path, 2, 100));
    println!("Part 2: {}", map.cheats(&shortest_path, 20, 100));
}
