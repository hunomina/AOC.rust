use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {}", part1(input.clone()));
    println!("part2 {}", part2(input));
}

type Input = (Vec<u32>, Vec<u32>);

fn parse_input(input: &str) -> Input {
    let mut list1 = vec![];
    let mut list2 = vec![];

    input.lines().for_each(|line| {
        let mut line = line.split_ascii_whitespace();
        list1.push(line.nth(0).unwrap().parse().unwrap());
        list2.push(line.nth(0).unwrap().parse().unwrap());
    });

    (list1, list2)
}

fn part1(mut input: Input) -> u32 {
    input.0.sort();
    input.1.sort();

    input.0.into_iter().enumerate().fold(0, |acc, (i, x)| {
        acc + if x > input.1[i] {
            x - input.1[i]
        } else {
            input.1[i] - x
        }
    })
}

fn part2(input: Input) -> u32 {
    input.0.into_iter().fold(0, |acc, n| {
        acc + input.1.iter().filter(|o| **o == n).count() as u32 * n
    })
}
