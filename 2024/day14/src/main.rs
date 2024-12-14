use std::{fs, time::Instant};

use regex::Regex;

use macroquad::prelude::*;

const MAP_SIZE: (usize, usize) = (101, 103);
const MAX_WINDOW_SIZE: i32 = 1000;
const CELL_WIDTH: f32 = (MAX_WINDOW_SIZE / MAP_SIZE.1 as i32) as f32;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Advent of Code 2024 - Day 14"),
        window_width: (MAP_SIZE.0 as f32 * CELL_WIDTH) as i32,
        window_height: (MAP_SIZE.1 as f32 * CELL_WIDTH) as i32,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());
    let mut robots = input.clone();

    let mut last_modified = Instant::now();
    let mut i = 0;

    loop {
        clear_background(BLACK);

        draw_robots(&robots).await;

        if is_key_down(KeyCode::A) {
            let now = Instant::now();
            if now.duration_since(last_modified).as_millis() > 100 {
                last_modified = now;
                i = if i > 0 { i - 1 } else { i };
                robots = get_next_positions(input.clone(), i);
            }
        } else if is_key_down(KeyCode::Z) {
            let now = Instant::now();
            if now.duration_since(last_modified).as_millis() > 100 {
                last_modified = now;
                i += 1;
                robots = get_next_positions(input.clone(), i);
            }
        }

        draw_error_message(&format!(
            "Iteration: {}; p1: {}",
            i,
            get_score(robots.iter().map(|robot| robot.0).collect())
        ));
        next_frame().await
    }
}

async fn draw_robots(robots: &Input) {
    for ((column, line), _) in robots {
        draw_rectangle(
            *column as f32 * CELL_WIDTH,
            *line as f32 * CELL_WIDTH,
            CELL_WIDTH,
            CELL_WIDTH,
            WHITE,
        );
    }
}

fn get_next_positions(robots: Input, iteration: isize) -> Input {
    robots
        .into_iter()
        .map(|(initial_position, mut movement)| {
            movement = (iteration * movement.0, iteration * movement.1);
            (
                (
                    (initial_position.0 + movement.0).rem_euclid(MAP_SIZE.0 as isize),
                    (initial_position.1 + movement.1).rem_euclid(MAP_SIZE.1 as isize),
                ),
                movement,
            )
        })
        .collect()
}

type Input = Vec<((isize, isize), (isize, isize))>;

fn parse_input(input: &str) -> Input {
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            let captures = regex.captures_iter(line).next().unwrap();

            let position = (
                captures.get(1).unwrap().as_str().parse().unwrap(),
                captures.get(2).unwrap().as_str().parse().unwrap(),
            );

            let movement = (
                captures.get(3).unwrap().as_str().parse().unwrap(),
                captures.get(4).unwrap().as_str().parse().unwrap(),
            );

            (position, movement)
        })
        .collect()
}

// for p1
fn get_score(robot_positions: Vec<(isize, isize)>) -> usize {
    let mut quadrants = (0, 0, 0, 0);
    for (x, y) in robot_positions.into_iter() {
        if x == MAP_SIZE.0 as isize / 2 || y == MAP_SIZE.1 as isize / 2 {
            continue;
        }

        if x < MAP_SIZE.0 as isize / 2 {
            if y < MAP_SIZE.1 as isize / 2 {
                // top-left
                quadrants.0 += 1;
            } else {
                // top-right
                quadrants.1 += 1;
            }
        } else {
            if y < MAP_SIZE.1 as isize / 2 {
                // bottom-left
                quadrants.2 += 1;
            } else {
                // bottom-right
                quadrants.3 += 1;
            }
        }
    }

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

pub fn draw_error_message(message: &str) {
    let text_size = measure_text(message, None, 30, 1.0);
    draw_text(
        message,
        screen_width() / 2. - text_size.width / 2.,
        screen_height() / 2. - text_size.height / 2.,
        30.0,
        RED,
    );
}
