const WIDTH: isize = 101;
const HEIGHT: isize = 103;

// const WIDTH: isize = 11;
// const HEIGHT: isize = 7;

#[derive(Debug)]
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
            Some(1)
        } else if x > centre_x && y < centre_y {
            Some(2)
        } else if x < centre_x && y > centre_y {
            Some(3)
        } else if x > centre_x && y > centre_y {
            Some(4)
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
    let mut count: [usize; 4] = [0; 4];

    let mut robots: Vec<_> = input.lines().map(|line| Robot::new(line)).collect();

    robots.iter_mut().for_each(|robot| {
        robot.step(100);

        if let Some(quadrant) = robot.quadrant() {
            count[quadrant - 1] += 1;
        }

        println!("{:?}", robot);
    });

    print_map(&robots);
    dbg!(&count);

    let part1 = count.iter().product::<usize>();
    println!("Part 1: {}", part1);
}
