use ::num::integer;
use regex::Regex;
use std::{collections::HashMap, fs, str::Chars};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {}", part1(input.clone()));
    println!("part2 {}", part2(input));
}

type Input<'a> = (Chars<'a>, HashMap<&'a str, (&'a str, &'a str)>);

fn parse_input(input: &str) -> Input {
    let mut splitted_line = input.split("\n\n");

    let moves = splitted_line.next().unwrap().chars();

    let mut map = HashMap::new();
    let re = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
    for (_, [key, left, right]) in re
        .captures_iter(splitted_line.next().unwrap())
        .map(|c| c.extract())
    {
        map.insert(key, (left, right));
    }
    (moves, map)
}

fn part1((moves, map): Input) -> usize {
    let mut moves = moves.cycle();
    let mut key = "AAA";
    let mut i = 0;

    while key != "ZZZ" {
        key = match moves.next().unwrap() {
            'L' => map.get(key).unwrap().0,
            'R' => map.get(key).unwrap().1,
            _ => panic!(),
        };
        i += 1;
    }

    i
}

fn part2((moves, map): Input) -> usize {
    let r = map
        .keys()
        .filter_map(|key| if key.ends_with('A') { Some(*key) } else { None })
        .map(|mut key| {
            let mut moves = moves.clone().cycle();
            let mut i = 0;

            while !key.ends_with('Z') {
                key = match moves.next().unwrap() {
                    'L' => map.get(key).unwrap().0,
                    'R' => map.get(key).unwrap().1,
                    _ => panic!(),
                };
                i += 1;
            }

            i
        })
        .collect::<Vec<_>>();
    r.iter().fold(r[0], |acc, v| integer::lcm(acc, *v))
}
