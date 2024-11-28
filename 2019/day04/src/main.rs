use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input));
    println!("part2 {:?}", part2(input));
}

type Input<'a> = (u32, u32);

fn parse_input(input: &str) -> Input {
    let mut input = input.split('-');
    (
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
    )
}

fn part1(input: Input) -> usize {
    (input.0..=input.1)
        .filter(|n| {
            let validation = validate_password(&n.to_string());
            (
                validation.0,
                validation.1,
                validation.2.into_iter().any(|(_, i)| i > 1),
            ) == (true, true, true)
        })
        .count()
}

fn part2(input: Input) -> usize {
    (input.0..=input.1)
        .filter(|n| {
            let validation = validate_password(&n.to_string());
            (
                validation.0,
                validation.1,
                validation.2.into_iter().any(|(_, i)| i == 2),
            ) == (true, true, true)
        })
        .count()
}

fn validate_password(n: &str) -> (bool, bool, HashMap<char, u32>) {
    (
        n.len() == 6,
        {
            let mut is_valid = true;
            let mut previous = 0;
            for c in n.chars() {
                let current = c.to_digit(10).unwrap();
                if current < previous {
                    is_valid = false;
                    break;
                }
                previous = current;
            }
            is_valid
        },
        {
            let mut map: HashMap<char, u32> = HashMap::new();
            for c in n.chars() {
                *map.entry(c).or_insert(0) += 1;
            }

            map
        },
    )
}
