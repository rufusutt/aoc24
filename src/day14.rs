const WIDTH: isize = 101;
const HEIGHT: isize = 103;

#[derive(Debug, Clone)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Robot {
    fn new(line: &str) -> Self {
        let mut parts = line.split(' ');

        let pos = parts.next().unwrap();
        let vel = parts.next().unwrap();

        let extract = |s: &str| -> (isize, isize) {
            let mut iter = s[2..].split(',').map(|x| x.parse::<isize>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        };

        let pos = extract(pos);
        let vel = extract(vel);

        Robot { pos, vel }
    }

    fn step(&mut self, steps: isize) {
        self.pos.0 += self.vel.0 * steps;
        self.pos.1 += self.vel.1 * steps;

        self.pos.0 %= WIDTH;
        self.pos.1 %= HEIGHT;

        if (self.pos.0) < 0 {
            self.pos.0 += WIDTH;
        }
        if (self.pos.1) < 0 {
            self.pos.1 += HEIGHT;
        }
    }

    fn quadrant(&self) -> Option<usize> {
        let x = self.pos.0;
        let y = self.pos.1;

        let centre_x = WIDTH / 2;
        let centre_y = HEIGHT / 2;

        if x < centre_x && y < centre_y {
            Some(0)
        } else if x > centre_x && y < centre_y {
            Some(1)
        } else if x < centre_x && y > centre_y {
            Some(2)
        } else if x > centre_x && y > centre_y {
            Some(3)
        } else {
            None
        }
    }
}

fn print_map(robots: &[Robot]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut found = false;

            for robot in robots {
                if robot.pos == (x, y) {
                    print!("#");
                    found = true;
                    break;
                }
            }

            if !found {
                print!(".");
            }
        }
        println!();
    }
}

pub fn solution(input: &str) {
    let mut robots: Vec<_> = input.lines().map(Robot::new).collect();
    let robot_count = robots.len();

    // Part 1
    let mut quadrants = [0; 4];
    robots.iter().cloned().for_each(|mut robot| {
        robot.step(100);
        if let Some(quadrant) = robot.quadrant() {
            quadrants[quadrant] += 1;
        }
    });

    let part1 = quadrants.iter().product::<usize>();
    println!("Part 1: {}", part1);

    // Part 2
    for i in 1.. {
        let mut count: [usize; 4] = [0; 4];

        robots.iter_mut().for_each(|robot| {
            robot.step(1);

            if let Some(quadrant) = robot.quadrant() {
                count[quadrant] += 1;
            }
        });

        if i % 10000 == 0 {
            println!("Iteration {}", i);
        }

        if count.iter().any(|&x| x >= robot_count / 2) {
            println!("Iteration {}", i);
            print_map(&robots);
            break;
        }
    }
}
