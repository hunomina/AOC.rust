use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input = Vec<((i128, i128), (i128, i128), (i128, i128))>;

fn parse_input(input: &str) -> Input {
    let regex = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    input
        .split("\n\n")
        .map(|group| {
            let c = regex.captures_iter(group).next().unwrap();

            (
                (
                    c.get(1).unwrap().as_str().parse().unwrap(),
                    c.get(2).unwrap().as_str().parse().unwrap(),
                ),
                (
                    c.get(3).unwrap().as_str().parse().unwrap(),
                    c.get(4).unwrap().as_str().parse().unwrap(),
                ),
                (
                    c.get(5).unwrap().as_str().parse().unwrap(),
                    c.get(6).unwrap().as_str().parse().unwrap(),
                ),
            )
        })
        .collect::<Vec<_>>()
}

fn part1(input: Input) -> i128 {
    input
        .into_iter()
        .filter_map(|(button_a, button_b, prize)| {
            solve_equation_system(button_a, button_b, prize).and_then(|(x, y)| {
                if x > 100 || y > 100 {
                    None
                } else {
                    Some((x, y))
                }
            })
        })
        .fold(0, |acc, (x, y)| acc + x * 3 + y)
}

fn part2(input: Input) -> i128 {
    input
        .into_iter()
        .filter_map(|(button_a, button_b, mut prize)| {
            prize.0 += 10000000000000;
            prize.1 += 10000000000000;

            solve_equation_system(button_a, button_b, prize)
        })
        .fold(0, |acc, (x, y)| acc + x * 3 + y)
}

fn solve_equation_system(
    (ax, ay): (i128, i128),
    (bx, by): (i128, i128),
    (x, y): (i128, i128),
) -> Option<(i128, i128)> {
    let div = ax * by - ay * bx;
    let x_coef = (x * by - y * bx) / div;
    let y_coef = (ax * y - ay * x) / div;

    if (ax * x_coef + bx * y_coef, ay * x_coef + by * y_coef) == (x, y) {
        Some((x_coef, y_coef))
    } else {
        None
    }
}
