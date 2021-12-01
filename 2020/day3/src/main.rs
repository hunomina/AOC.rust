// see : https://adventofcode.com/2020/day/3

use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let map = parse_input(input.as_str());

    let vectors: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let trees_encountered = vectors.iter().fold(1 as u64, |acc, vector| {
        let mut line = 0;
        let mut column = 0;
        let mut t = 0;
        loop {
            if map[line][column] == '#' {
                t += 1;
            }
            column = (column + vector.1) % map[line].len();
            line += vector.0;

            if line >= map.len() {
                break;
            }
        }
        println!("{:?} {}", vector, t);
        acc * t
    });

    println!("{}", trees_encountered);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| -> Vec<char> { l.chars().collect() })
        .collect()
}
