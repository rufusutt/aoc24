use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Map {
    tiles: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Option<Self> {
        let tiles = input
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>();
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

    fn count_edges(&self, pos: (isize, isize)) -> usize {
        let target = self.get_tile(pos);
        DIRECTIONS
            .iter()
            .map(|&(dx, dy)| {
                if self.get_tile((pos.0 + dx, pos.1 + dy)) != target {
                    1
                } else {
                    0
                }
            })
            .sum::<usize>()
    }

    fn score_region(
        &self,
        start: (isize, isize),
        visited: &mut HashSet<(isize, isize)>,
    ) -> Option<usize> {
        // If we've already visited this tile, return
        if !visited.insert(start) {
            return None;
        }

        // The kind of tile we're looking for
        let target_char = self.get_tile(start)?;

        let mut queue = vec![start];
        let mut perimeter = 0;
        let mut count = 0;

        while let Some(pos) = queue.pop() {
            perimeter += self.count_edges(pos);
            count += 1;

            for &(dx, dy) in &DIRECTIONS {
                let new_pos = (pos.0 + dx, pos.1 + dy);
                if self.get_tile(new_pos) == Some(target_char) && visited.insert(new_pos) {
                    queue.push(new_pos);
                }
            }
        }

        Some(perimeter * count)
    }
}

pub fn solution(input: &str) {
    let map = Map::new(input).unwrap();

    let mut visited = HashSet::new();

    let mut score = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let pos = (x as isize, y as isize);
            if let Some(s) = map.score_region(pos, &mut visited) {
                score += s;
            }
        }
    }

    println!("Part 1: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_edges() {
        let input = "###\n#.#\n###";
        let map = Map::new(input).unwrap();

        assert_eq!(map.count_edges((0, 0)), 2);
        assert_eq!(map.count_edges((1, 0)), 2);
        assert_eq!(map.count_edges((1, 1)), 4);
    }
}
