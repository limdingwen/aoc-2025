use std::fs;

fn is_invalid_1(number: String) -> bool {
    if number.chars().count() % 2 != 0 {
        return false;
    }
    let mid = number.chars().count() / 2;
    let (left, right) = number.split_at(mid);
    return left == right;
}

fn is_invalid_2(number: String) -> bool {
    'outer: for repeat_times in 2..=number.chars().count() {
        if number.len() % repeat_times != 0 {
            continue;
        }
        let chars = number.chars().collect::<Vec<_>>();
        let chunk_size = number.chars().count() / repeat_times;
        let chunks = chars.chunks(chunk_size).collect::<Vec<_>>();
        let first_chunk = chunks.get(0).unwrap();
        for chunk in chunks.iter().skip(1) {
            if chunk != first_chunk {
                continue 'outer;
            }
        }
        return true;
    }
    return false;
}

fn main() {
    // Read input
    let contents = fs::read_to_string("input.txt").unwrap();

    // Part 1
    {
        let mut accumulator: i64 = 0;
        for range in contents.split(",") {
            let range = range.split("-").collect::<Vec<&str>>();
            let from = range.get(0).unwrap();
            let to = range.get(1).unwrap();
            let from = from.parse::<i64>().unwrap();
            let to = to.parse::<i64>().unwrap();
            for number in from..=to {
                if is_invalid_1(number.to_string()) {
                    accumulator += number;
                }
            }
        }
        println!("{}", accumulator);
    }

    // Part 2
    {
        let mut accumulator: i64 = 0;
        for range in contents.split(",") {
            let range = range.split("-").collect::<Vec<&str>>();
            let from = range.get(0).unwrap();
            let to = range.get(1).unwrap();
            let from = from.parse::<i64>().unwrap();
            let to = to.parse::<i64>().unwrap();
            for number in from..=to {
                if is_invalid_2(number.to_string()) {
                    accumulator += number;
                }
            }
        }
        println!("{}", accumulator);
    }
}
