use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {}", part1(input.clone()));
    println!("part2 {}", part2(input));
}

type InputType<'a> = Vec<(usize, Vec<Vec<(u32, &'a str)>>)>;

fn parse_input(input: &'_ str) -> InputType<'_> {
    input
        .lines()
        .enumerate()
        .map(|(game_id, boxes)| {
            (
                game_id + 1,
                boxes
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .trim()
                    .split("; ")
                    .map(|b| {
                        b.split(", ")
                            .map(|i| {
                                let mut i = i.split_whitespace();
                                (i.next().unwrap().parse::<u32>().unwrap(), i.next().unwrap())
                            })
                            .collect()
                    })
                    .collect(),
            )
        })
        .collect()
}

fn part1(input: InputType) -> usize {
    const RED_LIMIT: u32 = 12;
    const GREEN_LIMIT: u32 = 13;
    const BLUE_LIMIT: u32 = 14;

    input
        .into_iter()
        .filter_map(|(game_id, sets)| {
            match sets.into_iter().find(|boxes| {
                boxes.iter().any(|(count, color)| match *color {
                    "red" => *count > RED_LIMIT,
                    "blue" => *count > BLUE_LIMIT,
                    "green" => *count > GREEN_LIMIT,
                    _ => false,
                })
            }) {
                Some(_) => None,
                None => Some(game_id),
            }
        })
        .sum()
}

fn part2(input: InputType) -> u32 {
    input.into_iter().fold(0, |acc, (_, sets)| {
        let mut color_count = (0, 0, 0); // r, g, b
        sets.into_iter().for_each(|boxes| {
            boxes.into_iter().for_each(|(count, color)| {
                let p = match color {
                    "red" => &mut color_count.0,
                    "green" => &mut color_count.1,
                    "blue" => &mut color_count.2,
                    _ => panic!(),
                };
                if *p < count {
                    *p = count
                }
            })
        });
        acc + color_count.0 * color_count.1 * color_count.2
    })
}
