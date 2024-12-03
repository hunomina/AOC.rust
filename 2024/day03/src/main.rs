use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());
    println!("part1 {}", part1(input));
    println!("part2 {}", part2(input));
}

type Input<'a> = &'a str;

fn parse_input(input: &str) -> Input {
    input
}

fn part1(input: Input) -> u32 {
    Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))")
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            c.get(2).unwrap().as_str().parse::<u32>().unwrap()
                * c.get(3).unwrap().as_str().parse::<u32>().unwrap()
        })
        .sum()
}

fn part2(input: Input) -> u32 {
    let mut skip = false;

    Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don\'t\(\)))")
        .unwrap()
        .captures_iter(input)
        .map(|c| {
            let command = c.get(0).unwrap().as_str();

            if command.starts_with("do()") {
                skip = false;
                0
            } else if command.starts_with("don't()") {
                skip = true;
                0
            } else if skip {
                0
            } else {
                c.get(2).unwrap().as_str().parse::<u32>().unwrap()
                    * c.get(3).unwrap().as_str().parse::<u32>().unwrap()
            }
        })
        .sum()
}
