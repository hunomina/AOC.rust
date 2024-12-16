use core::panic;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
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
const L_BOX: char = '[';
const R_BOX: char = ']';

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
    let (map, directions) = input;
    solve(map, directions)
}

fn part2(input: Input) -> usize {
    let (mut map, directions) = input;
    map = transform_map_for_p2(map);

    solve(map, directions)
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

fn solve(mut map: Map, directions: Vec<Direction>) -> usize {
    let mut robot_position = find_robot_position(&map);

    for direction in directions {
        let (can_move, movable_neighbours) = match direction {
            Direction::Up | Direction::Down => {
                can_move_up_or_down(&map, &robot_position, &direction)
            }
            Direction::Left | Direction::Right => {
                can_move_left_or_right(&map, &robot_position, &direction)
            }
        };

        if !can_move {
            continue;
        }

        let direction_point = direction.to_point();
        for movable_neighbour in movable_neighbours {
            let destination = (
                (movable_neighbour.0 as isize + direction_point.0) as usize,
                (movable_neighbour.1 as isize + direction_point.1) as usize,
            );

            map[destination.0][destination.1] =
                Some(map[movable_neighbour.0][movable_neighbour.1].unwrap());
            map[movable_neighbour.0][movable_neighbour.1] = None;
        }

        map[robot_position.0][robot_position.1] = None;
        robot_position = (
            (robot_position.0 as isize + direction_point.0) as usize,
            (robot_position.1 as isize + direction_point.1) as usize,
        );
        map[robot_position.0][robot_position.1] = Some(ROBOT);
    }

    map.into_iter().enumerate().fold(0, |acc, (i, line)| {
        acc + line.into_iter().enumerate().fold(0, |acc, (j, cell)| {
            acc + if cell != Some(BOX) && cell != Some(L_BOX) {
                0
            } else {
                i * 100 + j
            }
        })
    })
}

fn can_move_left_or_right(
    map: &Map,
    position: &(usize, usize),
    direction: &Direction,
) -> (bool, Vec<(usize, usize)>) {
    let map_size = (map.len() as isize, map[0].len() as isize);
    let direction_point = direction.to_point();
    let mut next_neighbour_position = (position.0 as isize, position.1 as isize);
    let mut next_non_box_neighbours = vec![];
    loop {
        next_neighbour_position = (
            next_neighbour_position.0 + direction_point.0,
            next_neighbour_position.1 + direction_point.1,
        );

        if !(next_neighbour_position.0 > 0
            && next_neighbour_position.1 > 0
            && next_neighbour_position.0 < map_size.0
            && next_neighbour_position.1 < map_size.1)
        {
            return (false, vec![]);
        }

        let next_neighbour = *map
            .get(next_neighbour_position.0 as usize)
            .unwrap()
            .get(next_neighbour_position.1 as usize)
            .unwrap();

        if next_neighbour == Some(BOX)
            || next_neighbour == Some(L_BOX)
            || next_neighbour == Some(R_BOX)
        {
            next_non_box_neighbours.push((
                next_neighbour_position.0 as usize,
                next_neighbour_position.1 as usize,
            ));
        } else if next_neighbour.is_none() {
            next_non_box_neighbours.reverse();
            return (true, next_non_box_neighbours);
        } else {
            return (false, vec![]);
        }
    }
}

// can a box be moved and also which boxes does it move as well ?
fn can_move_up_or_down(
    map: &Map,
    position: &(usize, usize),
    direction: &Direction,
) -> (bool, Vec<(usize, usize)>) {
    let map_size = (map.len() as isize, map[0].len() as isize);
    let direction_point = direction.to_point();

    let next_neighbour_position = (
        position.0 as isize + direction_point.0,
        position.1 as isize + direction_point.1,
    );

    if !(next_neighbour_position.0 > 0
        && next_neighbour_position.1 > 0
        && next_neighbour_position.0 < map_size.0
        && next_neighbour_position.1 < map_size.1)
    {
        // out of bound
        return (false, vec![]);
    }

    let next_neighbour_position = (
        next_neighbour_position.0 as usize,
        next_neighbour_position.1 as usize,
    );

    let next_neighbour = *map
        .get(next_neighbour_position.0)
        .unwrap()
        .get(next_neighbour_position.1)
        .unwrap();

    match next_neighbour {
        Some(v) => match v {
            BOX => match can_move_up_or_down(map, &next_neighbour_position, direction) {
                (true, other_boxes_to_move) => (
                    true,
                    other_boxes_to_move
                        .into_iter()
                        .chain(vec![next_neighbour_position])
                        .collect(),
                ),
                (false, _) => (false, vec![]),
            },
            L_BOX => {
                let can_l_box_moves = can_move_up_or_down(map, &next_neighbour_position, direction);
                let can_r_box_moves = can_move_up_or_down(
                    map,
                    &(next_neighbour_position.0, next_neighbour_position.1 + 1),
                    direction,
                );

                if can_l_box_moves.0 && can_r_box_moves.0 {
                    let mut movable_neighbours = can_l_box_moves.1;
                    movable_neighbours = merge_missing(movable_neighbours, can_r_box_moves.1);
                    movable_neighbours = merge_missing(
                        movable_neighbours,
                        vec![
                            next_neighbour_position,
                            (next_neighbour_position.0, next_neighbour_position.1 + 1),
                        ],
                    );

                    (true, movable_neighbours)
                } else {
                    (false, vec![])
                }
            }
            R_BOX => {
                let can_l_box_moves = can_move_up_or_down(
                    map,
                    &(next_neighbour_position.0, next_neighbour_position.1 - 1),
                    direction,
                );
                let can_r_box_moves = can_move_up_or_down(map, &next_neighbour_position, direction);

                if can_l_box_moves.0 && can_r_box_moves.0 {
                    let mut movable_neighbours = can_l_box_moves.1;
                    movable_neighbours = merge_missing(movable_neighbours, can_r_box_moves.1);
                    movable_neighbours = merge_missing(
                        movable_neighbours,
                        vec![
                            next_neighbour_position,
                            (next_neighbour_position.0, next_neighbour_position.1 - 1),
                        ],
                    );

                    (true, movable_neighbours)
                } else {
                    (false, vec![])
                }
            }
            _ => (false, vec![]),
        },
        None => (true, vec![]),
    }
}

fn transform_map_for_p2(map: Map) -> Map {
    let mut new_map = vec![vec![]; map.len()];
    for (x, line) in map.into_iter().enumerate() {
        for cell in line.into_iter() {
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

// once neighbours have been identified for a given position
// deduplicate neighbours that needs to be moved
// duplicates on neighbours to move can happen when two boxes are on top of each other
fn merge_missing(mut a: Vec<(usize, usize)>, b: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    for item in b.into_iter() {
        if !a.contains(&item) {
            a.push(item);
        }
    }

    a
}
