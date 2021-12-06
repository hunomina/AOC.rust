use std::{fs, iter::StepBy, ops::Range};

const INITIAL_TIMER: usize = 9;
const RESET_TIMER: usize = 7;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let fishes = parse_input(&input);

    println!("part1 result {}", part1(fishes.clone()));
    println!("part2 result {}", part2(fishes.clone()));
}

fn parse_input(input: &str) -> Vec<u8> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

fn part1(fishes: Vec<u8>) -> u64 {
    get_birth_count_for_period(fishes, 80)
}

fn part2(fishes: Vec<u8>) -> u64 {
    get_birth_count_for_period(fishes, 256)
}

fn get_birth_count_for_period(initial_fishes: Vec<u8>, period_in_days: usize) -> u64 {
    let initial_fishes_len = initial_fishes.len();
    let mut calendar = vec![0u64; period_in_days];

    initial_fishes.into_iter().for_each(|timer| {
        let day = timer as usize;
        calendar[day] += 1; // a child will be born in timer + 1 day
        get_children_birth_dates(day, period_in_days).for_each(|d| calendar[d] += 1);
    });

    // stops at "max_day - child first reproduction delay"
    // otherwise it will overflow when we try to create the first child (at +9 days)
    (0..period_in_days - INITIAL_TIMER).for_each(|day| {
        let fish_count = calendar[day];

        if fish_count == 0 {
            return;
        }

        let first_birth_day = day + INITIAL_TIMER;

        calendar[first_birth_day] += fish_count;
        get_children_birth_dates(first_birth_day, period_in_days)
            .for_each(|d| calendar[d] += fish_count);
    });

    // original parents + sum of all new possible
    initial_fishes_len as u64 + calendar.into_iter().sum::<u64>()
}

fn get_children_birth_dates(from_day: usize, to_day: usize) -> StepBy<Range<usize>> {
    (from_day + RESET_TIMER..to_day).step_by(RESET_TIMER)
}
