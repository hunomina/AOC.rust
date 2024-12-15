use core::panic;
use std::fs;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            '<' => Self::Left,
            'v' => Self::Down,
            _ => panic!(),
        }
    }
}

impl Direction {
    fn to_point(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

const BOX: char = 'O';
const ROBOT: char = '@';
const WALL: char = '#';

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Map = Vec<Vec<Option<char>>>;
type Input = (Map, Vec<Direction>);

fn parse_input(input: &str) -> Input {
    let mut split = input.split("\n\n");

    let map = split
        .next()
        .unwrap()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| if c == '.' { None } else { Some(c) })
                .collect()
        })
        .collect();

    let directions = split
        .next()
        .unwrap()
        .lines()
        .flat_map(|line| line.chars().map(Direction::from))
        .collect();

    (map, directions)
}

fn part1(input: Input) -> usize {
    let (mut map, directions) = input;
    let mut robot_position = find_robot_position(&map);

    for direction in directions {
        let neighbours =
            get_neighbours_until_empty_space_or_wall(&map.clone(), &robot_position, &direction);

        if neighbours.is_empty() {
            continue;
        }

        let first_neighbour = neighbours.first().unwrap();
        if neighbours.len() == 1 && first_neighbour.1.is_none() {
            map[first_neighbour.0 .0][first_neighbour.0 .1] = Some(ROBOT);
            map[robot_position.0][robot_position.1] = None;
            robot_position = first_neighbour.0;

            continue;
        }

        let last_neighbour = neighbours.last().unwrap();

        if last_neighbour.1.is_none() {
            map[last_neighbour.0 .0][last_neighbour.0 .1] = Some(BOX);
            map[first_neighbour.0 .0][first_neighbour.0 .1] = Some(ROBOT);
            map[robot_position.0][robot_position.1] = None;
            robot_position = first_neighbour.0;
        }
    }

    map.into_iter().enumerate().fold(0, |acc, (i, line)| {
        acc + line.into_iter().enumerate().fold(0, |acc, (j, cell)| {
            if cell != Some(BOX) {
                acc
            } else {
                acc + i * 100 + j
            }
        })
    })
}

fn part2(input: Input) {

    let (mut map, directions) = input;
    map = transform_map_for_p2(map);

    print_map(&map);
}

fn find_robot_position(map: &Map) -> (usize, usize) {
    for (x, line) in map.iter().enumerate() {
        for (y, cell) in line.iter().enumerate() {
            if *cell == Some(ROBOT) {
                return (x, y);
            }
        }
    }

    panic!("robot is missing");
}

fn get_neighbours_until_empty_space_or_wall(
    map: &Map,
    position: &(usize, usize),
    direction: &Direction,
) -> Vec<((usize, usize), Option<char>)> {
    let map_size = (map.len() as isize, map[0].len() as isize);
    let direction_point = direction.to_point();

    let mut neighbours = vec![];
    let mut next_neighbour_position = (position.0 as isize, position.1 as isize);

    loop {
        next_neighbour_position = (
            next_neighbour_position.0 + direction_point.0,
            next_neighbour_position.1 + direction_point.1,
        );

        if next_neighbour_position.0 > 0
            && next_neighbour_position.1 > 0
            && next_neighbour_position.0 < map_size.0
            && next_neighbour_position.1 < map_size.1
        {
            let next_neighbour = *map
                .get(next_neighbour_position.0 as usize)
                .unwrap()
                .get(next_neighbour_position.1 as usize)
                .unwrap();

            neighbours.push((
                (
                    next_neighbour_position.0 as usize,
                    next_neighbour_position.1 as usize,
                ),
                next_neighbour,
            ));

            if next_neighbour.is_none() || next_neighbour == Some(WALL) {
                break;
            }
        } else {
            break;
        }
    }

    neighbours
}

fn print_map(map: &Map) {
    for line in map {
        for cell in line {
            print!(
                "{}",
                match *cell {
                    Some(v) => v,
                    None => '.',
                }
            );
        }
        println!()
    }
}

fn transform_map_for_p2(map: Map) -> Map {
    let mut new_map = vec![vec![]; map.len()];
    for (x, line) in map.into_iter().enumerate() {
        for (_, cell) in line.into_iter().enumerate() {
            match cell {
                None => {
                    new_map[x].push(None);
                    new_map[x].push(None);
                }
                Some(c) => match c {
                    '#' => {
                        new_map[x].push(Some('#'));
                        new_map[x].push(Some('#'));
                    }
                    '@' => {
                        new_map[x].push(Some('@'));
                        new_map[x].push(None);
                    }
                    'O' => {
                        new_map[x].push(Some('['));
                        new_map[x].push(Some(']'));
                    }
                    _ => {}
                },
            }
        }
    }

    new_map
}
