use std::fs;

use regex::Regex;

const MOVE_PATTERN: &str = r"move (\d+) from (\d+) to (\d+)";

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let (stacks, moves) = parse_input(&input);

    println!("{}", part1(stacks.clone(), moves.clone()));
    println!("{}", part2(stacks, moves));
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(u8, usize, usize)>) {
    let mut split_input = input.split("\n\n");

    let mut stacks = split_input.next().unwrap().split('\n').collect::<Vec<_>>();

    let number_of_stacks = stacks
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<_>>()
        .len();
    let mut s: Vec<Vec<char>> = vec![vec![]; number_of_stacks];

    stacks.into_iter().for_each(|l| {
        let chars = l.as_bytes().chunks(4).map(|c| c[1] as char).enumerate();
        for (i, c) in chars {
            if c != ' ' {
                s[i].push(c);
            }
        }
    });

    let move_regex = Regex::new(MOVE_PATTERN).unwrap();
    let moves = split_input
        .next()
        .unwrap()
        .split('\n')
        .map(|l| {
            let captures = move_regex.captures(l).unwrap();
            (
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    (s, moves)
}

fn part1(mut stacks: Vec<Vec<char>>, moves: Vec<(u8, usize, usize)>) -> String {
    moves.into_iter().for_each(|(n, from, to)| {
        (0..n).for_each(|_| {
            let f = stacks[from - 1].remove(0);
            stacks[to - 1].insert(0, f);
        })
    });
    stacks
        .into_iter()
        .map(|l| *l.first().unwrap())
        .collect::<String>()
}

fn part2(mut stacks: Vec<Vec<char>>, moves: Vec<(u8, usize, usize)>) -> String {
    moves.into_iter().for_each(|(n, from, to)| {
        let mut to_prepend = (0..n)
            .map(|_| stacks[from - 1].remove(0))
            .collect::<Vec<_>>();

        to_prepend.extend(stacks[to - 1].iter());

        stacks[to - 1] = to_prepend;
    });
    stacks
        .into_iter()
        .map(|l| *l.first().unwrap())
        .collect::<String>()
}
