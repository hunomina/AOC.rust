use std::fs;

fn main() {
    println!(
        "{}",
        fs::read_to_string("src/input.txt")
            .unwrap()
            .split("\n\n")
            .fold(0, |acc, group| {
                let group_size = group.split('\n').count();
                acc + ('a'..='z').fold(0, |acc2, char| {
                    acc2 + (group.matches(char).count() == group_size) as usize
                })
            })
    );
}
