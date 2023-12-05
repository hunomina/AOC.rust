use std::{fs, thread};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {}", part1(input.clone()));
    println!("part2 {}", part2(input));
}

type Input = (Vec<u64>, Vec<Vec<(u64, u64, u64)>>);

fn parse_input(input: &str) -> Input {
    let mut i = input.split("\n\n");

    let seeds = i
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(&str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let maps = i
        .map(|map| {
            let mut split_map = map.split('\n');
            split_map.next();

            split_map
                .map(|m| {
                    let mut m = m.split_ascii_whitespace();
                    (
                        m.next().unwrap().parse().unwrap(),
                        m.next().unwrap().parse().unwrap(),
                        m.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    (seeds, maps)
}

fn part1((seeds, maps): Input) -> u64 {
    seeds
        .into_iter()
        .map(|seed| {
            maps.iter().fold(seed, |acc, map| {
                let r = map.iter().find_map(
                    |(destination_range_start, source_range_start, range_length)| {
                        if acc >= *source_range_start && acc < *source_range_start + *range_length {
                            Some(destination_range_start + acc - source_range_start)
                        } else {
                            None
                        }
                    },
                );

                r.unwrap_or(acc)
            })
        })
        .min()
        .unwrap()
}

fn part2((seeds, maps): Input) -> u64 {
    let mut handles = vec![];

    for i in (0..seeds.len() - 1).step_by(2) {
        let range = seeds[i]..seeds[i] + seeds[i + 1];
        let maps = maps.clone();
        let handle = thread::spawn(move || {
            range
                .clone()
                .map(|seed| {
                    maps.iter().fold(seed, |acc, map| {
                        let r = map.iter().find_map(
                            |(destination_range_start, source_range_start, range_length)| {
                                if acc >= *source_range_start
                                    && acc < *source_range_start + *range_length
                                {
                                    Some(destination_range_start + acc - source_range_start)
                                } else {
                                    None
                                }
                            },
                        );

                        r.unwrap_or(acc)
                    })
                })
                .min()
                .unwrap()
        });
        handles.push(handle);
    }

    let mut min = u64::MAX;
    for handle in handles {
        let value = handle.join().unwrap();
        if value < min {
            min = value;
        }
    }

    min
}
