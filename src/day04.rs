struct WordSearch {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

impl WordSearch {
    fn new(input: &str) -> Self {
        let chars = input
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>();
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        WordSearch {
            chars,
            width,
            height,
        }
    }

    fn directions() -> [[i32; 2]; 8] {
        [
            [0, 1],
            [1, 0],
            [1, 1],
            [1, -1],
            [0, -1],
            [-1, 0],
            [-1, -1],
            [-1, 1],
        ]
    }

    fn valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn search_from(&self, x: i32, y: i32, direction: [i32; 2], target: &[char]) -> bool {
        for (i, &c) in target.iter().enumerate() {
            let x = x + direction[0] * i as i32;
            let y = y + direction[1] * i as i32;

            if !self.valid(x, y) {
                return false;
            }

            if self.chars[y as usize * self.width + x as usize] != c {
                return false;
            }
        }
        true
    }

    fn count(&self, target: &[char]) -> usize {
        let mut count = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                for direction in Self::directions().iter() {
                    if self.search_from(x as i32, y as i32, *direction, target) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

struct XMasSearch {
    chars: Vec<char>,
    width: usize,
    height: usize,
}

impl XMasSearch {
    fn new(input: &str) -> Self {
        let chars = input
            .lines()
            .flat_map(|line| line.chars())
            .collect::<Vec<_>>();
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        XMasSearch {
            chars,
            width,
            height,
        }
    }

    fn at(&self, x: i32, y: i32) -> char {
        self.chars[y as usize * self.width + x as usize]
    }

    fn patterns() -> [[[i32; 2]; 3]; 4] {
        // X patters centered on 0,0
        [
            [[-1, -1], [0, 0], [1, 1]],
            [[1, -1], [0, 0], [-1, 1]],
            [[1, 1], [0, 0], [-1, -1]],
            [[-1, 1], [0, 0], [1, -1]],
        ]
    }

    fn is_match(&self, x: i32, y: i32) -> bool {
        let target = ['M', 'A', 'S'];

        // Center letter must be 'A'
        if self.at(x, y) != 'A' {
            return false;
        }

        // Count all matching patterns
        let mut count = 0;
        for pattern in Self::patterns().iter() {
            for ([dx, dy], &c) in pattern.iter().zip(target.iter()) {
                let x = x + dx;
                let y = y + dy;

                if self.at(x, y) != c {
                    break;
                }

                if dx == &pattern[2][0] && dy == &pattern[2][1] {
                    count += 1;
                }
            }
        }
        count >= 2
    }

    fn count(&self) -> usize {
        let mut count = 0;

        for y in 1..self.height as i32 - 1 {
            for x in 1..self.width as i32 - 1 {
                if self.is_match(x, y) {
                    count += 1;
                }
            }
        }

        count
    }
}

pub fn solution(input: &str) {
    let word_search = WordSearch::new(input);
    let target = ['X', 'M', 'A', 'S'];

    let count = word_search.count(&target);

    println!("Part 1: {}", count);

    let xmas_search = XMasSearch::new(input);
    let count = xmas_search.count();

    println!("Part 2: {}", count);
}
