use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());
    println!("solution 1: {}", part1(input.clone()));
    println!("solution 2: {}", part2(input.clone()));
}

type Input = Vec<u32>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(input: Input) -> u32 {
    input.into_iter().fold(0, |acc, line| acc + (line / 3) - 2)
}

fn part2(input: Input) -> u32 {
    input.into_iter().fold(0, |acc, line| {
        let mut init = line;
        let mut total_fuel = 0;

        while init > 0 {
            init = (init / 3).max(2) - 2;
            total_fuel += init;
        }

        acc + total_fuel
    })
}
