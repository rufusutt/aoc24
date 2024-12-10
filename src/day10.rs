use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Map {
    tiles: Vec<u32>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Option<Self> {
        let tiles = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<_>>>()?;
        let width = input.lines().next()?.chars().count();
        let height = input.lines().count();

        Some(Self {
            tiles,
            width,
            height,
        })
    }

    fn get_tile(&self, (x, y): (isize, isize)) -> Option<u32> {
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

    fn find(&self, height: u32) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(move |(i, &tile)| {
                if tile == height {
                    Some((i % self.width, i / self.width))
                } else {
                    None
                }
            })
            .map(|(x, y)| (x as isize, y as isize))
    }

    fn count_trail_peaks(
        &self,
        (x, y): (isize, isize),
        target: u32,
        visited: &mut HashSet<(isize, isize)>,
    ) -> usize {
        // If out of bounds, return 0
        let Some(current) = self.get_tile((x, y)) else {
            return 0;
        };

        // Height must be increasing
        if current != target {
            return 0;
        }

        // We've reached the end of the trail
        if current == 9 {
            if visited.insert((x, y)) {
                return 1;
            } else {
                return 0;
            }
        }

        // Count trails in all directions
        let mut count = 0;
        for &(dx, dy) in &DIRECTIONS {
            count += self.count_trail_peaks((x + dx, y + dy), target + 1, visited);
        }

        count
    }

    fn count_unique_trails(
        &self,
        (x, y): (isize, isize),
        target: u32,
    ) -> usize {
        // If out of bounds, return 0
        let Some(current) = self.get_tile((x, y)) else {
            return 0;
        };

        // Height must be increasing
        if current != target {
            return 0;
        }

        // We've reached the end of the trail
        if current == 9 {
            return 1;
        }

        // Count trails in all directions
        let mut count = 0;
        for &(dx, dy) in &DIRECTIONS {
            count += self.count_unique_trails((x + dx, y + dy), target + 1);
        }

        count
    }
}

pub fn solution(input: &str) {
    let map = Map::new(input).unwrap();

    let part1 = map.find(0).map(|start| {
        let mut visited = HashSet::new();
        map.count_trail_peaks(start, 0, &mut visited)
    }).sum::<usize>();
    println!("Part 1: {}", part1);

    let part2 = map.find(0).map(|start| {
        map.count_unique_trails(start, 0)
    }).sum::<usize>();
    println!("Part 2: {}", part2);
}
