use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input = (Vec<(u32, u32)>, Vec<Vec<u32>>);

fn parse_input(input: &str) -> Input {
    let mut input = input.split("\n\n");

    let rules = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut line = line.split('|');
            (
                line.next().unwrap().parse().unwrap(),
                line.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let updates = input
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn part1(input: Input) -> u32 {
    let (rules, updates) = input;

    updates
        .into_iter()
        .filter(|update| is_valid_update(update, &rules).0)
        .fold(0, |acc, update| acc + update[update.len() / 2])
}

fn part2(input: Input) -> u32 {
    let (rules, updates) = input;

    updates
        .into_iter()
        .filter_map(|mut update| {
            let (is_a_valid_update, mut invalid_page_index, mut conflicting_page_index) =
                is_valid_update(&update, &rules);

            if is_a_valid_update {
                return None;
            }

            loop {
                update.swap(conflicting_page_index, invalid_page_index);

                let is_a_valid_update = is_valid_update(&update, &rules);

                if is_a_valid_update.0 {
                    break;
                }

                invalid_page_index = is_a_valid_update.1;
                conflicting_page_index = is_a_valid_update.2;
            }

            Some(update)
        })
        .fold(0, |acc, update| acc + update[update.len() / 2])
}

fn get_rules_for_page(rules: &[(u32, u32)], page: u32) -> Vec<&(u32, u32)> {
    rules
        .iter()
        .filter(|(a, b)| *a == page || *b == page)
        .collect()
}

fn is_valid_update(update: &Vec<u32>, rules: &[(u32, u32)]) -> (bool, usize, usize) {
    for (i, page) in update.iter().enumerate() {
        let current_rules = get_rules_for_page(rules, *page);

        for rule in current_rules {
            if *page == rule.0 {
                if let Some(n) = update.as_slice()[..i].iter().position(|r| *r == rule.1) {
                    return (false, i, n);
                }
            } else if *page == rule.1 {
                if let Some(n) = update.as_slice()[i..].iter().position(|r| *r == rule.0) {
                    return (false, i, i + n);
                }
            } else {
                panic!("💣");
            }
        }
    }

    (true, 0, 0)
}
