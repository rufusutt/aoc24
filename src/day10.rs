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
        pos: (isize, isize),
        target: u32,
        visited: &mut HashSet<(isize, isize)>,
    ) -> usize {
        match self.get_tile(pos) {
            // If out of bounds
            None => 0,
            // If height is not the target
            Some(height) if height != target => 0,
            // If we've reached the end of the trail
            Some(9) => usize::from(visited.insert(pos)),
            // Count peaks in all directions
            Some(_) => DIRECTIONS
                .iter()
                .map(|&(dx, dy)| {
                    let next_pos = (pos.0 + dx, pos.1 + dy);
                    self.count_trail_peaks(next_pos, target + 1, visited)
                })
                .sum(),
        }
    }

    fn count_unique_trails(&self, pos: (isize, isize), target: u32) -> usize {
        match self.get_tile(pos) {
            // If out of bounds
            None => 0,
            // If height is not the target
            Some(height) if height != target => 0,
            // If we've reached the end of the trail
            Some(9) => 1,
            // Count trails in all directions
            Some(_) => DIRECTIONS
                .iter()
                .map(|&(dx, dy)| {
                    let next_pos = (pos.0 + dx, pos.1 + dy);
                    self.count_unique_trails(next_pos, target + 1)
                })
                .sum(),
        }
    }
}

pub fn solution(input: &str) {
    let map = Map::new(input).unwrap();

    let part1 = map
        .find(0)
        .map(|start| {
            let mut visited = HashSet::new();
            map.count_trail_peaks(start, 0, &mut visited)
        })
        .sum::<usize>();
    println!("Part 1: {}", part1);

    let part2 = map
        .find(0)
        .map(|start| map.count_unique_trails(start, 0))
        .sum::<usize>();
    println!("Part 2: {}", part2);
}
