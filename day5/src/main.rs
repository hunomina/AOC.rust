// https://adventofcode.com/2020/day/5

// plane : 128 rows, 8 columns

const MIN_ROW: u8 = 0;
const MAX_ROW: u8 = 127;

const MIN_COLUMN: u8 = 0;
const MAX_COLUMN: u8 = 7;

use std::clone::Clone;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::marker::Copy;

#[derive(Debug, Copy, Clone)]
struct Position {
    row: u8,
    column: u8,
}

struct Plain {
    seats: Vec<Seat>,
    memo: HashMap<u16, Option<Seat>>,
}

impl Plain {
    fn get_seat_by_id(&mut self, id: u16) -> Option<Seat> {
        if self.memo.contains_key(&id) {
            return *self.memo.get(&id).unwrap();
        }
        for seat in self.seats.iter() {
            if seat.get_id() == id {
                self.memo.insert(id, Some(*seat));
                return Some(*seat);
            }
        }
        None
    }
}

#[derive(Copy, Clone)]
struct Seat {
    position: Position,
}

impl Seat {
    fn from_reference(reference: &str) -> Seat {
        Seat {
            position: Position {
                row: Seat::get_row(reference),
                column: Seat::get_column(reference),
            },
        }
    }
    fn get_row(reference: &str) -> u8 {
        let mut min = MIN_ROW;
        let mut max = MAX_ROW;
        for i in 0..6 {
            match reference.chars().nth(i).unwrap() {
                'F' => max -= (max - min) / 2 + 1,   // lower
                'B' => min += ((max - min) / 2) + 1, // upper
                _ => {}
            };
        }
        match reference.chars().nth(6).unwrap() {
            'F' => min,
            _ => max,
        }
    }
    fn get_column(reference: &str) -> u8 {
        let mut min = MIN_COLUMN;
        let mut max = MAX_COLUMN;
        for i in 0..2 {
            match reference.chars().nth(7 + i).unwrap() {
                'L' => max -= (max - min) / 2 + 1,   // lower
                'R' => min += ((max - min) / 2) + 1, // upper
                _ => {}
            };
        }
        match reference.chars().nth(9).unwrap() {
            'L' => min,
            _ => max,
        }
    }
    fn get_id(&self) -> u16 {
        (self.position.row as u16) * 8 + (self.position.column as u16)
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let seat_rows = input.split('\n');
    let mut seats: Vec<Seat> = vec![];

    for seat_row in seat_rows {
        seats.push(Seat::from_reference(seat_row));
    }

    let mut plain = Plain {
        seats,
        memo: HashMap::new(),
    };
    let max_seat = Seat::from_reference("BBBBBBBRRR");

    for seat_id in 1..max_seat.get_id() {
        if plain.get_seat_by_id(seat_id - 1).is_some()
            && plain.get_seat_by_id(seat_id).is_none()
            && plain.get_seat_by_id(seat_id + 1).is_some()
        {
            println!("{}", seat_id);
            break;
        }
    }
}
