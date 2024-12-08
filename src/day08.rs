use std::collections::HashSet;

pub fn solution(input: &str) {
    // Collect all antenna locations
    let antennas: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, char)| {
                if char != '.' {
                    Some((char, (x as isize, y as isize)))
                } else {
                    None
                }
            })
        })
        .collect();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut antinodes = HashSet::new();

    // For every unique pair of antennas
    for (i, &(a_freq, (ax, ay))) in antennas.iter().enumerate() {
        for &(b_freq, (bx, by)) in antennas.iter().skip(i + 1) {
            // Must be same "frequency"
            if a_freq != b_freq {
                continue;
            }

            // Calculate the antinode step size
            let dx = bx - ax;
            let dy = by - ay;

            let mut add_antinodes = |start_x, start_y, dx, dy| {
                for i in 1.. {
                    let nx = start_x + dx * i;
                    let ny = start_y + dy * i;

                    if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
                        break;
                    }

                    antinodes.insert((i, (nx, ny)));
                }
            };

            add_antinodes(bx, by, dx, dy);
            add_antinodes(ax, ay, -dx, -dy);
        }
    }

    // Collect all first-order antinodes
    let mut first_order = HashSet::new();
    for &(order, pos) in &antinodes {
        if order == 1 {
            first_order.insert(pos);
        }
    }

    // Collect all orders of antinodes
    let mut all_orders = HashSet::new();
    for &(_, pos) in &antinodes {
        all_orders.insert(pos);
    }

    println!("Part 1: {}", first_order.len());
    println!("Part 2: {}", all_orders.len());

    // Print map with antinodes
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            if all_orders.contains(&(x as isize, y as isize)) {
                print!("#");
            } else {
                print!("{}", char);
            }
        });
        println!();
    });
}
