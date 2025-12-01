use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input = Vec<(char, f32)>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (dir, dist) = line.split_at(1);
            (
                dir.chars().next().unwrap(),
                dist.trim().parse::<f32>().unwrap(),
            )
        })
        .collect()
}

fn part1(input: Input) -> usize {
    let mut result = 0;

    let mut pointer: f32 = 50.0;

    for (dir, dist) in input {
        match dir {
            'L' => pointer -= dist,
            'R' => pointer += dist,
            _ => panic!("unknown direction"),
        }

        pointer %= 100.0;
        if pointer < 0.0 {
            pointer += 100.0;
        }

        if pointer == 0.0 {
            result += 1;
        }
    }

    result
}

fn part2(input: Input) -> usize {
    let mut result = 0;

    let mut pointer: f32 = 50.0;

    for (dir, dist) in input {
        let rotation = match dir {
            'L' => -1,
            'R' => 1,
            _ => panic!("unknown direction"),
        };

        let dist_to_zero = if rotation == 1 {
            100.0 - pointer
        } else {
            pointer
        };

        if dist_to_zero > 0.0 && dist >= dist_to_zero {
            result += 1;
        }

        result += ((dist - dist_to_zero) / 100.0) as usize;

        pointer += rotation as f32 * dist;
        pointer %= 100.0;
        if pointer < 0.0 {
            pointer += 100.0;
        }
    }

    result
}
