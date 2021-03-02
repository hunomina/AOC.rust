// see : https://adventofcode.com/2020/day/2

use std::fs;

struct Password {
    value: String,
    policy: PasswordPolicy,
}

struct PasswordPolicy {
    char: char,
    min: u8,
    max: u8,
}

impl Password {
    fn valid(&self) -> bool {
        (self
            .value
            .chars()
            .nth((self.policy.min - 1).into())
            .unwrap()
            == self.policy.char)
            ^ (self
                .value
                .chars()
                .nth((self.policy.max - 1).into())
                .unwrap()
                == self.policy.char)
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let passwords = parse_input(input.as_str());
    println!(
        "{}",
        passwords.iter().fold(0, |c, password| {
            if password.valid() {
                c + 1
            } else {
                c
            }
        })
    );
}

fn parse_input(input: &str) -> Vec<Password> {
    let mut passwords: Vec<Password> = vec![];

    let lines = input.split('\n');

    for line in lines {
        let split = line.split(' ').collect::<Vec<&str>>();
        let min_max = split[0].split('-').collect::<Vec<&str>>();

        passwords.push(Password {
            value: split[2].to_string(),
            policy: PasswordPolicy {
                char: split[1].chars().nth(0).unwrap(),
                min: min_max[0].parse::<u8>().unwrap(),
                max: min_max[1].parse::<u8>().unwrap(),
            },
        });
    }

    passwords
}
