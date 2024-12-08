use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: Input) -> usize {
    let field_dimensions = (input.len(), input[0].len());
    let map = map_antennas(input);

    let antinodes: HashMap<char, Vec<(i32, i32)>> = find_antinodes(map)
        .into_iter()
        .map(|(c, antinodes)| {
            (
                c,
                antinodes
                    .into_iter()
                    .filter(|antinode| {
                        antinode.0 < field_dimensions.0 as i32
                            && antinode.1 < field_dimensions.1 as i32
                    })
                    .collect(),
            )
        })
        .collect();

    antinodes.into_values().flatten().count()
}

fn part2(input: Input) -> usize {
    let field_dimensions = (input.len(), input[0].len());

    let map = map_antennas(input);

    find_antinodes_part_2(map, field_dimensions)
        .into_values()
        .flatten()
        .collect::<HashSet<_>>()
        .len()
}

fn map_antennas(input: Input) -> HashMap<char, Vec<(i32, i32)>> {
    let mut h = HashMap::new();

    for (i, line) in input.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            if *cell == '.' {
                continue;
            }

            h.entry(*cell).or_insert(vec![]).push((i as i32, j as i32));
        }
    }

    h
}

fn find_antinodes(map: HashMap<char, Vec<(i32, i32)>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut h = HashMap::new();

    for (antenna, positions) in map.into_iter() {
        let antinodes_for_antenna = positions
            .iter()
            .flat_map(|current_position| {
                let mut antinodes = vec![];

                for position in positions.iter() {
                    if position == current_position {
                        continue;
                    }

                    let distance = (
                        if position.0 > current_position.0 {
                            position.0 - current_position.0
                        } else {
                            current_position.0 - position.0
                        },
                        if position.1 > current_position.1 {
                            position.1 - current_position.1
                        } else {
                            current_position.1 - position.1
                        },
                    );

                    if current_position.0 < position.0 && current_position.0 >= distance.0 {
                        if current_position.1 < position.1 && current_position.1 >= distance.1 {
                            antinodes.push((
                                current_position.0 - distance.0,
                                current_position.1 - distance.1,
                            ));
                        } else if current_position.1 > position.1 {
                            antinodes.push((
                                current_position.0 - distance.0,
                                current_position.1 + distance.1,
                            ));
                        }
                    } else if current_position.1 < position.1 && current_position.1 >= distance.1 {
                        antinodes.push((position.0 + distance.0, position.1 - distance.1));
                    } else if current_position.1 > position.1 {
                        antinodes.push((position.0 + distance.0, position.1 + distance.1));
                    };
                }

                antinodes
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        h.insert(antenna, antinodes_for_antenna);
    }

    h
}

fn find_antinodes_part_2(
    map: HashMap<char, Vec<(i32, i32)>>,
    field_size: (usize, usize),
) -> HashMap<char, Vec<(i32, i32)>> {
    let mut h = HashMap::new();

    for (antenna, positions) in map.into_iter() {
        let antinodes_for_antenna = positions
            .iter()
            .flat_map(|current_position| {
                let mut antinodes: Vec<(i32, i32)> = vec![];

                for position in positions.iter() {
                    if position == current_position {
                        continue;
                    }

                    let distance = (
                        current_position.0 - position.0,
                        current_position.1 - position.1,
                    );

                    let (mut left, mut right) = (vec![*current_position], vec![*position]);

                    let mut previous_left = *left.last().unwrap();
                    let should_exit_left = |previous: (i32, i32)| {
                        previous.0 - distance.0 >= field_size.0 as i32
                            || previous.0 - distance.0 < 0
                            || previous.1 - distance.1 >= field_size.1 as i32
                            || previous.1 - distance.1 < 0
                    };

                    while !should_exit_left(previous_left) {
                        let new = (previous_left.0 - distance.0, previous_left.1 - distance.1);
                        left.push(new);
                        previous_left = new;
                    }

                    let mut previous_right = *right.last().unwrap();
                    let should_exit_right = |previous: (i32, i32)| {
                        previous.0 + distance.0 >= field_size.0 as i32
                            || previous.0 + distance.0 < 0
                            || previous.1 + distance.1 >= field_size.1 as i32
                            || previous.1 + distance.1 < 0
                    };

                    while !should_exit_right(previous_right) {
                        let new = (previous_right.0 + distance.0, previous_right.1 + distance.1);
                        right.push(new);
                        previous_right = new;
                    }

                    antinodes.append(&mut left);
                    antinodes.append(&mut right);
                }

                antinodes
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        h.insert(antenna, antinodes_for_antenna);
    }

    h
}
