use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {}", part1(input.clone()));
    println!("part2 {}", part2(input));
}

type Input = Vec<(HashSet<u32>, HashSet<u32>)>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut x = line.split(": ").nth(1).unwrap().split(" | ");
            (
                x.next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
                x.next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn part1(input: Input) -> u32 {
    input
        .into_iter()
        .map(|(wining_numbers, numbers)| {
            let intersection = wining_numbers.intersection(&numbers).collect::<Vec<_>>();

            if intersection.is_empty() {
                return 0;
            }

            2u32.pow(intersection.len() as u32 - 1)
        })
        .sum()
}

fn part2(input: Input) -> u32 {
    let mut map = HashMap::new();
    for (id, _) in input.iter().enumerate() {
        map.insert(id + 1, 1); // key: card id, value: number of occurences
    }

    input
        .into_iter()
        .enumerate()
        .for_each(|(index, (wining_numbers, numbers))| {
            let index = index + 1;
            let intersection = wining_numbers.intersection(&numbers).collect::<Vec<_>>();

            let number_of_current_card = *map.get(&index).unwrap();
            for i in 1..intersection.len() + 1 {
                let current_value = map[&(index + i)];
                let v = map.get_mut(&(index + i)).unwrap();
                *v = current_value + number_of_current_card;
            }
        });

    map.into_iter().fold(0, |acc, (_, n)| acc + n)
}
