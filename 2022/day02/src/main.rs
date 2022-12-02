use core::panic;
use std::{cmp::Ordering, fs};

fn main() {
    let part1 = fs::read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut split_line = line.split(' ');
            let (opponent, me) = (
                Sign::from(split_line.nth(0).unwrap()),
                Sign::from(split_line.nth(0).unwrap()),
            );
            <Sign as Into<u32>>::into(me)
                + match me.partial_cmp(&opponent).unwrap() {
                    Ordering::Greater => 6,
                    Ordering::Equal => 3,
                    Ordering::Less => 0,
                }
        })
        .sum::<u32>();

    let part2 = fs::read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut split_line = line.split(' ');
            let (opponent, expected_outcome) = (
                Sign::from(split_line.nth(0).unwrap()),
                ExpectedOutcome::from(split_line.nth(0).unwrap()),
            );
            let me = opponent.matches(expected_outcome);
            <Sign as Into<u32>>::into(me)
                + match me.partial_cmp(&opponent).unwrap() {
                    Ordering::Greater => 6,
                    Ordering::Equal => 3,
                    Ordering::Less => 0,
                }
        })
        .sum::<u32>();

    println!("{:?}", part1);
    println!("{:?}", part2);
}

#[derive(PartialEq, Clone, Copy)]
enum Sign {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Sign {
    fn from(c: &str) -> Self {
        match c {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!(),
        }
    }
}

impl Into<u32> for Sign {
    fn into(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl PartialOrd for Sign {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Self::Rock => match other {
                Self::Rock => Ordering::Equal,
                Self::Paper => Ordering::Less,
                Self::Scissors => Ordering::Greater,
            },
            Self::Paper => match other {
                Self::Rock => Ordering::Greater,
                Self::Paper => Ordering::Equal,
                Self::Scissors => Ordering::Less,
            },
            Self::Scissors => match other {
                Self::Rock => Ordering::Less,
                Self::Paper => Ordering::Greater,
                Self::Scissors => Ordering::Equal,
            },
        })
    }
}

impl Sign {
    fn matches(&self, expected_outcome: ExpectedOutcome) -> Self {
        match expected_outcome {
            ExpectedOutcome::Lose => match self {
                Self::Paper => Self::Rock,
                Self::Rock => Self::Scissors,
                Self::Scissors => Self::Paper,
            },
            ExpectedOutcome::Draw => self.clone(),
            ExpectedOutcome::Win => match self {
                Self::Paper => Self::Scissors,
                Self::Rock => Self::Paper,
                Self::Scissors => Self::Rock,
            },
        }
    }
}

enum ExpectedOutcome {
    Lose,
    Draw,
    Win,
}

impl From<&str> for ExpectedOutcome {
    fn from(c: &str) -> Self {
        match c {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!(),
        }
    }
}
