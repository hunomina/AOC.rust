use regex::Regex;
use std::fs;

type TargetArea = (Position, Position);
type Position = (i32, i32);
type Velocity = (i32, i32);

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let target_area = parse_input(&input);

    println!("part1 result {}", part1(target_area.clone()));
    println!("part2 result {}", part2(target_area.clone()));
}

fn parse_input(input: &str) -> TargetArea {
    let r = Regex::new(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$").unwrap();
    let captures = r.captures_iter(input).nth(0).unwrap();
    (
        (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
        (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
    )
}

fn part1(target_area: TargetArea) -> i32 {
    get_max_height_for_velocity_vector(find_best_velocity(target_area))
}

fn part2(target_area: TargetArea) -> usize {
    find_all_possible_velocity_horizontal_values(&target_area)
        .into_iter()
        .fold(0, |acc, horizontal_value| {
            acc + find_all_possible_velocity_vertical_values_for_horizontal_value(
                horizontal_value,
                &target_area,
            )
            .len()
        })
}

fn find_all_possible_velocity_horizontal_values(target_area: &TargetArea) -> Vec<i32> {
    (1..=target_area.0 .1)
        .rev()
        .into_iter()
        .filter_map(|possible_horizontal_value| {
            let mut possible_value_to_one = (1..=possible_horizontal_value).rev();
            let mut total = 0;
            while let Some(next_possible_value) = possible_value_to_one.next() {
                total += next_possible_value;
                if total >= target_area.0 .0 {
                    return if total <= target_area.0 .1 {
                        Some(possible_horizontal_value)
                    } else {
                        None
                    };
                }
            }
            None
        })
        .collect()
}

fn find_all_possible_velocity_vertical_values_for_horizontal_value(
    horizontal_value: i32,
    target_area: &TargetArea,
) -> Vec<i32> {
    (-target_area.1 .0.abs()..=target_area.1 .0.abs())
        .filter_map(|y| is_valid_velocity_vector((horizontal_value, y), target_area).then(|| y))
        .collect()
}

fn find_max_velocity_vertical_value_for_horizontal_value(
    horizontal_value: i32,
    target_area: &TargetArea,
) -> i32 {
    find_all_possible_velocity_vertical_values_for_horizontal_value(horizontal_value, target_area)
        .into_iter()
        .max()
        .unwrap()
}

fn find_best_velocity(target_area: TargetArea) -> Velocity {
    find_all_possible_velocity_horizontal_values(&target_area)
        .into_iter()
        .map(|horizontal_value| {
            (
                horizontal_value.clone(),
                find_max_velocity_vertical_value_for_horizontal_value(
                    horizontal_value,
                    &target_area,
                ),
            )
        })
        .max_by(|(_, y1), (_, y2)| y1.cmp(y2))
        .unwrap()
}

fn compute_next_step(
    mut position: Position,
    mut velocity_vector: Position,
) -> (Position, Velocity) {
    position.0 += velocity_vector.0;
    position.1 += velocity_vector.1;
    velocity_vector.0 += if velocity_vector.0 > 0 {
        -1
    } else if velocity_vector.0 < 0 {
        1
    } else {
        0
    };
    velocity_vector.1 -= 1;

    (position, velocity_vector)
}

fn is_valid_velocity_vector(mut velocity_vector: Velocity, target_area: &TargetArea) -> bool {
    let mut from = (0, 0);

    loop {
        let next_step = compute_next_step(from, velocity_vector);

        from = next_step.0;
        velocity_vector = next_step.1;

        if from.0 > target_area.0 .1 {
            return false;
        }

        if from.1 < target_area.1 .0 {
            return false;
        }

        if from.0 >= target_area.0 .0 && from.1 <= target_area.1 .1 {
            return true;
        }
    }
}

fn get_max_height_for_velocity_vector(mut velocity_vector: Velocity) -> i32 {
    let mut from = (0, 0);

    loop {
        let next_step = compute_next_step(from, velocity_vector);

        if next_step.0 .1 < from.1 {
            return from.1;
        }

        from = next_step.0;
        velocity_vector = next_step.1;
    }
}
