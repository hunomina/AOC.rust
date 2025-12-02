use std::{fs, ops::RangeInclusive};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input = Vec<RangeInclusive<u64>>;

fn parse_input(input: &str) -> Input {
    input
        .split(',')
        .map(|r| {
            let mut s = r.split('-');
            let start = s.next().unwrap().parse::<u64>().unwrap();
            let end = s.next().unwrap().parse::<u64>().unwrap();
            start..=end
        })
        .collect()
}

fn part1(input: Input) -> u64 {
    input.into_iter().fold(0, |acc, range| {
        range.fold(acc, |a, n| {
            let s = n.to_string();
            let half = s.len() / 2;

            let (left, right) = s.split_at(half);
            if left == right {
                a + n
            } else {
                a
            }
        })
    })
}

fn part2(input: Input) -> u64 {
    input.into_iter().fold(0, |acc, range| {
        range.fold(acc, |a, n| {
            let mut has_repetitive_pattern = false;

            let s = n.to_string();
            let s_len = s.len();

            for period_len in 1..=s_len / 2 {
                if s_len % period_len == 0 {
                    let pattern = &s[..period_len];
                    let repetitions = s_len / period_len;

                    if pattern.repeat(repetitions) == s {
                        has_repetitive_pattern = true;
                        break;
                    }
                }
            }

            if has_repetitive_pattern {
                a + n
            } else {
                a
            }
        })
    })
}
