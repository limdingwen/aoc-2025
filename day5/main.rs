use std::{collections::HashSet, fs};

fn main() {
    // Read input
    let contents = fs::read_to_string("input-example.txt").unwrap();
    let parts = contents.split("\n\n").collect::<Vec<_>>();
    let ranges_str = parts.get(0).unwrap();
    let available_str = parts.get(1).unwrap();
    let ranges = ranges_str
        .lines()
        .map(|line| {
            let parts = line.split("-").collect::<Vec<_>>();
            let start = parts.get(0).unwrap().parse::<u64>().unwrap();
            let end = parts.get(1).unwrap().parse::<u64>().unwrap();
            (start, end)
        })
        .collect::<Vec<_>>();
    let available = available_str
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // Part 1
    {
        let total = available
            .iter()
            .filter(|value| {
                let is_fresh = ranges
                    .iter()
                    .any(|(start, end)| **value >= *start && **value <= *end);
                is_fresh
            })
            .count();
        println!("{}", total);
    }

    // Part 2
    {
        let exclusive_ranges = ranges.iter().fold(Vec::new(), |accumulator, (start, end)| {
            let new_exclusive_ranges = accumulator.iter().fold(
                Vec::new(),
                |accumulator, (existing_start, existing_end)| {
                    if start < existing_start && end > existing_end {
                        return accumulator;
                    }
                    if end < existing_start {}
                },
            );
            [new_exclusive_ranges, vec![(*start, *end)]].concat()
        });
    }
}
