use std::fs;

#[derive(Clone, Copy)]
enum Movement {
    Up(u32),
    Down(u32),
    Forward(u32),
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let movements = parse_input(input.as_str());

    let part1 = part1(movements.clone());
    println!("part1 result : {}", part1);

    let part2 = part2(movements.clone());
    println!("part2 result : {}", part2);
}

fn parse_input(input: &str) -> Vec<Movement> {
    input
        .lines()
        .into_iter()
        .map(|s| {
            let mut splitted_command = s.split(' ');
            let (direction, distance) = (
                splitted_command.nth(0).unwrap(),
                splitted_command.nth(0).unwrap().parse::<u32>().unwrap(),
            );
            match direction {
                "forward" => Movement::Forward(distance),
                "up" => Movement::Up(distance),
                "down" => Movement::Down(distance),
                _ => panic!("Unknown command"),
            }
        })
        .collect()
}

fn part1(movements: Vec<Movement>) -> i32 {
    let position = movements
        .into_iter()
        .fold((0, 0), |position, movement| match movement {
            Movement::Forward(d) => (position.0 + d as i32, position.1),
            Movement::Up(d) => (position.0, position.1 - d as i32),
            Movement::Down(d) => (position.0, position.1 + d as i32),
        });
    position.0 * position.1
}

fn part2(movements: Vec<Movement>) -> i32 {
    let position = movements
        .into_iter()
        .fold((0, 0, 0), |position, movement| match movement {
            Movement::Forward(d) => (
                position.0 + d as i32,
                position.1 + (position.2 * d as i32),
                position.2,
            ),
            Movement::Up(d) => (position.0, position.1, position.2 - d as i32),
            Movement::Down(d) => (position.0, position.1, position.2 + d as i32),
        });
    position.0 * position.1
}
