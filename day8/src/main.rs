use std::fs;

enum TerminationStatus {
    Done(i64),
    InfiniteLoop,
}

struct Program {
    instructions: Vec<Instruction>,
    already_executed: Vec<usize>,
    cursor: usize,
    accumulator: i64,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        Program {
            instructions: instructions,
            cursor: 0,
            accumulator: 0,
            already_executed: vec![],
        }
    }
    fn run(&mut self) -> TerminationStatus {
        let max_custor = self.instructions.len() - 1;
        loop {
            if self.cursor > max_custor {
                break;
            }
            if self.already_executed.contains(&self.cursor) {
                return TerminationStatus::InfiniteLoop;
            }
            self.already_executed.push(self.cursor.clone());
            match &self.instructions[self.cursor] {
                Instruction::Nop(_) => {
                    self.cursor += 1;
                }
                Instruction::Jmp(v) => {
                    self.cursor = (self.cursor as i64 + v) as usize;
                }
                Instruction::Acc(v) => {
                    self.cursor += 1;
                    self.accumulator += v;
                }
            }
        }
        TerminationStatus::Done(self.accumulator)
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Nop(i64),
    Jmp(i64),
    Acc(i64),
}

impl Instruction {
    fn convert(self) -> Self {
        match self {
            Instruction::Nop(v) => Instruction::Jmp(v),
            Instruction::Jmp(v) => Instruction::Nop(v),
            _ => self,
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let instructions = parse_input(input);
    let result = instructions
        .iter()
        .enumerate()
        // remove Acc instructions (can not cause infinite loop)
        .filter(|(_, instruction)| {
            if let Instruction::Acc(_) = instruction {
                return false;
            }
            true
        })
        // find the first finishing program
        .find_map(|(index, _)| {
            let mut instructions_copy = instructions.to_vec();
            instructions_copy[index] = instructions_copy[index].convert();
            match Program::new(instructions_copy).run() {
                TerminationStatus::Done(v) => Some(v),
                _ => None,
            }
        });

    println!("{}", result.unwrap());
}

fn parse_input(input: String) -> Vec<Instruction> {
    input
        .split('\n')
        .map(|line| {
            let mut split_line = line.split(' ');
            let instruction = split_line.nth(0).unwrap();
            let value: i64 = split_line.nth(0).unwrap().parse().unwrap();
            match instruction {
                "jmp" => Instruction::Jmp(value),
                "acc" => Instruction::Acc(value),
                "nop" => Instruction::Nop(value),
                v => panic!("{}", v),
            }
        })
        .collect()
}
