use std::collections::{HashSet, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solution(input: &str) {
    let (map, directions) = input.split_once("\n\n").unwrap();

    let width = map.find('\n').unwrap() as i32;
    let pos = map.find('@').unwrap() as i32;

    let height = (map.len() as i32 + 1) / (width + 1);
    let x = pos % (width + 1);
    let y = pos / (width + 1);

    let mut map1 = Vec::new();
    let mut map2 = Vec::new();

    (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .for_each(|(x, y)| {
            match map.as_bytes()[(y * (width + 1) + x) as usize] {
                b'O' => {
                    map1.push(b'[');
                    map2.extend([b'[', b']']);
                }
                b => {
                    map1.push(b);
                    map2.extend([b; 2]);
                }
            };
        });

    let mut q = VecDeque::new();
    let mut set = HashSet::new();

    let dirs: Vec<_> = directions
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '^' => DIRECTIONS[0],
            '>' => DIRECTIONS[1],
            'v' => DIRECTIONS[2],
            '<' => DIRECTIONS[3],
            _ => panic!("Invalid direction: {}", c),
        })
        .collect();

    let mut solve = |mut map: Vec<u8>, w, mut x, mut y, s| {
        'outer: for &(dx, dy) in &dirs {
            q.clear();
            set.clear();
            q.push_back([x, y]);

            while let Some((x, y, x2, y2)) = q.pop_front().map(|[x, y]| (x, y, x + dx, y + dy)) {
                if set.insert([x, y]) {
                    match map[(y2 * w + x2) as usize] {
                        b'#' => continue 'outer,
                        b'[' => q.extend(&[[x2, y2], [x2 + 1, y2]][..s]),
                        b']' => q.extend([[x2, y2], [x2 - 1, y2]]),
                        _ => {}
                    }
                }
            }

            while !set.is_empty() {
                let items: Vec<_> = set.iter().cloned().collect();
                for [x, y] in items {
                    if !set.contains(&[x + dx, y + dy]) {
                        map.swap((y * w + x) as usize, ((y + dy) * w + x + dx) as usize);
                        set.remove(&[x, y]);
                    }
                }
            }

            (x, y) = (x + dx, y + dy);
        }

        let str = std::str::from_utf8(&map).unwrap();
        for y in 0..height {
            println!("{}", &str[(y * w) as usize..(y * w + w) as usize]);
        }

        map.iter()
            .enumerate()
            .map(|(i, &c)| i64::from(c == b'[') * i64::from(100 * (i as i32 / w) + i as i32 % w))
            .sum::<i64>()
    };

    println!("Part 1: {}", solve(map1, width, x, y, 1));
    println!("Part 2: {}", solve(map2, width * 2, x * 2, y, 2));
}
