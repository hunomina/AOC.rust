use std::{collections::HashMap, fs};

const START: &str = "start";
const END: &str = "end";

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let caves = parse_input(&input);

    println!("part1 result {}", part1(caves.clone()));
    println!("part2 result {}", part2(caves.clone()));
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut caves: HashMap<&str, Vec<&str>> = HashMap::new();

    input.lines().for_each(|line| {
        let mut splitted_line = line.split('-');
        let (from, to) = (splitted_line.nth(0).unwrap(), splitted_line.nth(0).unwrap());

        if caves.contains_key(&from) {
            caves.get_mut(&from).unwrap().push(to.clone());
        } else {
            caves.insert(from.clone(), vec![to.clone()]);
        }

        if caves.contains_key(&to) {
            caves.get_mut(&to).unwrap().push(from.clone());
        } else {
            caves.insert(to.clone(), vec![from.clone()]);
        }
    });

    caves
}

fn part1(caves: HashMap<&str, Vec<&str>>) -> usize {
    get_paths_to_end_part(&caves, vec![START], can_visit_cave_part1).len()
}

fn part2(caves: HashMap<&str, Vec<&str>>) -> usize {
    get_paths_to_end_part(&caves, vec![START], can_visit_cave_part2).len()
}

fn get_paths_to_end_part<'a>(
    caves: &'a HashMap<&str, Vec<&str>>,
    from: Vec<&'a str>,
    can_visit_cave: fn(&Vec<&str>, &str) -> bool,
) -> Vec<Vec<&'a str>> {
    caves
        .get(from.last().unwrap())
        .unwrap()
        .into_iter()
        .filter_map(|linked_cave| {
            if *linked_cave == END {
                let mut local_from = from.clone();
                local_from.push(linked_cave);
                Some(vec![local_from])
            } else if can_visit_cave(&from, linked_cave) {
                let mut local_from = from.clone();
                local_from.push(linked_cave);
                Some(get_paths_to_end_part(&caves, local_from, can_visit_cave))
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn is_small_cave(cave: &str) -> bool {
    cave.chars().nth(0).unwrap().is_lowercase()
}

fn can_visit_cave_part1(history: &Vec<&str>, cave: &str) -> bool {
    if cave == START || cave == END {
        false
    } else if is_small_cave(cave) {
        count_cave_occurence(&history, cave) == 0
    } else {
        true
    }
}

fn can_visit_cave_part2(history: &Vec<&str>, cave: &str) -> bool {
    if cave == START || cave == END {
        false
    } else if is_small_cave(cave) {
        count_cave_occurence(&history, cave) == 0
            || count_max_occurence_of_small_caves(&history) < 2
    } else {
        true
    }
}

fn count_cave_occurence(history: &Vec<&str>, cave: &str) -> usize {
    history.iter().fold(0, |acc, c| acc + (*c == cave) as usize)
}

fn count_max_occurence_of_small_caves(history: &Vec<&str>) -> usize {
    history
        .iter()
        .filter(|cave| is_small_cave(cave))
        .fold(HashMap::new(), |mut m, x| {
            *m.entry(x).or_default() += 1;
            m
        })
        .into_iter()
        .max_by_key(|(_, v)| *v)
        .unwrap()
        .1
}
