use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input = Vec<Vec<u8>>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: Input) -> u32 {
    input
        .into_iter()
        .fold(0, |acc, bank| acc + solve::<u32>(bank, 2))
}

fn part2(input: Input) -> u64 {
    input
        .into_iter()
        .fold(0, |acc, bank| acc + solve::<u64>(bank, 12))
}

fn solve<T>(mut bank: Vec<u8>, iteration: usize) -> T
where
    T: From<u8> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
{
    let mut result = vec![];

    for i in (1..=iteration).rev() {
        let local_window = bank[..=bank.len() - i].to_vec();

        let (local_window_max_value, local_windows_max_index) =
            find_max_value_and_index(&local_window);

        result.push(local_window_max_value);

        bank = bank[local_windows_max_index + 1..bank.len()].to_vec();
    }

    result
        .into_iter()
        .fold(T::from(0), |acc, n| acc * T::from(10) + T::from(n))
}

fn find_max_value_and_index(vec: &[u8]) -> (u8, usize) {
    let (mut max_value, mut max_index) = (vec[0], 0);

    for (k, v) in vec.iter().enumerate() {
        if *v > max_value {
            (max_value, max_index) = (*v, k);
        }

        // why not, but doesn't really help performance
        //if max_value == 9 {
        //    break;
        //}
    }

    (max_value, max_index)
}
