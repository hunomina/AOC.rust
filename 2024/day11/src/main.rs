use std::{collections::HashMap, fs, vec};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input = Vec<u64>;

fn parse_input(input: &str) -> Input {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(input: Input) -> usize {
    run(input, 25)
}

fn part2(input: Input) -> usize {
    run(input, 75)
}

fn run(input: Input, iteration_count: usize) -> usize {
    let mut map = input.into_iter().map(|n| (n, 1)).collect();

    for _ in 0..iteration_count {
        map = map_count_next_values(map);
    }

    map.into_iter().fold(0, |acc, (_, count)| acc + count)
}

fn map_count_next_values(map: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut n = HashMap::new();

    for (stone, count) in map {
        let next_values = compute_next_value(stone);

        for next_value in next_values {
            let entry = n.get_mut(&next_value);

            if entry.is_none() {
                n.insert(next_value, count);
            } else {
                *entry.unwrap() += count;
            }
        }
    }

    n
}

fn compute_next_value(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![1];
    }

    let o = n.to_string();
    if o.len() % 2 == 0 {
        return vec![
            o[..o.len() / 2].parse().unwrap(),
            o[o.len() / 2..].parse().unwrap(),
        ];
    }

    vec![n * 2024]
}
