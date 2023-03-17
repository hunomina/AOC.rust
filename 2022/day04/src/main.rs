use std::{fs, ops::RangeInclusive};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    println!("part1 {:?}", part1(parse_input(&input)));
    println!("part2 {:?}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input
        .lines()
        .map(|line| {
            let mut line_split = line.split(',');

            let (mut first_range, mut second_range) = (
                line_split.next().unwrap().split('-'),
                line_split.next().unwrap().split('-'),
            );

            (
                first_range.next().unwrap().parse().unwrap()
                    ..=first_range.next().unwrap().parse().unwrap(),
                second_range.next().unwrap().parse().unwrap()
                    ..=second_range.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn part1(ranges: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> u32 {
    ranges
        .into_iter()
        .map(|(r1, r2)| {
            r1.contains(r2.start()) && r1.contains(r2.end())
                || r2.contains(r1.start()) && r2.contains(r1.end())
        })
        .fold(0, |acc, x| if x { acc + 1 } else { acc })
}

fn part2(ranges: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>) -> u32 {
    ranges
        .into_iter()
        .map(|(r1, r2)| {
            r1.contains(r2.start())
                || r1.contains(r2.end())
                || r2.contains(r1.start())
                || r2.contains(r1.end())
        })
        .fold(0, |acc, x| if x { acc + 1 } else { acc })
}
