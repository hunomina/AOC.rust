use std::fs;
fn main() {
    let mut elves_calories = fs::read_to_string("src/input.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|n| n.parse::<u32>().unwrap())
                .fold(0, |acc, n| acc + n)
        })
        .collect::<Vec<_>>();
    elves_calories.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", elves_calories[0]);
    println!(
        "Part 2: {}",
        elves_calories[0] + elves_calories[1] + elves_calories[2]
    );
}
