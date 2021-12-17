use std::fs;

type Position = (usize, usize);

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let caves = parse_input(&input);

    println!("part1 result {}", part1(caves.clone()));
    println!("part2 result {}", part2(caves.clone()));
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c as u8 - 0x30).collect())
        .collect()
}

fn part1(caves: Vec<Vec<u8>>) -> u32 {
    get_low_points(&caves)
        .into_iter()
        .map(|(i, j)| caves[i][j])
        .fold(0, |acc, n| acc + 1 + n as u32)
}

fn part2(caves: Vec<Vec<u8>>) -> usize {
    let mut bassins_sizes: Vec<_> = get_low_points(&caves)
        .into_iter()
        .map(|low_point| get_bassin(&caves, low_point).len())
        .collect();

    // could be optimized by storing the top 3 sizes in a tuple (0, 0, 0) and replacing the values by reducing the bassins on it
    bassins_sizes.sort();
    bassins_sizes.reverse();

    bassins_sizes[0..3].into_iter().fold(1, |acc, n| acc * n)
}

fn get_bassin(caves: &Vec<Vec<u8>>, origin: Position) -> Vec<Position> {
    let mut bassin = vec![origin];

    get_higher_neighbours(caves, origin)
        .into_iter()
        .for_each(|higher_neighbour| {
            get_bassin(caves, higher_neighbour)
                .into_iter()
                .for_each(|cave| {
                    if !bassin.contains(&cave) {
                        bassin.push(cave);
                    }
                });
        });

    bassin
}

fn get_low_points(caves: &Vec<Vec<u8>>) -> Vec<Position> {
    caves
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter().enumerate().filter_map(move |(j, cell)| {
                if let None = get_neighbours(&caves, (i, j))
                    .into_iter()
                    .find(|(x, y)| get_cave(&caves, (*x, *y)).unwrap() <= cell)
                {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect()
}

fn get_higher_neighbours(caves: &Vec<Vec<u8>>, from: Position) -> Vec<Position> {
    get_neighbours(caves, from)
        .into_iter()
        .filter(|neighbour| {
            caves[neighbour.0][neighbour.1] != 9
                && caves[neighbour.0][neighbour.1] > caves[from.0][from.1]
        })
        .collect()
}

fn get_neighbours(caves: &Vec<Vec<u8>>, from: Position) -> Vec<Position> {
    let mut neighbours = vec![];

    if from.0 > 0 {
        neighbours.push((from.0 - 1, from.1));
    }

    if from.1 > 0 {
        neighbours.push((from.0, from.1 - 1));
    }

    if let Some(_) = get_cave(caves, (from.0 + 1, from.1)) {
        neighbours.push((from.0 + 1, from.1));
    }

    if let Some(_) = get_cave(caves, (from.0, from.1 + 1)) {
        neighbours.push((from.0, from.1 + 1));
    }

    neighbours
}

fn get_cave(caves: &Vec<Vec<u8>>, position: Position) -> Option<&u8> {
    caves.get(position.0).and_then(|line| line.get(position.1))
}
