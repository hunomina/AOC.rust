use std::{collections::HashMap, fs};

type Position = (usize, usize);

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let board = parse_input(&input);

    println!("part1 result {}", part1(board.clone()));
    println!("part2 result {}", part2(board.clone()));
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| u8::from_str_radix(c.to_string().as_str(), 10).unwrap())
                .collect()
        })
        .collect()
}

fn part1(board: Vec<Vec<u8>>) -> u32 {
    find_shortest_distance(board, (0, 0))
}

fn part2(board: Vec<Vec<u8>>) -> u32 {
    find_shortest_distance(build_board_for_part2(board), (0, 0))
}

fn find_shortest_distance(board: Vec<Vec<u8>>, from: Position) -> u32 {
    let mut distances: HashMap<Position, u32> = HashMap::from([(from, 0)]);
    let mut to_visit = vec![from];
    while to_visit.len() > 0 {
        let current = to_visit.remove(0);
        let current_distance = *distances.get(&current).unwrap();

        get_neighbors(&board, &current)
            .into_iter()
            .for_each(|neighbor| {
                let distance_to_neighbor = current_distance + board[neighbor.0][neighbor.1] as u32;

                // u32::MAX here serves as Infinity
                if distance_to_neighbor < *distances.get(&neighbor).unwrap_or(&std::u32::MAX) {
                    to_visit.push(neighbor);
                    distances.insert(neighbor, distance_to_neighbor);
                }
            });
    }

    *distances
        .get(&(board.len() - 1, board[0].len() - 1))
        .unwrap()
}

fn build_board_for_part2(mut board: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let board_original_size = (board.len(), board[0].len());

    (1..5).for_each(|i| {
        (0..board_original_size.0).for_each(|line| {
            (0..board_original_size.1).for_each(|column| {
                let cell = board[line][column];
                board[line].push(compute_risk_level(&cell, i));
            });
        })
    });

    (1..5).for_each(|i| {
        (0..board_original_size.0)
            .for_each(|line| board.push(compute_line_risk_level(&board[line], i)));
    });

    board
}

fn compute_risk_level(from: &u8, add: u8) -> u8 {
    if from + add > 9 {
        (from + add) % 9
    } else {
        from + add
    }
}

fn compute_line_risk_level(line: &Vec<u8>, add: u8) -> Vec<u8> {
    line.iter()
        .map(|cell| compute_risk_level(cell, add))
        .collect()
}

fn get_neighbors(board: &Vec<Vec<u8>>, current: &Position) -> Vec<Position> {
    let mut neighbors = vec![];

    if current.0 > 0 {
        neighbors.push((current.0 - 1, current.1));
    }

    if current.1 > 0 {
        neighbors.push((current.0, current.1 - 1));
    }

    if let Some(_) = board.get(current.0 + 1) {
        neighbors.push((current.0 + 1, current.1));
    }

    if let Some(_) = board[current.0].get(current.1 + 1) {
        neighbors.push((current.0, current.1 + 1));
    }

    neighbors
}
