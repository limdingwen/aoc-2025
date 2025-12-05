use std::fs;

fn find_max_combination(chars: &[char], digits: u8, number: u64, number_max: u64) -> u64 {
    if digits == 0 {
        return number;
    }
    if (chars.len() as u8) < digits {
        return 0;
    }

    // Stop if it's not possible to get number max
    let test_number = (0..digits).fold(number, |acc, _| acc * 10 + 9);
    if test_number <= number_max {
        return number_max;
    }

    // Use biggest digits first
    let mut digit_indices: Vec<(usize, u64)> = chars
        .iter()
        .enumerate()
        .map(|(i, c)| (i, c.to_digit(10).unwrap() as u64))
        .collect();
    digit_indices.sort_by(|a, b| b.1.cmp(&a.1));
    let digit_indices = digit_indices;

    // Try each digit as the next digit, then recurse for the remaining digits
    let mut number_max = number_max;
    for (i, digit) in digit_indices {
        let new_number = number * 10 + digit;
        let remaining_chars = &chars[(i + 1)..];
        let candidate = find_max_combination(remaining_chars, digits - 1, new_number, number_max);
        if candidate > number_max {
            number_max = candidate;
        }
    }
    return number_max;
}

fn main() {
    // Read input
    let contents = fs::read_to_string("input.txt").unwrap();

    // Part 1
    {
        let total = contents
            .lines()
            .map(|line| {
                let chars = line.chars().collect::<Vec<_>>();
                let indices = 0..chars.len();
                let indices = indices.flat_map(|i| ((i + 1)..chars.len()).map(move |j| (i, j)));
                let number_max = indices
                    .map(|(i, j)| {
                        let a = chars.get(i).unwrap();
                        let b = chars.get(j).unwrap();
                        let chars = vec![*a, *b];
                        let number = String::from_iter(chars);
                        let number = number.parse::<u64>().unwrap();
                        number
                    })
                    .max()
                    .unwrap();
                number_max
            })
            .sum::<u64>();
        println!("{}", total);
    }

    // Part 2
    {
        let total = contents
            .lines()
            .map(|line| {
                let chars = line.chars().collect::<Vec<_>>();
                let number_max = find_max_combination(&chars, 12, 0, 0);
                number_max
            })
            .sum::<u64>();
        println!("{}", total);
    }
}
