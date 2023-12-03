use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {}", part1(input.clone()));
    println!("part2 {}", part2(input));
}

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part1(input: Input) -> u64 {
    let mut all_part_numbers = vec![];

    for line_index in 0..input.len() {
        let mut r = vec![];
        let mut is_part_number = false;
        for column_index in 0..input[line_index].len() {
            if !input[line_index][column_index].is_ascii_digit() {
                if is_part_number && !r.is_empty() {
                    r.reverse();
                    all_part_numbers.push(
                        r.into_iter()
                            .enumerate()
                            .fold(0u64, |acc, (i, n)| acc + (n as u64 * 10u64.pow(i as u32))),
                    );
                    is_part_number = false;
                }
                r = vec![];
                continue;
            }

            r.push(input[line_index][column_index].to_digit(10).unwrap()); // add current
            if is_near_special_char(&input, (line_index, column_index)) {
                is_part_number = true;
            }
        }

        if is_part_number && !r.is_empty() {
            r.reverse();
            all_part_numbers.push(
                r.into_iter()
                    .enumerate()
                    .fold(0u64, |acc, (i, n)| acc + (n as u64 * 10u64.pow(i as u32))),
            );
        }
    }

    all_part_numbers.into_iter().sum()
}

fn part2(input: Input) -> u64 {
    let mut all_part_numbers = vec![];

    for line_index in 0..input.len() {
        let mut r = vec![];
        let mut close_gear = None;
        for column_index in 0..input[line_index].len() {
            // is current position a number ?
            if !input[line_index][column_index].is_ascii_digit() {
                let mut r_clone = r.clone();
                r_clone.reverse();
                if close_gear.is_some() && !r_clone.is_empty() {
                    all_part_numbers.push((
                        close_gear.unwrap(),
                        r_clone
                            .into_iter()
                            .enumerate()
                            .fold(0u64, |acc, (i, n)| acc + (n as u64 * 10u64.pow(i as u32))),
                    ));
                    close_gear = None;
                }
                r = vec![];
                continue;
            }

            r.push(input[line_index][column_index].to_digit(10).unwrap()); // add current
            if close_gear.is_none() {
                let gear = has_gear_close(&input, (line_index, column_index));
                if gear.is_some() {
                    close_gear = gear;
                }
            }
        }

        if close_gear.is_some() && !r.is_empty() {
            r.reverse();
            all_part_numbers.push((
                close_gear.unwrap(),
                r.into_iter()
                    .enumerate()
                    .fold(0u64, |acc, (i, n)| acc + (n as u64 * 10u64.pow(i as u32))),
            ));
        }
    }

    all_part_numbers
        .iter()
        .enumerate()
        .fold(0, |acc, (index, (gear_position, number))| {
            acc + number
                * all_part_numbers[index + 1..]
                    .iter()
                    .find(|(other_gear_position, _)| gear_position == other_gear_position)
                    .unwrap_or(&((0, 0), 0))
                    .1
        })
}

fn is_near_special_char(input: &Input, position: (usize, usize)) -> bool {
    get_neighbours(position)
        .into_iter()
        .map(|(line, column)| {
            *input
                .get(line)
                .unwrap_or(&vec![])
                .get(column)
                .unwrap_or(&'.')
        })
        .any(|c| !c.is_ascii_digit() && c != '.')
}

fn has_gear_close(input: &Input, position: (usize, usize)) -> Option<(usize, usize)> {
    get_neighbours(position).into_iter().find(|(line, column)| {
        *input
            .get(*line)
            .unwrap_or(&vec![])
            .get(*column)
            .unwrap_or(&'.')
            == '*'
    })
}

fn get_neighbours(position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![
        (position.0, position.1 + 1),
        (position.0 + 1, position.1),
        (position.0 + 1, position.1 + 1),
    ];
    if position.0 > 0 {
        if position.1 > 0 {
            neighbours.push((position.0 - 1, position.1 - 1))
        }

        neighbours.push((position.0 - 1, position.1));
        neighbours.push((position.0 - 1, position.1 + 1));
    }

    if position.1 > 0 {
        neighbours.push((position.0, position.1 - 1));
        neighbours.push((position.0 + 1, position.1 - 1));
    }

    neighbours
}
