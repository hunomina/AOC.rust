use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());
    println!("solution 1: {}", part1(input.clone()));
    println!("solution 2: {}", part2(input.clone()));
}

type Input = Vec<u32>;

fn parse_input(input: &str) -> Input {
    input.split(",").map(|c| c.parse().unwrap()).collect()
}

fn part1(mut input: Input) -> u32 {
    input[1] = 12;
    input[2] = 2;

    let mut opcode_id = 0;
    loop {
        let operation = input[4 * opcode_id];
        let first_value = input[1 + 4 * opcode_id];
        let second_value = input[2 + 4 * opcode_id];
        let output = input[3 + 4 * opcode_id];

        if operation == 99 {
            break;
        }

        let new_value = if operation == 1 {
            input[first_value as usize] + input[second_value as usize]
        } else {
            input[first_value as usize] * input[second_value as usize]
        };

        input[output as usize] = new_value;
        opcode_id += 1;
    }

    input[0]
}

fn part2(input: Input) -> u32 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut input_clone = input.clone();
            input_clone[1] = noun;
            input_clone[2] = verb;

            let mut opcode_id = 0;
            loop {
                let operation = input_clone[4 * opcode_id];
                let first_value = input_clone[1 + 4 * opcode_id];
                let second_value = input_clone[2 + 4 * opcode_id];
                let output = input_clone[3 + 4 * opcode_id];

                if operation == 99 {
                    break;
                }

                let new_value = if operation == 1 {
                    input_clone[first_value as usize] + input_clone[second_value as usize]
                } else {
                    input_clone[first_value as usize] * input_clone[second_value as usize]
                };

                input_clone[output as usize] = new_value;
                opcode_id += 1;
            }

            if input_clone[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("solution not found")
}
