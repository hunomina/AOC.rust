use std::fs;

type Position = (usize, usize);

#[derive(Clone, Debug)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Clone)]
struct Puzzle {
    board: Vec<Vec<bool>>,
    folds: Vec<Fold>,
}

impl Puzzle {
    fn print(&self) {
        for line in self.board.iter() {
            for cell in line.iter() {
                print!("{}", if *cell { '#' } else { '.' })
            }
            println!();
        }
    }

    fn count_visible_dots(&self) -> u32 {
        self.board.iter().fold(0, |acc, line| {
            acc + line.iter().fold(0, |acc, cell| acc + *cell as u32)
        })
    }

    fn fold_all(&mut self) {
        (0..self.folds.len()).for_each(|_| self.fold());
    }

    fn fold(&mut self) {
        match self.folds.remove(0) {
            Fold::Horizontal(n) => {
                (0..n).for_each(|line| {
                    (0..self.board[line].len()).for_each(|column| {
                        self.board[line][column] = self.board[line][column]
                            || match self.board.get(n * 2 - line) {
                                Some(line) => line[column],
                                None => false,
                            };
                    });
                });
                // remove lines bellow n (n included)
                self.board = self.board[0..n].to_vec();
            }
            Fold::Vertical(n) => {
                (0..self.board.len()).for_each(|line| {
                    (0..n).for_each(|column| {
                        self.board[line][column] = self.board[line][column]
                            || *self.board[line].get(n * 2 - column).unwrap_or(&false);
                    });
                });
                // remove columns after n (n included)
                self.board.iter_mut().for_each(|line| {
                    *line = line[0..n].to_vec();
                });
            }
        };
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let puzzle = parse_input(&input);

    let mut p = puzzle.clone();
    p.fold();
    println!("part1 result {}", p.count_visible_dots());

    let mut p = puzzle.clone();
    p.fold_all();
    println!("part2 output:");
    p.print();
}

fn parse_input(input: &str) -> Puzzle {
    let mut input = input.split("\r\n\r\n");

    let positions: Vec<Position> = input
        .next()
        .unwrap()
        .lines()
        .map(|position| {
            let mut position = position.split(',');
            (
                position.next().unwrap().parse().unwrap(),
                position.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut board = create_board(&positions);

    positions.into_iter().for_each(|(x, y)| board[y][x] = true);

    let folds = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut splited_line = line.split('=');
            match splited_line.next().unwrap().ends_with('x') {
                true => Fold::Vertical(splited_line.next().unwrap().parse().unwrap()),
                false => Fold::Horizontal(splited_line.next().unwrap().parse().unwrap()),
            }
        })
        .collect();

    Puzzle { board, folds }
}

fn create_board(positions: &Vec<Position>) -> Vec<Vec<bool>> {
    vec![
        vec![
            false;
            positions
                .iter()
                .fold(0, |acc, (x, _)| if *x > acc { *x } else { acc })
                + 1
        ];
        positions
            .iter()
            .fold(0, |acc, (_, y)| if *y > acc { *y } else { acc })
            + 1
    ]
}
