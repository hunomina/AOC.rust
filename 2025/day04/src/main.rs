use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input = Vec<Vec<char>>;

fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: Input) -> usize {
    let mut count = 0;

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if can_be_removed(&input, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn part2(mut input: Input) -> usize {
    let initial_count_of_rolls = count_rolls_of_paper(&input);
    let mut previous_count_of_rolls = initial_count_of_rolls;

    loop {
        let mut to_remove = vec![];

        for row in 0..input.len() {
            for col in 0..input[row].len() {
                if can_be_removed(&input, row, col) {
                    to_remove.push((row, col));
                }
            }
        }

        for (row, col) in to_remove {
            input[row][col] = '.';
        }

        let current_count = count_rolls_of_paper(&input);
        if current_count == previous_count_of_rolls {
            break;
        }
        previous_count_of_rolls = current_count;
    }

    initial_count_of_rolls - previous_count_of_rolls
}

fn get_neighbours(grid: &[Vec<char>], row: usize, col: usize) -> Vec<Option<char>> {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ]
    .iter()
    .map(|(dr, dc)| {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        if new_row >= 0 && new_row < rows && new_col >= 0 && new_col < cols {
            Some(grid[new_row as usize][new_col as usize])
        } else {
            None
        }
    })
    .collect()
}

fn count_existing_neighbours(grid: &Input, row: usize, col: usize) -> usize {
    get_neighbours(grid, row, col)
        .iter()
        .filter(|n| n.is_some() && n.unwrap() == '@')
        .count()
}

fn can_be_removed(grid: &Input, row: usize, col: usize) -> bool {
    grid[row][col] == '@' && count_existing_neighbours(grid, row, col) < 4
}

fn count_rolls_of_paper(input: &Input) -> usize {
    input
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == '@')
        .count()
}
