use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {}", part1(input.clone()));
    println!("part2 {}", part2(input));
}

type Input = Vec<Vec<(char, u32)>>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| {
                    let (direction, distance) = s.split_at(1);
                    (direction.chars().next().unwrap(), distance.parse().unwrap())
                })
                .collect()
        })
        .collect()
}

fn part1(input: Input) -> i32 {
    let wires = get_wires_positions(input);

    get_intersections(&wires)
        .into_iter()
        .fold(i32::MAX, |acc, (x, y): (i32, i32)| {
            acc.min(x.abs() + y.abs())
        })
}

fn part2(input: Input) -> u32 {
    let wires = get_wires_positions(input);

    let mut minimum_steps = u32::MAX;

    for intersection in get_intersections(&wires).into_iter() {
        let mut steps = 0;
        for wire in wires.iter() {
            let mut position_id = 1;
            loop {
                steps += 1;
                if wire[position_id] == intersection {
                    break;
                }
                position_id += 1;
            }
        }
        minimum_steps = minimum_steps.min(steps);
    }

    minimum_steps
}

fn get_wires_positions(input: Input) -> Vec<Vec<(i32, i32)>> {
    let mut wires = vec![vec![(0, 0)]; input.len()];

    for (wire_id, movements) in input.into_iter().enumerate() {
        for (direction, distance) in movements {
            for _ in 0..distance {
                let mut new = *wires[wire_id].last().unwrap();
                match direction {
                    'L' => new.0 -= 1,
                    'R' => new.0 += 1,
                    'U' => new.1 -= 1,
                    'D' => new.1 += 1,
                    _ => panic!(),
                };
                wires[wire_id].push(new);
            }
        }
    }

    wires
}

fn get_intersections(wires: &[Vec<(i32, i32)>]) -> Vec<(i32, i32)> {
    let (longest_wire, shortest_wire) = if wires[0].len() > wires[1].len() {
        (0, 1)
    } else {
        (1, 0)
    };

    let mut intersections = vec![];
    for position in wires[shortest_wire].iter() {
        if *position == (0, 0) {
            continue;
        }

        if wires[longest_wire].contains(position) {
            intersections.push(*position);
        }
    }

    intersections
}
