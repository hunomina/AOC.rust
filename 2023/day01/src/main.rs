use std::fs;

const NUMBERS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {}", part1(input.clone()));
    println!("part2 {}", part2(input));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().into_iter().collect())
        .collect()
}

fn part1(input: Vec<Vec<char>>) -> u32 {
    input
        .into_iter()
        .map(|l| {
            let l: Vec<u32> = l.into_iter().filter_map(|n| n.to_digit(10)).collect();
            l.first().unwrap() * 10 + l.last().unwrap()
        })
        .sum()
}

fn part2(input: Vec<Vec<char>>) -> u32 {
    input
        .into_iter()
        .map(|l| {
            let mut numbers = vec![];
            for mut i in 0..l.len() {
                match l[i].to_digit(10) {
                    Some(d) => numbers.push(d),
                    None => {
                        match find_starting_number_in_string(String::from_iter(l[i..].iter())) {
                            Some(d) => {
                                numbers.push(d as u32 + 1);
                                i += NUMBERS[d].len();
                            }
                            None => {}
                        }
                    }
                }
            }
            numbers.first().unwrap() * 10 + numbers.last().unwrap()
        })
        .sum()
}

fn find_starting_number_in_string(input: String) -> Option<usize> {
    for (index, number) in NUMBERS.into_iter().enumerate() {
        if input.starts_with(number) {
            return Some(index);
        }
    }
    None
}
