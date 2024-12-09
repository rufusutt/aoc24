fn checksum(fs: &[Option<u64>]) -> u64 {
    fs.iter()
        .enumerate()
        .flat_map(|(i, id)| id.as_ref().map(|id| id * i as u64))
        .sum()
}

fn part1(mut fs: Vec<Option<u64>>) {
    let mut left = 0;
    let mut right = fs.len();

    // Move values from right to left
    while left < right {
        if fs[left].is_none() {
            // Find the next non-None value from the right
            while right > left && fs[right - 1].is_none() {
                right -= 1;
            }
            if right > left {
                fs[left] = fs[right - 1];
                fs[right - 1] = None;
                right -= 1;
            }
        }
        left += 1;
    }

    println!("Part 1: {}", checksum(&fs));
}

fn part2(mut fs: Vec<Option<u64>>) {
    // Get unique file IDs in descending order
    let file_ids: Vec<_> = fs
        .iter()
        .flatten()
        .copied()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .rev()
        .collect();

    for &id in &file_ids {
        // Find the file's location and size
        let (file_start, file_size) = fs
            .iter()
            .enumerate()
            .find_map(|(start, &value)| {
                if value == Some(id) {
                    let size = fs[start..].iter().take_while(|&&x| x == Some(id)).count();
                    Some((start, size))
                } else {
                    None
                }
            })
            .unwrap();

        // Find leftmost viable empty space
        let best_slot = (0..file_start)
            .find(|&pos| fs[pos..].iter().take_while(|&&x| x.is_none()).count() >= file_size);

        // Move the file if a viable slot was found
        if let Some(slot_start) = best_slot {
            fs[slot_start..slot_start + file_size].fill(Some(id));
            fs[file_start..file_start + file_size].fill(None);
        }
    }

    println!("Part 2: {}", checksum(&fs));
}

pub fn solution(input: &str) {
    let fs: Vec<_> = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let digit = c.to_digit(10).expect("Input should be digits");
            let is_space = i % 2 == 1;
            let id = (i / 2) as u64;

            std::iter::repeat(if is_space { None } else { Some(id) }).take(digit as usize)
        })
        .collect();

    part1(fs.clone());
    part2(fs);
}
