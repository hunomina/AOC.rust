use std::collections::HashMap;

const ALPAHBET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let part1 = input
        .lines()
        .map(|l| {
            let (first_compartment, second_partment) = l.split_at(l.len() / 2);
            let l = first_compartment
                .chars()
                .find(|c| second_partment.contains([*c]))
                .unwrap();
            ALPAHBET.find(l).unwrap() + 1
        })
        .sum::<usize>();
    let part2 = input
        .lines()
        .enumerate()
        .fold(vec![], |mut acc: Vec<Vec<&str>>, (l_index, l_value)| {
            if l_index % 3 == 0 {
                acc.push(vec![]);
            }
            acc.last_mut().unwrap().push(l_value);
            acc
        })
        .into_iter()
        .map(|group| {
            let group_badge = group
                .into_iter()
                .map(|l| {
                    let mut l = l.chars().collect::<Vec<_>>();
                    l.sort();
                    l.dedup();
                    l.into_iter()
                        .fold(HashMap::new(), |mut acc: HashMap<char, i32>, c| {
                            if let Some(x) = acc.get_mut(&c) {
                                *x += 1;
                            } else {
                                acc.insert(c, 1);
                            }
                            acc
                        })
                })
                .fold(HashMap::new(), |mut acc, map| {
                    for (key, value) in map.into_iter() {
                        if let Some(x) = acc.get_mut(&key) {
                            *x += 1;
                        } else {
                            acc.insert(key, value);
                        }
                    }
                    acc
                })
                .into_iter()
                .find(|(_, value)| *value >= 3)
                .unwrap()
                .0;
            ALPAHBET.find(group_badge).unwrap() + 1
        })
        .sum::<usize>();
    //         ;
    println!("part1: {:?}", part1);
    println!("part2: {:?}", part2);
}
