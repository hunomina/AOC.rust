use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str()).unwrap();

    println!("part1 {}", part1(input.clone()));
    println!("part2 {}", part2(input));
}

type Input = Vec<(u32, u32)>;

fn parse_input(input: &str) -> Option<Input> {
    let mut lines = input.lines();
    let times = lines
        .next()?
        .split(':')
        .nth(1)?
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()?
        .split(':')
        .nth(1)?
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut r = vec![];
    for i in 0..times.len() {
        r.push((times[i], distances[i]));
    }

    Some(r)
}

fn part1(input: Input) -> u32 {
    input
        .into_iter()
        .map(|(time, distance)| {
            let mut r = vec![];
            for i in 1..time {
                let d = i * (time - i);
                if distance < d {
                    r.push(d);
                }
            }
            r.len()
        })
        .product::<usize>() as u32
}

fn part2(input: Input) -> u64 {
    let mut time = String::new();
    let mut distance = String::new();

    input.into_iter().for_each(|(t, d)| {
        time.push_str(&t.to_string());
        distance.push_str(&d.to_string());
    });

    let time = time.parse::<u64>().unwrap();
    let distance = distance.parse::<u64>().unwrap();

    let mut r = 0;
    for i in 1..time {
        let d = i * (time - i);
        if distance < d {
            r += 1;
        }
    }
    r
}
