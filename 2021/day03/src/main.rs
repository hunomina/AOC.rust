use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let entries = parse_input(&input);

    let part1 = part1(entries.clone());
    println!("part1 result {}", part1);

    let part2 = part2(entries.clone());
    println!("part2 result {}", part2);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|s| s.bytes().map(|bit| bit as char).collect())
        .collect()
}

fn part1(entries: Vec<Vec<char>>) -> u32 {
    let entry_size = entries.get(0).unwrap().len();
    let result: String = (0..entry_size)
        .map(|i| get_most_common_bit_for_column(&entries, i))
        .collect();
    let gamma_rate = u32::from_str_radix(&result, 2).unwrap();

    let reversed_result = reverse_bits(result.clone());
    let epsilon_rate = u32::from_str_radix(&reversed_result, 2).unwrap();

    gamma_rate * epsilon_rate
}

fn part2(entries: Vec<Vec<char>>) -> u32 {
    find_oxygen_generator_rating(entries.clone()) * find_co_two_scrubber_rating(entries.clone())
}

fn reduce_column(entries: &Vec<Vec<char>>, index: usize) -> i32 {
    entries.iter().map(|e| e[index]).fold(0, |acc, c| match c {
        '0' => acc - 1,
        '1' => acc + 1,
        _ => panic!("woops"),
    })
}

fn get_most_common_bit_for_column(entries: &Vec<Vec<char>>, index: usize) -> char {
    if reduce_column(entries, index) >= 0 {
        '1'
    } else {
        '0'
    }
}

fn get_less_common_bit_for_column(entries: &Vec<Vec<char>>, index: usize) -> char {
    if reduce_column(entries, index) < 0 {
        '1'
    } else {
        '0'
    }
}

fn reverse_bits(value: String) -> String {
    value
        .chars()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => panic!("woops"),
        })
        .collect()
}

fn find_oxygen_generator_rating(mut entries: Vec<Vec<char>>) -> u32 {
    let entry_size = entries.get(0).unwrap().len();

    for i in 0..entry_size {
        let most_common_bit = get_most_common_bit_for_column(&entries, i);
        entries = entries
            .into_iter()
            .filter(|entry| *entry.get(i).unwrap() as char == most_common_bit)
            .collect();
        if entries.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(
        entries
            .get(0)
            .unwrap()
            .into_iter()
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap()
}

fn find_co_two_scrubber_rating(mut entries: Vec<Vec<char>>) -> u32 {
    let entry_size = entries.get(0).unwrap().len();

    for i in 0..entry_size {
        let less_common_bit = get_less_common_bit_for_column(&entries, i);
        entries = entries
            .into_iter()
            .filter(|entry| *entry.get(i).unwrap() == less_common_bit)
            .collect();
        if entries.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(
        entries
            .get(0)
            .unwrap()
            .into_iter()
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap()
}
