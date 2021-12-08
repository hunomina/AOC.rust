use std::{collections::HashMap, fs};

const SIZES: [usize; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let lines = parse_input(&input);

    println!("part1 result {}", part1(lines.clone()));
    println!("part2 result {}", part2(lines.clone()));
}

fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .lines()
        .map(|line| {
            let mut splitted_line = line.split(" | ");
            (
                splitted_line.nth(0).unwrap().split(' ').collect(),
                splitted_line.nth(0).unwrap().split(' ').collect(),
            )
        })
        .collect()
}

fn part1(lines: Vec<(Vec<&str>, Vec<&str>)>) -> u32 {
    lines
        .into_iter()
        .map(|(_, outputs)| {
            outputs.into_iter().fold(0, |acc, output| {
                if output.len() == SIZES[1]
                    || output.len() == SIZES[4]
                    || output.len() == SIZES[7]
                    || output.len() == SIZES[8]
                {
                    acc + 1
                } else {
                    acc
                }
            })
        })
        .sum()
}

fn part2(lines: Vec<(Vec<&str>, Vec<&str>)>) -> u32 {
    lines
        .into_iter()
        .map(|(inputs, outputs)| {
            let mut numbers = HashMap::new();

            numbers.insert(1, *inputs.iter().find(|input| input.len() == 2).unwrap());
            numbers.insert(4, *inputs.iter().find(|input| input.len() == 4).unwrap());
            numbers.insert(7, *inputs.iter().find(|input| input.len() == 3).unwrap());
            numbers.insert(8, *inputs.iter().find(|input| input.len() == 7).unwrap());

            // 9 is the only 6 chars digit that contains 4
            numbers.insert(
                9,
                *inputs
                    .iter()
                    .find(|s| s.len() == 6 && string_reduction(s, numbers[&4]).len() == 2)
                    .unwrap(),
            );

            // 6 is the only 6 chars digit that does not contains 7
            numbers.insert(
                6,
                *inputs
                    .iter()
                    .find(|s| s.len() == 6 && s.len() != merge_two_numbers(s, numbers[&7]).len())
                    .unwrap(),
            );

            // 0 is the last 6 chars digit
            numbers.insert(
                0,
                *inputs
                    .iter()
                    .find(|s| s.len() == 6 && **s != numbers[&6] && **s != numbers[&9])
                    .unwrap(),
            );

            // 3 is the only 5 chars digit that contains 1
            numbers.insert(
                3,
                *inputs
                    .iter()
                    .find(|s| s.len() == 5 && string_reduction(s, numbers[&1]).len() == 3)
                    .unwrap(),
            );

            // 5 is the only 5 chars digit contained by 6
            numbers.insert(
                5,
                *inputs
                    .iter()
                    .find(|s| s.len() == 5 && string_reduction(numbers[&6], s).len() == 1)
                    .unwrap(),
            );

            // 2 is the last 5 chars digit
            numbers.insert(
                2,
                *inputs
                    .iter()
                    .find(|s| s.len() == 5 && **s != numbers[&3] && **s != numbers[&5])
                    .unwrap(),
            );

            // reorder chars to easy comparison
            let numbers: HashMap<_, _> = numbers
                .into_iter()
                .map(|(key, chars)| (key, order_chars(chars)))
                .collect();

            // fold outputs true values into a string
            // then parse it into an integer
            outputs
                .into_iter()
                .map(order_chars)
                .fold(String::new(), |mut acc, output| {
                    let t = numbers
                        .iter()
                        .find_map(|(key, value)| if output == *value { Some(key) } else { None })
                        .unwrap();
                    acc.push_str(t.to_string().as_str());
                    acc
                })
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn order_chars(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<_>>();
    chars.sort_by(|a, b| b.cmp(a));
    chars.into_iter().collect()
}

fn merge_two_numbers(a: &str, b: &str) -> String {
    let mut s = String::new();

    a.chars().into_iter().for_each(|c| {
        if !s.contains(c) {
            s.push(c);
        }
    });

    b.chars().into_iter().for_each(|c| {
        if !s.contains(c) {
            s.push(c);
        }
    });

    s
}

fn string_reduction(a: &str, b: &str) -> String {
    let (shortest, longest) = if a.len() < b.len() { (a, b) } else { (b, a) };

    longest
        .chars()
        .filter_map(|c| if shortest.contains(c) { None } else { Some(c) })
        .collect()
}

// <--- TRIED TO DO IT IN A MORE ELEGANT WAY --->

// fn part2(lines: Vec<(Vec<&str>, Vec<&str>)>) -> u32 {
//     let numbers_mapping = get_numbers_mapping();
//
//     lines
//         .into_iter()
//         .map(|(mixed_up_numbers, outputs)| {
//             let mut potentials_mapping = get_potentials_mapping();
//             mixed_up_numbers.into_iter().for_each(|mixed_up_number| {
//                 let exploded_mixed_up_number = mixed_up_number.chars().collect::<Vec<_>>();
//
//                 let mut encountered_chars = numbers_mapping
//                     .iter()
//                     .filter_map(|(_, chars)| {
//                         if chars.len() == exploded_mixed_up_number.len() {
//                             Some(chars)
//                         } else {
//                             None
//                         }
//                     })
//                     .flatten()
//                     .map(|c| *c)
//                     .collect::<Vec<_>>();
//                 encountered_chars.sort();
//                 encountered_chars.dedup();
//
//                 println!("mixed up numbers {:?}", exploded_mixed_up_number);
//                 println!("encountered chars {:?}", encountered_chars);
//
//                 encountered_chars.into_iter().for_each(|encountered_char| {
//                     let loc = potentials_mapping[&encountered_char].clone();
//                     potentials_mapping.insert(
//                         encountered_char,
//                         loc.iter()
//                             .filter(|c| exploded_mixed_up_number.contains(c))
//                             .map(|c| *c)
//                             .collect(),
//                     );
//                 });
//
//                 println!("{:?}", potentials_mapping);
//             });
//             0
//         })
//         .sum()
// }
//
// fn get_numbers_mapping() -> HashMap<usize, Vec<char>> {
//     HashMap::from([
//         (0, vec!['a', 'b', 'c', 'e', 'f', 'g']),
//         (1, vec!['c', 'f']),
//         (2, vec!['a', 'c', 'd', 'e', 'g']),
//         (3, vec!['a', 'c', 'd', 'f', 'g']),
//         (4, vec!['b', 'c', 'd', 'f']),
//         (5, vec!['a', 'b', 'd', 'f', 'g']),
//         (6, vec!['a', 'b', 'd', 'e', 'f', 'g']),
//         (7, vec!['a', 'c', 'f']),
//         (8, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
//         (9, vec!['a', 'b', 'c', 'd', 'f', 'g']),
//     ])
// }
//
// fn get_potentials_mapping() -> HashMap<char, Vec<char>> {
//     let a_to_g: Vec<char> = ('a'..='g').into_iter().collect();
//     let mut potentials_mapping = HashMap::new();
//
//     a_to_g.iter().for_each(|c| {
//         potentials_mapping.insert(*c, a_to_g.clone());
//     });
//
//     potentials_mapping
// }
