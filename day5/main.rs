use std::fs;

fn main() {
    // Read input
    let contents = fs::read_to_string("input.txt").unwrap();
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
        let mut ranges = ranges.clone();
        ranges.sort_by(|(s1, _), (s2, _)| s1.cmp(s2));
        let mut exclusive_ranges: Vec<(u64, u64)> = Vec::new();
        for (s2, e2) in ranges {
            if let Some(&(s1, e1)) = exclusive_ranges.last() {
                if e2 <= e1 {
                    continue;
                }
                if s2 <= e1 {
                    exclusive_ranges.pop();
                    exclusive_ranges.push((s1, e2));
                    continue;
                }
            }
            exclusive_ranges.push((s2, e2));
        }
        let total = exclusive_ranges
            .into_iter()
            .map(|(s, e)| e - s + 1)
            .sum::<u64>();
        println!("{}", total);
    }
}
