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
            disk.push((if blank { None } else { Some(id) }, 1));
        }

        if !blank {
            id += 1;
        }
        blank = !blank;
    }

    compute_checksum(reorganize_disk(disk))
}

fn part2(input: Input) -> u64 {
    let mut disk = vec![];

    let mut id = 0;
    let mut blank = false;
    for n in input.iter() {
        if *n > 0 {
            disk.push((if blank { None } else { Some(id) }, *n));
        }

        if !blank {
            id += 1;
        }
        blank = !blank;
    }

    compute_checksum(reorganize_disk(disk))
}

fn reorganize_disk(mut disk: Vec<(Option<u64>, u64)>) -> Vec<(Option<u64>, u64)> {
    for i in 0..disk.len() {
        let current_position = disk.len() - 1 - i;

        if disk[current_position].0.is_none() {
            continue;
        }

        let current_partition_len = disk[current_position].1;

        let swap_position = disk
            .iter()
            .position(|partition| partition.1 >= current_partition_len && partition.0.is_none());

        if swap_position.is_none() || swap_position.unwrap() >= current_position {
            continue;
        }

        let swap_position = swap_position.unwrap();

        let swap_partition_len = disk[swap_position].1;

        disk.get_mut(swap_position).unwrap().1 = current_partition_len;

        disk.swap(current_position, swap_position);

        let empty_partition_len = swap_partition_len - current_partition_len;
        if empty_partition_len > 0 {
            disk.insert(swap_position + 1, (None, empty_partition_len));
        }
    }

    disk
}

fn compute_checksum(disk: Vec<(Option<u64>, u64)>) -> u64 {
    let mut checksum = 0;

    let mut i = 0;
    for (partition_id, file_size) in disk.into_iter() {
        for _ in 0..file_size {
            checksum += partition_id.unwrap_or(0) * i;
            i += 1;
        }
    }

    checksum
}
