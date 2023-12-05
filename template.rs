use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());
}

type Input = ();

fn parse_input(input: &str) -> Input {}

fn part1(input: Input) {}

fn part2(input: Input) {}
