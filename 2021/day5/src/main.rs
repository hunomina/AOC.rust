use std::fs;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn from_str(s: &str) -> Self {
        let mut splitted_string = s.split(',');
        Self {
            x: splitted_string.nth(0).unwrap().parse().unwrap(),
            y: splitted_string.nth(0).unwrap().parse().unwrap(),
        }
    }

    fn has_same_x_as(&self, other: &Point) -> bool {
        self.x == other.x
    }

    fn has_same_y_as(&self, other: &Point) -> bool {
        self.y == other.y
    }
}

#[derive(Clone, Copy)]
struct Vector {
    from: Point,
    to: Point,
}

impl Vector {
    fn is_horizontal(&self) -> bool {
        self.from.has_same_x_as(&self.to)
    }

    fn is_vertical(&self) -> bool {
        self.from.has_same_y_as(&self.to)
    }

    fn is_diagonal(&self) -> bool {
        i32::abs(self.from.x as i32 - self.to.x as i32)
            == i32::abs(self.from.y as i32 - self.to.y as i32)
    }

    fn get_all_points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            (if self.from.y > self.to.y {
                self.to.y..=self.from.y
            } else {
                self.from.y..=self.to.y
            })
            .map(|y| Point { x: self.from.x, y })
            .collect()
        } else if self.is_vertical() {
            (if self.from.x < self.to.x {
                self.from.x..=self.to.x
            } else {
                self.to.x..=self.from.x
            })
            .map(|x| Point { x, y: self.from.y })
            .collect()
        } else if self.is_diagonal() {
            let mut points = vec![self.from.clone()];
            let mut last_inserted_point = *points.last().unwrap();

            while last_inserted_point != self.to {
                let p = Point {
                    x: if last_inserted_point.x < self.to.x {
                        last_inserted_point.x + 1
                    } else {
                        last_inserted_point.x - 1
                    },
                    y: if last_inserted_point.y < self.to.y {
                        last_inserted_point.y + 1
                    } else {
                        last_inserted_point.y - 1
                    },
                };

                points.push(p.clone());
                last_inserted_point = p;
            }

            points
        } else {
            vec![]
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let vents = parse_input(&input);

    println!("part1 result {}", part1(vents.clone()));
    println!("part2 result {}", part2(vents.clone()));
}

fn parse_input(input: &str) -> Vec<Vector> {
    input
        .lines()
        .map(|line| {
            let mut splitted_line = line.split(" -> ");
            Vector {
                from: Point::from_str(splitted_line.nth(0).unwrap()),
                to: Point::from_str(splitted_line.nth(0).unwrap()),
            }
        })
        .collect()
}

fn part1(mut vents: Vec<Vector>) -> u32 {
    vents = vents
        .into_iter()
        .filter(|vector| vector.is_horizontal() || vector.is_vertical())
        .collect();

    let mut grid = create_grid(
        get_max_y(&vents) as usize + 1,
        get_max_x(&vents) as usize + 1,
    );

    vents
        .into_iter()
        .map(|vent| vent.get_all_points())
        .flatten()
        .for_each(|point| grid[point.y as usize][point.x as usize] += 1);

    grid.into_iter()
        .map(|line| {
            line.into_iter()
                .fold(0u32, |acc, value| acc + (value > 1) as u32)
        })
        .sum()
}

fn part2(vents: Vec<Vector>) -> u32 {
    let mut grid = create_grid(
        get_max_y(&vents) as usize + 1,
        get_max_x(&vents) as usize + 1,
    );

    vents
        .into_iter()
        .map(|vent| vent.get_all_points())
        .flatten()
        .for_each(|point| grid[point.y as usize][point.x as usize] += 1);

    grid.into_iter()
        .map(|line| {
            line.into_iter()
                .fold(0u32, |acc, value| acc + (value > 1) as u32)
        })
        .sum()
}

fn get_max_x(vectors: &Vec<Vector>) -> u32 {
    vectors.iter().fold(0, |acc, vector| {
        if vector.from.x > acc {
            vector.from.x
        } else if vector.to.x > acc {
            vector.to.x
        } else {
            acc
        }
    })
}

fn get_max_y(vectors: &Vec<Vector>) -> u32 {
    vectors.iter().fold(0, |acc, vector| {
        if vector.from.y > acc {
            vector.from.y
        } else if vector.to.y > acc {
            vector.to.y
        } else {
            acc
        }
    })
}

fn create_grid(line: usize, column: usize) -> Vec<Vec<u8>> {
    vec![vec![0u8; column]; line]
}
