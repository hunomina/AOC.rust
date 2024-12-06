use std::{collections::HashSet, fs};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn into_movement(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: Input) -> usize {
    get_circuit(find_initial_position(&input), &input)
        .unwrap()
        .len()
}

fn part2(mut input: Input) -> usize {
    let initial_position = find_initial_position(&input);

    get_circuit(initial_position, &input)
        .unwrap()
        .into_iter()
        .fold(0, |mut acc, (x, y)| {
            let cell = input[x][y];
            if cell != '.' {
                return acc;
            }

            input[x][y] = '#';

            if get_circuit(initial_position, &input).is_none() {
                acc += 1;
            }

            input[x][y] = '.';
            acc
        })
}

fn get_circuit(
    (mut x, mut y, mut direction): (usize, usize, Direction),
    input: &Input,
) -> Option<HashSet<(usize, usize)>> {
    let mut history = HashSet::new();

    while x != 0 && x != input.len() - 1 && y != 0 && y != input[0].len() - 1 {
        let next_position = (
            (x as i32 + direction.into_movement().0) as usize,
            (y as i32 + direction.into_movement().1) as usize,
        );

        if input[next_position.0][next_position.1] == '#' {
            direction = direction.next();
            continue;
        }

        let new_cell = (next_position.0, next_position.1, direction);
        if history.contains(&new_cell) {
            return None; // loop detected
        }

        history.insert(new_cell);
        x = next_position.0;
        y = next_position.1;
    }

    Some(history.into_iter().map(|(x, y, _)| (x, y)).collect())
}

fn find_initial_position(map: &Input) -> (usize, usize, Direction) {
    for (i, line) in map.iter().enumerate() {
        for (j, cell) in line.iter().enumerate() {
            if *cell == '^' {
                return (i, j, Direction::Up);
            }
        }
    }

    panic!()
}
