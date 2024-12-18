use std::{fs, ops::Range, thread};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Registers = (usize, usize, usize);
type Input = (Registers, Vec<usize>);

fn parse_input(input: &str) -> Input {
    let mut split = input.split("\n\n");

    let mut registers = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(": ").last().unwrap().parse().unwrap());

    let registers = (
        registers.next().unwrap(),
        registers.next().unwrap(),
        registers.next().unwrap(),
    );

    let instructions = split
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    (registers, instructions)
}

fn part1(input: Input) -> String {
    let (registers, instructions) = input;

    solve(registers, instructions, false)
        .unwrap()
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part2(input: Input) -> usize {
    // bruteforce DOES NOT WORK 😅
    let (registers, instructions) = input;

    let mut thread_pool_id = 0;
    let thread_pool_size = 2000;
    let iterations_per_thread = 10000;

    loop {
        let min_for_thread_pool = thread_pool_id * thread_pool_size * iterations_per_thread;
        let mut handles = vec![];

        for thread_id in 1..thread_pool_size {
            let range = min_for_thread_pool + (thread_id * iterations_per_thread)
                ..min_for_thread_pool + ((thread_id + 1) * iterations_per_thread);
            let instructions_clone = instructions.clone();
            let handle =
                thread::spawn(move || solve_for_range(range, registers, instructions_clone));

            handles.push(handle);
        }

        for handle in handles {
            if let Ok(v) = handle.join() {
                if let Some(v) = v {
                    return v;
                }
            }
        }
        thread_pool_id += 1;
    }
}

fn solve_for_range(
    range: Range<usize>,
    mut registers: Registers,
    instructions: Vec<usize>,
) -> Option<usize> {
    for i in range {
        registers.0 = i;
        if let Some(outputs) = solve(registers, instructions.clone(), true) {
            if outputs == instructions {
                return Some(i);
            }
        }
    }

    None
}

fn solve(mut registers: Registers, instructions: Vec<usize>, p2: bool) -> Option<Vec<usize>> {
    let mut outputs = vec![];
    let mut instruction_pointer = 0;
    while instruction_pointer < instructions.len() {
        let (instruction, literal_operand) = (
            instructions[instruction_pointer],
            instructions[instruction_pointer + 1],
        );

        let (new_registers, jump, output) =
            apply_instruction(registers, instruction, literal_operand);
        registers = new_registers;

        instruction_pointer = if let Some(jump_location) = jump {
            jump_location
        } else {
            instruction_pointer + 2
        };

        if let Some(value) = output {
            outputs.push(value);
        }

        if p2 && outputs.len() == instructions.len() {
            return Some(outputs);
        }
    }

    Some(outputs)
}

fn get_combo_operand(literal_operand: usize, registers: &Registers) -> usize {
    match literal_operand {
        0..=3 => literal_operand,
        4 => registers.0,
        5 => registers.1,
        6 => registers.2,
        _ => panic!("AH!"),
    }
}

fn apply_instruction(
    mut registers: Registers,
    instruction: usize,
    literal_operand: usize,
) -> (Registers, Option<usize>, Option<usize>) {
    let mut jump = None;
    let mut output = None;

    match instruction {
        0 => {
            let combo_operand_value = get_combo_operand(literal_operand, &registers);
            registers.0 /= 2_usize.pow(combo_operand_value as u32);
        }
        1 => {
            registers.1 ^= literal_operand;
        }
        2 => {
            registers.1 = get_combo_operand(literal_operand, &registers).rem_euclid(8);
        }
        3 => {
            if registers.0 != 0 {
                jump = Some(literal_operand);
            }
        }
        4 => {
            registers.1 ^= registers.2;
        }
        5 => {
            output = Some(get_combo_operand(literal_operand, &registers).rem_euclid(8));
        }
        6 => {
            let combo_operand_value = get_combo_operand(literal_operand, &registers);
            registers.1 = registers.0 / 2_usize.pow(combo_operand_value as u32);
        }
        7 => {
            let combo_operand_value = get_combo_operand(literal_operand, &registers);
            registers.2 = registers.0 / 2_usize.pow(combo_operand_value as u32);
        }
        _ => {}
    }

    (registers, jump, output)
}
