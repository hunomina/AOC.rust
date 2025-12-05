use std::{fs, ops::RangeInclusive};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input = (Vec<RangeInclusive<usize>>, Vec<usize>);

fn parse_input(input: &str) -> Input {
    let mut split = input.split("\n\n");
    let first_part = split.next().unwrap();
    let second_part = split.next().unwrap();

    let ranges = first_part
        .lines()
        .map(|line| {
            let mut s = line.split('-');

            s.next().unwrap().parse().unwrap()..=s.next().unwrap().parse().unwrap()
        })
        .collect();

    let ids = second_part
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ids)
}

fn part1((ranges, ids): Input) -> usize {
    ids.into_iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn part2((ranges, _): Input) -> usize {
    let mut sorted_ranges: Vec<_> = ranges.into_iter().collect();
    sorted_ranges.sort_by_key(|r| *r.start());

    let merged =
        sorted_ranges
            .into_iter()
            .fold(Vec::<RangeInclusive<usize>>::new(), |mut acc, range| {
                if let Some(last) = acc.last_mut() {
                    if range.start() <= &(*last.end() + 1) {
                        *last = *last.start()..=(*last.end().max(range.end()));
                    } else {
                        acc.push(range);
                    }
                } else {
                    acc.push(range);
                }
                acc
            });

    merged.iter().map(|r| r.end() - r.start() + 1).sum()
}
