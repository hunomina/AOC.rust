use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {}", part1(input.clone()));
    println!("part2 {}", part2(input.clone()));
}

type Input = Vec<Vec<u32>>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: Input) -> usize {
    input.into_iter().fold(0, |acc, report| {
        acc + match validate_report(report) {
            false => 0,
            true => 1,
        }
    })
}

fn part2(input: Input) -> usize {
    input.into_iter().fold(0, |acc, report| {
        acc + {
            let mut result = 0;
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if validate_report(new_report) {
                    result = 1;
                    break;
                }
            }

            result
        }
    })
}

fn validate_report(report: Vec<u32>) -> bool {
    let ordering = report[0].cmp(&report[1]);

    for i in 0..report.len() - 1 {
        let pair = (report[i], report[i + 1]);
        let diff = pair
            .0
            .checked_sub(pair.1)
            .unwrap_or_else(|| pair.1.checked_sub(pair.0).unwrap());
        if pair.0.cmp(&pair.1) != ordering || diff == 0 || diff > 3 {
            return false;
        }
    }
    true
}
