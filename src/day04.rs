use std::convert::TryFrom;

#[derive(Debug)]
struct Grid {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Grid {
    fn new(input: &str) -> Option<Self> {
        let chars = input.lines().flat_map(|line| line.chars()).collect();
        let width = input.lines().next()?.chars().count();
        let height = input.lines().count();

        Some(Self {
            chars,
            width,
            height,
        })
    }

    fn get(&self, Point { x, y }: Point) -> Option<char> {
        if self.is_valid_point(Point { x, y }) {
            Some(self.chars[y as usize * self.width + x as usize])
        } else {
            None
        }
    }

    fn is_valid_point(&self, Point { x, y }: Point) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height as i32)
            .flat_map(move |y| (0..self.width as i32).map(move |x| Point { x, y }))
    }
}

struct WordSearch<'a> {
    grid: &'a Grid,
}

impl<'a> WordSearch<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self { grid }
    }

    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (0, -1),
        (-1, 0),
        (-1, -1),
        (-1, 1),
    ];

    fn search_from(&self, start: Point, (dx, dy): (i32, i32), target: &[char]) -> bool {
        target.iter().enumerate().all(|(i, &c)| {
            let point = Point {
                x: start.x + dx * i32::try_from(i).unwrap(),
                y: start.y + dy * i32::try_from(i).unwrap(),
            };
            self.grid.get(point) == Some(c)
        })
    }

    fn count(&self, target: &[char]) -> usize {
        self.grid
            .points()
            .flat_map(|point| {
                Self::DIRECTIONS
                    .iter()
                    .filter(move |&&dir| self.search_from(point, dir, target))
            })
            .count()
    }
}

struct XMasSearch<'a> {
    grid: &'a Grid,
}

impl<'a> XMasSearch<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self { grid }
    }

    const PATTERNS: [[(i32, i32); 3]; 4] = [
        [(-1, -1), (0, 0), (1, 1)],
        [(1, -1), (0, 0), (-1, 1)],
        [(1, 1), (0, 0), (-1, -1)],
        [(-1, 1), (0, 0), (1, -1)],
    ];

    const TARGET: [char; 3] = ['M', 'A', 'S'];

    fn is_match(&self, center: Point) -> bool {
        if self.grid.get(center) != Some('A') {
            return false;
        }

        let matching_patterns = Self::PATTERNS
            .iter()
            .filter(|pattern| {
                pattern.iter().zip(&Self::TARGET).all(|(&(dx, dy), &c)| {
                    let point = Point {
                        x: center.x + dx,
                        y: center.y + dy,
                    };
                    self.grid.get(point) == Some(c)
                })
            })
            .count();

        matching_patterns == 2
    }

    fn count(&self) -> usize {
        self.grid
            .points()
            .filter(|&point| self.is_match(point))
            .count()
    }
}

pub fn solution(input: &str) {
    let grid = Grid::new(input).unwrap();

    let word_search = WordSearch::new(&grid);
    let part1 = word_search.count(&['X', 'M', 'A', 'S']);
    println!("Part 1: {}", part1);

    let xmas_search = XMasSearch::new(&grid);
    let part2 = xmas_search.count();
    println!("Part 2: {}", part2);
}
