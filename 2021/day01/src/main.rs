use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let numbers = parse_input(input.as_str());

    let part1 = part1(numbers.clone());
    println!("part1 result : {}", part1); // 1655

    let part2 = part2(numbers.clone());
    println!("part2 result : {}", part2); // 1683
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

fn part1(numbers: Vec<u32>) -> usize {
    let first = numbers[0];
    numbers
        .into_iter()
        .fold((first, 0), |acc, x| {
            (x, if x > acc.0 { acc.1 + 1 } else { acc.1 })
        })
        .1
}

fn part2(numbers: Vec<u32>) -> usize {
    part1(
        (0..numbers.len() - 2)
            .map(|i| numbers[i] + numbers[i + 1] + numbers[i + 2])
            .collect(),
    )
}
