use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let entries = parse_input(&input);

    let part1 = part1(entries.clone());
    println!("part1 result {}", part1);
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn part1(entries: Vec<String>) -> u32 {
    let entries: Vec<Vec<u8>> = entries.into_iter().map(|e| e.into_bytes()).collect();
    let entry_size = entries.get(0).unwrap().len();
    let result: String = (0..entry_size)
        .map(|i| {
            let column = entries
                .iter()
                .map(|e| e[i] as char)
                .fold(0i32, |acc, c| match c {
                    '0' => acc - 1,
                    '1' => acc + 1,
                    _ => panic!("woops"),
                });
            if column > 0 {
                '1'
            } else {
                '0'
            }
        })
        .collect();
    let gamma_rate = u32::from_str_radix(&result, 2).unwrap();

    let not_result: String = result
        .chars()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => panic!("woops"),
        })
        .collect();
    let epsilon_rate = u32::from_str_radix(&not_result, 2).unwrap();

    gamma_rate * epsilon_rate
}

fn part2() {}
