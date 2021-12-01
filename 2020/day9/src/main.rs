use std::fs;

const PREAMBULE_SIZE: usize = 25;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let numbers = parse_input(input);

    // PART 1

    let mut first_invalid_number = numbers[PREAMBULE_SIZE..]
        .iter()
        .enumerate()
        .find(|number| {
            !is_valid(
                number.1.clone(),
                numbers[number.0..PREAMBULE_SIZE + number.0].to_vec(),
            )
        })
        .unwrap();

    first_invalid_number.0 += PREAMBULE_SIZE; // correct number position in input by adding a padding equal to the preambule
    println!("{:?}", first_invalid_number);

    // PART 2
    let candidates = numbers[0..first_invalid_number.0].to_vec();

    let r = candidates
        .iter()
        .enumerate()
        .find_map(|(key, _)| {
            find_possible_contiguous_sum(*first_invalid_number.1, key, &candidates)
        })
        .unwrap();

    println!("{:?}", r.iter().min().unwrap() + r.iter().max().unwrap());
}

fn parse_input(input: String) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn is_valid(number: usize, before: Vec<usize>) -> bool {
    before
        .iter()
        .find(|n| {
            before
                .iter()
                .find(|n2| n != n2 && **n + **n2 == number)
                .is_some()
        })
        .is_some()
}

type Sum = Vec<usize>;

fn find_possible_contiguous_sum(
    objective: usize,
    mut start_index: usize,
    candidates: &Vec<usize>,
) -> Option<Sum> {
    let mut total = vec![];
    while objective > total.iter().sum() {
        total.push(candidates[start_index]);
        start_index += 1;
    }

    if objective < total.iter().sum() {
        return None;
    }
    return Some(total);
}
