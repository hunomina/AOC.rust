fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let chars = input.chars().collect::<Vec<_>>();

    let mut result = vec![];
    let (mut start, mut end) = (0, 0);
    let mut i = 0;
    while i < chars.len() {
        end += 1;
        let c = &chars[i];
        if !result.contains(&c) {
            result.push(c);
            // part 1: if result.len() == 4 {
            if result.len() == 14 {
                break;
            }
            i += 1;
        } else {
            start += 1;
            end = start;
            i = start;
            result = vec![];
        }
    }
    println!("{}", end);
}
