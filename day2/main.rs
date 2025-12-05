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

fn invalid_total_in_range(range: &str, is_invalid: fn(String) -> bool) -> u64 {
    let range = range.split("-").collect::<Vec<&str>>();
    let from = range.get(0).unwrap();
    let to = range.get(1).unwrap();
    let from = from.parse::<u64>().unwrap();
    let to = to.parse::<u64>().unwrap();
    let total = (from..=to).map(|number| {
        match is_invalid(number.to_string()) {
            true => number,
            false => 0,
        }
    }).sum::<u64>();
    total
}

fn main() {
    // Read input
    let contents = fs::read_to_string("input.txt").unwrap();

    // Part 1
    {
        let total = contents.split(",").map(|range| invalid_total_in_range(range, is_invalid_1)).sum::<u64>();
        println!("{}", total);
    }

    // Part 2
    {
        let total = contents.split(",").map(|range| invalid_total_in_range(range, is_invalid_2)).sum::<u64>();
        println!("{}", total);
    }
}
