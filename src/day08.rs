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
    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;

    let mut antinodes = HashSet::new();

    // For every unique pair of antennas
    for (i, &(a_freq, (ax, ay))) in antennas.iter().enumerate() {
        for &(b_freq, (bx, by)) in antennas.iter().skip(i + 1) {
            // Must be same "frequency"
            if a_freq != b_freq {
                continue;
            }

            // Calculate the step size
            let dx = bx - ax;
            let dy = by - ay;

            let gen_antinodes = |start_x, start_y, dx, dy| {
                (0..)
                    .map(move |i| {
                        let nx = start_x + dx * i;
                        let ny = start_y + dy * i;
                        (i, (nx, ny))
                    })
                    .take_while(|&(_, (nx, ny))| nx >= 0 && ny >= 0 && nx < width && ny < height)
            };

            antinodes.extend(gen_antinodes(bx, by, dx, dy));
            antinodes.extend(gen_antinodes(ax, ay, -dx, -dy));
        }
    }

    let first_order: HashSet<_> = antinodes
        .iter()
        .filter(|&&(order, _)| order == 1)
        .map(|&(_, pos)| pos)
        .collect();
    println!("Part 1: {}", first_order.len());

    let all_orders: HashSet<_> = antinodes.iter().map(|&(_, pos)| pos).collect();
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
