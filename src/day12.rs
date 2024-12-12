use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const CORNERS: [((isize, isize), (isize, isize)); 4] = [
    ((0, -1), (1, 0)),
    ((1, 0), (0, 1)),
    ((0, 1), (-1, 0)),
    ((-1, 0), (0, -1)),
];

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

    fn count_corners(&self, pos: (isize, isize)) -> usize {
        let mut count = 0;

        let centre = self.get_tile(pos);

        for &(d1, d2) in &CORNERS {
            let d1_tile = self.get_tile((pos.0 + d1.0, pos.1 + d1.1));
            let d2_tile = self.get_tile((pos.0 + d2.0, pos.1 + d2.1));
            let diag_tile = self.get_tile((pos.0 + d1.0 + d2.0, pos.1 + d1.1 + d2.1));

            if d1_tile != centre && d2_tile != centre {
                count += 1;
            }

            if d1_tile == centre && d2_tile == centre && diag_tile != centre {
                count += 1;
            }
        }

        count
    }

    fn explore_area(
        &self,
        start: (isize, isize),
        visited: &mut HashSet<(isize, isize)>,
    ) -> Option<(usize, usize, usize)> {
        // If we've already visited this tile, return
        if !visited.insert(start) {
            return None;
        }

        // The kind of tile we're looking for
        let target_char = self.get_tile(start)?;

        let mut queue = vec![start];
        let mut perimeter = 0;
        let mut corners = 0;
        let mut count = 0;

        while let Some(pos) = queue.pop() {
            perimeter += self.count_edges(pos);
            corners += self.count_corners(pos);
            count += 1;

            for &(dx, dy) in &DIRECTIONS {
                let new_pos = (pos.0 + dx, pos.1 + dy);
                if self.get_tile(new_pos) == Some(target_char) && visited.insert(new_pos) {
                    queue.push(new_pos);
                }
            }
        }

        Some((count, perimeter, corners))
    }
}

pub fn solution(input: &str) {
    let map = Map::new(input).unwrap();

    let mut part1 = 0;
    let mut part2 = 0;

    let mut visited = HashSet::new();
    for y in 0..map.height {
        for x in 0..map.width {
            let pos = (x as isize, y as isize);
            if let Some((c, p, cr)) = map.explore_area(pos, &mut visited) {
                part1 += c * p;
                part2 += c * cr;
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
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

    #[test]
    fn test_count_corners() {
        let input = "###\n#.#\n###";
        let map = Map::new(input).unwrap();

        assert_eq!(map.count_corners((0, 0)), 2);
        assert_eq!(map.count_corners((1, 0)), 0);
        assert_eq!(map.count_corners((1, 1)), 4);
    }

    #[test]
    fn test_explore_area() {
        let input = "###\n#.#\n###";
        let map = Map::new(input).unwrap();

        let mut visited = HashSet::new();
        assert_eq!(map.explore_area((0, 0), &mut visited), Some((8, 16, 8)));
        assert_eq!(map.explore_area((1, 1), &mut visited), Some((1, 4, 4)));
    }
}
