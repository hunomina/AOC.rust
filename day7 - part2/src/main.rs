// https://adventofcode.com/2020/day/7/input

use std::{collections::HashMap, fs};

#[derive(Debug, Hash)]
struct BagInfos {
    color: String,
    accent: String,
}

impl PartialEq for BagInfos {
    fn eq(&self, other: &Self) -> bool {
        (self.color.clone(), self.accent.clone()) == (other.color.clone(), other.accent.clone())
    }
}
impl Eq for BagInfos {}

type BagContent = HashMap<BagInfos, u8>;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let bags: HashMap<BagInfos, BagContent> = parse_input(input);

    println!(
        "{:?}",
        count_contained_bags(
            &BagInfos {
                color: "gold".to_string(),
                accent: "shiny".to_string()
            },
            &bags
        )
    );
}

fn count_contained_bags(bag: &BagInfos, bags: &HashMap<BagInfos, BagContent>) -> usize {
    bags.get(bag).unwrap().iter().fold(0, |acc, bag| {
        acc + bag.1.clone() as usize + bag.1.clone() as usize * count_contained_bags(bag.0, bags)
    })
}

fn parse_input(input: String) -> HashMap<BagInfos, BagContent> {
    let mut bags: HashMap<BagInfos, BagContent> = HashMap::new();
    input.lines().for_each(|line| {
        let mut splited_line = line.split(" bags contain ");
        let bag_infos = parse_bag_infos(splited_line.nth(0).unwrap());
        let mut contains: BagContent = HashMap::new();

        let contains_str = splited_line.nth(0).unwrap().trim_end_matches('.');

        if contains_str == "no other bags" {
            bags.insert(bag_infos, contains);
            return;
        }

        contains_str.split(", ").for_each(|contain| {
            let mut splited_contain = contain.split_whitespace();
            let count = splited_contain.nth(0).unwrap().parse().unwrap();
            contains.insert(
                BagInfos {
                    accent: splited_contain.nth(0).unwrap().to_string(),
                    color: splited_contain.nth(0).unwrap().to_string(),
                },
                count,
            );
        });
        bags.insert(bag_infos, contains);
    });
    bags
}

fn parse_bag_infos(s: &str) -> BagInfos {
    let mut split = s.split_whitespace();
    BagInfos {
        accent: split.nth(0).unwrap().to_string(),
        color: split.nth(0).unwrap().to_string(),
    }
}
