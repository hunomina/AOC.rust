use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Equation = (i64, Vec<i64>);
type Input = Vec<Equation>;
type TransformationInput<'a> = (&'a Equation, usize, i64);

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut splitted_line = line.split(": ");
            (
                splitted_line.next().unwrap().parse().unwrap(),
                splitted_line
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn part1(input: Input) -> i64 {
    let tranformations = |(equation, step, current_value): TransformationInput| {
        vec![
            current_value + equation.1[step],
            current_value * equation.1[step],
        ]
    };

    input
        .into_iter()
        .filter(|n| is_valid_equation(n, tranformations))
        .fold(0, |acc, equation| acc + equation.0)
}

fn part2(input: Input) -> i64 {
    let tranformations = |(equation, step, current_value): TransformationInput| {
        vec![
            current_value + equation.1[step],
            current_value * equation.1[step],
            format!("{}{}", current_value, equation.1[step])
                .parse()
                .unwrap(),
        ]
    };

    input
        .into_iter()
        .filter(|n| is_valid_equation(n, tranformations))
        .fold(0, |acc, equation| acc + equation.0)
}

fn is_valid_equation<F: Fn(TransformationInput) -> Vec<i64>>(
    equation: &(i64, Vec<i64>),
    apply_tranformations: F,
) -> bool {
    let mut r = vec![equation.1[0]];

    for step in 1..equation.1.len() {
        r = r
            .into_iter()
            .flat_map(|current_value| apply_tranformations((equation, step, current_value)))
            .collect();
    }

    r.into_iter().any(|n| n == equation.0)
}
