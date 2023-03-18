use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(&input);

    println!("part1: {:?}", part1(input.clone())); // 1870
    println!("part2: {:?}", part2(input));
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn viewing_distance_from_left(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> usize {
    let v = grid[line][column];

    let mut i = 1;
    while let Some(n) = grid[line].get(column - i) {
        if *n >= v {
            return i;
        }
        if column - i == 0 {
            break;
        }
        i += 1;
    }
    i
}

fn viewing_distance_from_right(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> usize {
    let v = grid[line][column];

    let mut i = 1;
    while let Some(n) = grid[line].get(column + i) {
        if *n >= v {
            return i;
        }
        i += 1;
    }
    i - 1
}

fn viewing_distance_from_above(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> usize {
    let v = grid[line][column];

    let mut i = 1;
    while let Some(n) = grid.get(line - i) {
        if *n.get(column).unwrap() >= v {
            return i;
        }
        if line - i == 0 {
            break;
        }
        i += 1;
    }
    i
}

fn viewing_distance_from_bellow(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> usize {
    let v = grid[line][column];

    let mut i = 1;
    while let Some(n) = grid.get(line + i) {
        if *n.get(column).unwrap() >= v {
            return i;
        }

        i += 1;
    }
    i - 1
}

fn is_visible(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> bool {
    let v = grid[line][column];
    (viewing_distance_from_left(grid, line, column) == column && v > grid[line][0])
        || (viewing_distance_from_right(grid, line, column) == grid[0].len() - column - 1
            && v > grid[line][grid[line].len() - 1])
        || (viewing_distance_from_above(grid, line, column) == line && v > grid[0][column])
        || (viewing_distance_from_bellow(grid, line, column) == grid.len() - line - 1
            && v > grid[grid.len() - 1][column])
}

fn compute_scenic_score(grid: &Vec<Vec<u8>>, line: usize, column: usize) -> usize {
    viewing_distance_from_left(grid, line, column)
        * viewing_distance_from_right(grid, line, column)
        * viewing_distance_from_above(grid, line, column)
        * viewing_distance_from_bellow(grid, line, column)
}

fn part1(grid: Vec<Vec<u8>>) -> usize {
    (1..grid.len() - 1)
        .map(|i| {
            (1..grid[i].len() - 1)
                .map(|j| if is_visible(&grid, i, j) { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
        + grid.len() * 2
        + (grid[0].len() - 2) * 2
}

fn part2(grid: Vec<Vec<u8>>) -> usize {
    (1..grid.len() - 1)
        .map(|i| {
            (1..grid[i].len() - 1)
                .map(|j| compute_scenic_score(&grid, i, j))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}
