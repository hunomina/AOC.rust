use std::fs;

enum EntryState {
    Valid,
    Incomplete(usize),
    Corrupted(usize, char),
}

impl EntryState {
    fn is_incomplete(&self) -> bool {
        match self {
            EntryState::Incomplete(_) => true,
            _ => false,
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let entries = parse_input(&input);

    println!("part1 result {}", part1(entries.clone()));
    println!("part1 result {}", part2(entries.clone()));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(mut entries: Vec<Vec<char>>) -> u32 {
    entries
        .iter_mut()
        .map(|entry| validate_entry(entry, 0))
        .fold(0, |acc, state| {
            acc + match state {
                EntryState::Corrupted(_, c) => get_point_for_char_part1(&c),
                _ => 0,
            }
        })
}

fn part2(mut entries: Vec<Vec<char>>) -> u64 {
    let mut added_char_collection: Vec<_> = entries
        .iter_mut()
        .filter_map(|entry| validate_entry(entry, 0).is_incomplete().then(|| entry))
        .map(|incomplete_entry| {
            // since entry is passed as a mutable ref on theprevious filter_map
            // all valid pairs have already been removed from it in order to detect it as incomplete
            let mut added_chars: Vec<_> = incomplete_entry.iter().map(get_pair).collect();
            added_chars.reverse();
            added_chars
                .into_iter()
                .fold(0, |acc, c| acc * 5 + get_point_for_char_part2(&c))
        })
        .collect();

    added_char_collection.sort();
    added_char_collection[added_char_collection.len() / 2]
}

fn validate_entry(entry: &mut Vec<char>, mut position: usize) -> EntryState {
    while let Some(current) = entry.get(position) {
        // zero is a special case because it's impossible to get the previous value
        // so impossible to check for pair is entry[0] is a closing character
        if position == 0 {
            if !is_opening_char(&entry[position]) {
                return EntryState::Corrupted(position, *current);
            }
            // can be skipped otherwise
            position = 1;
            continue;
        }

        if is_opening_char(&current) {
            // if current is an opening then go next check if it's its pair
            let next_validation = validate_entry(entry, position + 1);
            if let EntryState::Valid = next_validation {
                // remove the pair and go next
                entry.remove(position + 1);
                entry.remove(position);
                continue;
            }
            return next_validation;
        }

        if form_a_pair(&entry[position - 1], &current) {
            // if current is a closing char and forms a pair with previous, then it's VALID
            return EntryState::Valid;
        } else {
            // if current is a closing char and does not forms a pair with previous, then it's a CORRUPTED one
            return EntryState::Corrupted(position, *current);
        }
    }

    EntryState::Incomplete(position - 1)
}

fn is_opening_char(c: &char) -> bool {
    *c == '(' || *c == '{' || *c == '[' || *c == '<'
}

fn form_a_pair(a: &char, b: &char) -> bool {
    get_pair(&a) == *b
}

fn get_pair(c: &char) -> char {
    match c {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => panic!("character not supported"),
    }
}

fn get_point_for_char_part1(c: &char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_point_for_char_part2(c: &char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}
<<<<<<< HEAD

fn get_point_for_added_chars(chars: Vec<char>) -> u64 {
    chars
        .into_iter()
        .fold(0, |acc, c| acc * 5 + get_point_for_char_part2(&c))
}
=======
>>>>>>> fd3191a01f09c57438e597344effadcd5c003478
