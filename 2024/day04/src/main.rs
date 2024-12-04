use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());
    println!("part1 {}", part1(input.clone()));
    println!("part2 {}", part2(input.clone()));
}

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: Input) -> usize {
    let target = String::from("XMAS");
    let neighbours_directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    input.iter().enumerate().fold(0, |acc, (x, line)| {
        acc + line.iter().enumerate().fold(0, |mut acc, (y, n)| {
            if *n != 'X' {
                return acc;
            }

            for direction in neighbours_directions {
                let mut s = String::from("X");
                let mut current = (x as i32 + direction.0, y as i32 + direction.1);

                while target.starts_with(&s) && target != s {
                    if let Some(ss) = input
                        .get(current.0 as usize)
                        .unwrap_or(&vec![])
                        .get(current.1 as usize)
                    {
                        s.push(*ss);
                        current = (current.0 + direction.0, current.1 + direction.1)
                    } else {
                        break;
                    }
                }

                if target == s {
                    acc += 1;
                }
            }

            acc
        })
    })
}

fn part2(input: Input) -> usize {
    let neighbours_directions = [(-1, -1), (1, -1)];

    input.iter().enumerate().fold(0, |acc, (x, line)| {
        acc + line.iter().enumerate().fold(0, |acc, (y, n)| {
            if *n != 'A' {
                return acc;
            }

            let mas_count: usize = neighbours_directions
                .iter()
                .map(|direction| {
                    let mut s = String::new();
                    s.push(
                        *input
                            .get((x as i32 + direction.0) as usize)
                            .unwrap_or(&vec![])
                            .get((y as i32 + direction.1) as usize)
                            .unwrap_or(&'_'),
                    );
                    s.push('A');
                    s.push(
                        *input
                            .get((x as i32 - direction.0) as usize)
                            .unwrap_or(&vec![])
                            .get((y as i32 - direction.1) as usize)
                            .unwrap_or(&'_'),
                    );

                    if "SAM" == s || "MAS" == s {
                        1
                    } else {
                        0
                    }
                })
                .sum();

            acc + if mas_count == 2 { 1 } else { 0 }
        })
    })
}
