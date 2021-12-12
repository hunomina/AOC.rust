use std::fs;

type Position = (usize, usize);

#[derive(Clone)]
struct Octopuses {
    octopuses: Vec<Vec<Octopus>>,
}

impl Octopuses {
    fn increase_all_energy(&mut self) {
        self.octopuses.iter_mut().for_each(|line| {
            line.iter_mut()
                .for_each(|octopus| octopus.increase_energy())
        });
    }

    fn try_flash_all(&mut self) -> u8 {
        (0..self.octopuses.len())
            .into_iter()
            .map(|i| {
                (0..self.octopuses[i].len())
                    .into_iter()
                    .map(|j| self.try_flash_at((i, j)))
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .sum()
    }

    fn try_flash_at(&mut self, position: Position) -> u8 {
        if self.octopuses[position.0][position.1].try_flash() {
            1 + self
                .get_neighbours(position)
                .into_iter()
                .fold(0, |acc, neighbour| {
                    self.octopuses[neighbour.0][neighbour.1].increase_energy();
                    acc + self.try_flash_at(neighbour)
                })
        } else {
            0
        }
    }

    fn get_neighbours(&self, position: Position) -> Vec<Position> {
        let mut neighbours = vec![];

        // all above neighbours
        if position.0 > 0 {
            if position.1 > 0 {
                neighbours.push((position.0 - 1, position.1 - 1));
            }
            neighbours.push((position.0 - 1, position.1));
            if let Some(_) = self.octopuses[position.0 - 1].get(position.1 + 1) {
                neighbours.push((position.0 - 1, position.1 + 1));
            }
        }

        // all bellow neighbours
        if let Some(l) = self.octopuses.get(position.0 + 1) {
            if position.1 > 0 {
                neighbours.push((position.0 + 1, position.1 - 1));
            }
            neighbours.push((position.0 + 1, position.1));
            if let Some(_) = l.get(position.1 + 1) {
                neighbours.push((position.0 + 1, position.1 + 1));
            }
        }

        // left neighbour
        if position.1 > 0 {
            neighbours.push((position.0, position.1 - 1));
        }

        // right neighbour
        if let Some(_) = self.octopuses[position.0].get(position.1 + 1) {
            neighbours.push((position.0, position.1 + 1));
        }

        neighbours
    }

    fn reset_flashes_state(&mut self) {
        self.octopuses.iter_mut().for_each(|line| {
            line.iter_mut().for_each(|octopus| {
                octopus.flashed = false;
            })
        });
    }

    fn count(&self) -> usize {
        self.octopuses.iter().fold(0, |acc, line| acc + line.len())
    }
}

#[derive(Clone)]
struct Octopus {
    energy_level: u8,
    flashed: bool,
}

impl Octopus {
    fn new(energy_level: u8) -> Self {
        Octopus {
            energy_level,
            flashed: false,
        }
    }

    fn increase_energy(&mut self) {
        if !self.flashed {
            self.energy_level += 1;
        }
    }

    fn try_flash(&mut self) -> bool {
        if !self.flashed && self.energy_level > 9 {
            self.flashed = true;
            self.energy_level = 0;
            true
        } else {
            false
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let octopuses = parse_input(&input);
    println!("part1 result {}", part1(octopuses.clone()));
    println!("part2 result {}", part2(octopuses.clone()));
}

fn parse_input(input: &str) -> Octopuses {
    Octopuses {
        octopuses: input
            .lines()
            .map(|line| line.chars().map(|c| Octopus::new(c as u8 - 0x30)).collect())
            .collect(),
    }
}

fn part1(mut octopuses: Octopuses) -> u32 {
    (0..100).fold(0, |mut acc, _| {
        octopuses.increase_all_energy();
        acc = acc + octopuses.try_flash_all() as u32;
        octopuses.reset_flashes_state();
        acc
    })
}

fn part2(mut octopuses: Octopuses) -> u32 {
    let mut step = 0;
    loop {
        octopuses.increase_all_energy();
        step += 1;
        if octopuses.try_flash_all() == octopuses.count() as u8 {
            break;
        } else {
            octopuses.reset_flashes_state();
        }
    }
    step
}
