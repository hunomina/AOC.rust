use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input<'a> = (Vec<&'a str>, &'a str);

fn parse_input(input: &str) -> Input<'_> {
    let mut problems = input.lines().collect::<Vec<&str>>();
    let last_line = problems.pop().unwrap();

    (problems, last_line)
}

fn part1((problems, operators): Input) -> u128 {
    let problems: Vec<Vec<&str>> = problems
        .into_iter()
        .map(|problem| problem.split_ascii_whitespace().collect())
        .collect();
    let operators: Vec<char> = operators
        .split_ascii_whitespace()
        .map(|op| op.chars().next().unwrap())
        .collect();

    (0..problems[0].len())
        .map(|i| {
            problems
                .iter()
                .map(|problem| problem[i].parse::<u128>().unwrap())
                .reduce(|acc, n| compute(acc, operators[i], n))
                .unwrap()
        })
        .sum()
}

fn part2((problems, operators): Input) -> u128 {
    let mut results = vec![];

    let problems: Vec<Vec<char>> = problems
        .into_iter()
        .map(|problem| problem.chars().collect())
        .collect();

    for (range, operator) in get_operator_ranges(operators) {
        let result = range
            .map(|i| {
                problems
                    .iter()
                    .filter_map(|problem| {
                        let ch = problem[i];
                        (ch != ' ').then_some(ch)
                    })
                    .collect::<String>()
                    .parse::<u128>()
                    .unwrap()
            })
            .reduce(|acc, n| compute(acc, operator, n))
            .unwrap();

        results.push(result);
    }

    results.iter().sum()
}

fn get_operator_positions(s: &str) -> Vec<(usize, char)> {
    s.chars()
        .enumerate()
        .filter(|(_, c)| *c == '+' || *c == '*')
        .collect()
}

fn get_operator_ranges(s: &str) -> Vec<(std::ops::Range<usize>, char)> {
    get_operator_positions(s)
        .iter()
        .enumerate()
        .map(|(i, &(pos, operator))| {
            let start = pos;
            let end = get_operator_positions(s)
                .get(i + 1)
                .map(|&(next_pos, _)| next_pos - 1)
                .unwrap_or(s.len());
            (start..end, operator)
        })
        .collect()
}

fn compute(left: u128, operator: char, right: u128) -> u128 {
    match operator {
        '+' => left + right,
        '*' => left * right,
        _ => panic!("unknown operator: {}", operator),
    }
}
