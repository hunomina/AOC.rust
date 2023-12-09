use std::{cmp::Ordering, collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input));
}

type Input = Vec<(Hand, u32)>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut splitted_line = line.split_ascii_whitespace();
            (
                Hand::new(splitted_line.next().unwrap().chars().collect()),
                splitted_line.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn part1(mut input: Input) -> u32 {
    input.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    input
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + ((i as u32 + 1) * bid))
}

type Cards = [u8; 5];

#[derive(Debug)]
struct Hand {
    cards: Cards,
    hand_type: HandType,
    has_jokers: bool,
}

impl Hand {
    fn new(cards: Vec<char>) -> Self {
        let mut cards = cards.into_iter();
        let cards = [
            Self::transform_char_into_card(cards.next().unwrap()),
            Self::transform_char_into_card(cards.next().unwrap()),
            Self::transform_char_into_card(cards.next().unwrap()),
            Self::transform_char_into_card(cards.next().unwrap()),
            Self::transform_char_into_card(cards.next().unwrap()),
        ];

        let hand_type = Self::detect_hand_type(&cards);

        Hand {
            cards,
            hand_type,
            has_jokers: false,
        }
    }

    fn transform_char_into_card(c: char) -> u8 {
        if c.is_ascii_digit() {
            return c.to_digit(10).unwrap() as u8;
        }

        match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!(),
        }
    }

    fn detect_hand_type(cards: &Cards) -> HandType {
        let mut map = HashMap::new();

        for c in cards {
            map.insert(c, map.get(&c).unwrap_or(&0usize) + 1);
        }

        let map_values = map.values();

        if map_values.clone().any(|n| *n == 5) {
            HandType::FIVE
        } else if map_values.clone().any(|n| *n == 4) {
            HandType::FOUR
        } else if map_values.clone().any(|n| *n == 3) {
            if map_values.clone().any(|n| *n == 2) {
                HandType::FULL
            } else {
                HandType::THREE
            }
        } else if map_values.clone().filter(|n| **n == 2).count() == 2 {
            HandType::TWO_PAIRS
        } else if map_values.clone().any(|n| *n == 2) {
            HandType::PAIR
        } else {
            HandType::LA_DECHE
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum HandType {
    FIVE,
    FOUR,
    FULL,
    THREE,
    TWO_PAIRS,
    PAIR,
    LA_DECHE,
}

impl HandType {
    fn value(&self) -> u8 {
        match self {
            HandType::FIVE => 6,
            HandType::FOUR => 5,
            HandType::FULL => 4,
            HandType::THREE => 3,
            HandType::TWO_PAIRS => 2,
            HandType::PAIR => 1,
            HandType::LA_DECHE => 0,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type.value() > other.hand_type.value() {
            Some(Ordering::Greater)
        } else if self.hand_type.value() == other.hand_type.value() {
            for i in 0..5 {
                let card_cmp = self.cards[i].cmp(&other.cards[i]);
                if let Ordering::Equal = card_cmp {
                    continue;
                }

                return Some(card_cmp);
            }
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }
}
