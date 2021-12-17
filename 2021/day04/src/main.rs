use core::panic;
use std::fs;

// moving a won board to an other vector reduces computation costs
// and avoids data ownership problems for part2
#[derive(Debug, Clone)]
struct Game {
    drawn_numbers: Vec<u8>,
    boards: Vec<Board>,
    won_boards: Vec<Board>,
}

impl Game {
    fn new(drawn_numbers: Vec<u8>, boards: Vec<Board>) -> Self {
        Self {
            drawn_numbers,
            boards,
            won_boards: vec![],
        }
    }

    fn find_first_won_board(&self) -> Option<&Board> {
        self.won_boards.get(0)
    }

    fn new_draw(&mut self, value: u8) {
        self.boards
            .iter_mut()
            .for_each(|board| board.new_draw(value));

        let (new_won_boards, not_won_board) =
            self.boards.to_owned().into_iter().partition(|b| b.is_won());

        self.boards = not_won_board;
        self.won_boards.extend(new_won_boards);
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn is_won(&self) -> bool {
        for index in 0..self.cells.len() {
            if self.is_line_complete(index) {
                return true;
            }

            if self.is_column_complete(index) {
                return true;
            }
        }

        false
    }

    fn is_line_complete(&self, index: usize) -> bool {
        self.cells[index].iter().find(|cell| !cell.drawn).is_none()
    }

    fn is_column_complete(&self, index: usize) -> bool {
        self.cells
            .iter()
            .map(|line| line[index])
            .find(|cell| !cell.drawn)
            .is_none()
    }

    fn new_draw(&mut self, value: u8) {
        for line in self.cells.iter_mut() {
            for cell in line.iter_mut() {
                cell.update_state(value);
            }
        }
    }

    fn get_undrawn_cells(&self) -> Vec<u8> {
        self.cells
            .iter()
            .map(|line| {
                line.iter()
                    .filter_map(|cell| if !cell.drawn { Some(cell.value) } else { None })
            })
            .flatten()
            .collect()
    }

    fn get_undrawned_cells_score(&self) -> u32 {
        self.get_undrawn_cells()
            .into_iter()
            .fold(0u32, |acc, n| acc + n as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell {
    value: u8,
    drawn: bool,
}

impl Cell {
    fn new(value: u8) -> Self {
        Cell {
            drawn: false,
            value,
        }
    }

    fn update_state(&mut self, value: u8) {
        if self.value == value {
            self.drawn = true;
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let game = parse_input(&input);

    let part1 = part1(game.clone());
    println!("part1 result {}", part1);

    let part2 = part2(game.clone());
    println!("part2 result {}", part2);
}

fn parse_input(input: &str) -> Game {
    let mut parts: Vec<&str> = input.split("\r\n\r\n").collect();

    // first part are drawn numbers
    let drawn_numbers: Vec<u8> = parts
        .remove(0)
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let boards = parts
        .iter()
        .map(|part| {
            let cells = part
                .lines()
                .map(|line| {
                    line.trim()
                        .split_ascii_whitespace()
                        .map(|n| Cell::new(n.parse().unwrap()))
                        .collect()
                })
                .collect();

            Board { cells }
        })
        .collect();

    Game::new(drawn_numbers, boards)
}

fn part1(mut game: Game) -> u32 {
    for drawn_number in game.drawn_numbers.to_owned().into_iter() {
        game.new_draw(drawn_number);
        if let Some(board) = game.find_first_won_board() {
            return board.get_undrawned_cells_score() * drawn_number as u32;
        }
    }
    panic!("No board has won");
}

fn part2(mut game: Game) -> u32 {
    for drawn_number in game.drawn_numbers.to_owned().into_iter() {
        game.new_draw(drawn_number);
        if game.boards.len() == 0 {
            return game.won_boards[game.won_boards.len() - 1].get_undrawned_cells_score()
                * drawn_number as u32;
        }
    }
    panic!("No board has won");
}
