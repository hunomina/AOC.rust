use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input = Vec<Vec<u32>>;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part1(input: Input) -> usize {
    find_tail_heads(&input)
        .into_iter()
        .map(|trail_head| {
            find_trails(&input, trail_head)
                .into_iter()
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn part2(input: Input) -> usize {
    find_tail_heads(&input)
        .into_iter()
        .map(|trail_head| find_trails(&input, trail_head).len())
        .sum()
}

fn find_tail_heads(input: &Input) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (x, line)| {
            acc.push(line.iter().enumerate().fold(vec![], |mut acc2, (y, cell)| {
                if *cell == 0 {
                    acc2.push((x, y));
                }

                acc2
            }));

            acc
        })
        .into_iter()
        .flatten()
        .collect()
}

fn find_trails(input: &Input, trail_head: (usize, usize)) -> Vec<(usize, usize)> {
    let mut found_picks: Vec<(usize, usize)> = vec![];
    let mut discovered = vec![trail_head];

    while !discovered.is_empty() {
        discovered = discovered
            .into_iter()
            .flat_map(|cell| {
                let mut valid_neighbours = vec![];

                for direction in DIRECTIONS.iter() {
                    let neighbour_position = (
                        (cell.0 as i32 + direction.0) as usize,
                        (cell.1 as i32 + direction.1) as usize,
                    );

                    let stub = vec![];
                    let neighbour_value = input
                        .get(neighbour_position.0)
                        .unwrap_or(&stub)
                        .get(neighbour_position.1);

                    if neighbour_value.is_none() {
                        continue;
                    }

                    let neighbour_value = *neighbour_value.unwrap();

                    if input[cell.0][cell.1] == 8 && neighbour_value == 9 {
                        found_picks.push(neighbour_position);
                    } else if neighbour_value == input[cell.0][cell.1] + 1 {
                        valid_neighbours.push(neighbour_position);
                    }
                }

                valid_neighbours
            })
            .collect();
    }

    found_picks
}
