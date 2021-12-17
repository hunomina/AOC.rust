use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let crabs_positions = parse_input(&input);

    println!("part1 result {}", part1(crabs_positions.clone()));
    println!("part2 result {}", part2(crabs_positions.clone()));
}

fn parse_input(input: &str) -> Vec<u16> {
    input.split(',').map(|i| i.parse().unwrap()).collect()
}

fn part1(mut crabs_positions: Vec<u16>) -> u32 {
    crabs_positions.sort();
    let middle = crabs_positions[crabs_positions.len() / 2];
    crabs_positions.into_iter().fold(0u32, |acc, v| {
        acc + if middle > v { middle - v } else { v - middle } as u32
    })
}

fn part2(crabs_positions: Vec<u16>) -> u32 {
    crabs_positions
        .iter()
        .enumerate()
        .fold(u32::MAX, |acc, (crab_index, crab_position)| {
            let mut local_value = 0;

            (0..crabs_positions.len()).for_each(|i| {
                if i == crab_index {
                    return;
                }

                let (local_min, local_max) = if *crab_position < crabs_positions[i] {
                    (*crab_position, crabs_positions[i])
                } else {
                    (crabs_positions[i], *crab_position)
                };

                local_value += sum(local_min, local_max);
            });

            if local_value < acc {
                local_value
            } else {
                acc
            }
        })
}

#[inline]
fn sum(from: u16, to: u16) -> u32 {
    (1..=to - from).fold(0, |acc, v| acc + v as u32)
}
