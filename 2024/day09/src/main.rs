use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let input = parse_input(input.as_str());

    println!("part1 {:?}", part1(input.clone()));
    println!("part2 {:?}", part2(input.clone()));
}

type Input = Vec<u64>;

fn parse_input(input: &str) -> Input {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect()
}

fn part1(input: Input) -> u64 {
    let mut disk = vec![];

    let mut id = 0;
    let mut blank = false;
    for n in input.iter() {
        for _ in 0..*n {
            if blank {
                disk.push(None);
            } else {
                disk.push(Some(id));
            }
        }
        if !blank {
            id += 1;
        }
        blank = !blank;
    }

    let disk_len = disk.len();
    for i in 0..disk_len {
        let item = disk[i];
        if item.is_some() {
            continue;
        }

        if !disk[i..].iter().any(|v| v.is_some()) {
            break;
        }

        let swap_position = disk.iter().rev().position(|ll| ll.is_some()).unwrap();
        disk.swap(i, disk_len - 1 - swap_position);
    }

    compute_checksum(disk.into_iter())
}

fn part2(input: Input) -> u64 {
    let mut disk = vec![];

    let mut id = 0;
    let mut blank = false;
    for n in input.iter() {
        let v = if blank { None } else { Some(id) };

        if *n > 0 {
            disk.push(vec![v; *n as usize]);
        }

        if !blank {
            id += 1;
        }
        blank = !blank;
    }

    for i in 0..disk.len() {
        let current_position = disk.len() - 1 - i;

        if disk[current_position][0].is_none() {
            continue;
        }

        let current_partition_len = disk[current_position].len();

        let swap_position = disk
            .iter()
            .position(|ll| ll.len() >= disk[current_position].len() && ll[0].is_none());

        if swap_position.is_none() || swap_position.unwrap() >= current_position {
            continue;
        }

        let swap_position = swap_position.unwrap();

        let swapped_partition_len = disk[swap_position].len();
        // truncate swap partition
        disk.get_mut(swap_position)
            .unwrap()
            .truncate(current_partition_len);

        disk.swap(current_position, swap_position);

        let empty_partition_len = swapped_partition_len - current_partition_len;
        if empty_partition_len > 0 {
            disk.insert(swap_position + 1, vec![None; empty_partition_len]);
        }
    }

    compute_checksum(disk.into_iter().flatten())
}

fn compute_checksum<I: Iterator<Item = Option<u64>>>(iter: I) -> u64 {
    iter.enumerate()
        .fold(0, |acc, (i, v)| acc + i as u64 * v.unwrap_or(0))
}
