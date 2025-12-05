use std::fs;

fn main() {
    // Read input
    let contents = fs::read_to_string("input.txt").unwrap();

    // Part 1
    {
        let mut dial = 50;
        let mut at_zero = 0;
        for line in contents.lines() {
            let direction = match line.chars().nth(0).unwrap() {
                'L' => -1,
                'R' => 1,
                _ => panic!("Invalid direction"),
            };
            let distance: i32 = line[1..].parse().unwrap();
            dial += direction * distance;
            dial = dial.rem_euclid(100);
            if dial == 0 {
                at_zero += 1;
            }
        }
        println!("{}", at_zero);
    }

    // Part 2
    {
        let mut dial: i32 = 50;
        let mut at_zero = 0;
        for line in contents.lines() {
            let direction = match line.chars().nth(0).unwrap() {
                'L' => -1,
                'R' => 1,
                _ => panic!("Invalid direction"),
            };
            let distance: i32 = line[1..].parse().unwrap();
            for _ in 0..distance {
                dial += direction;
                dial = dial.rem_euclid(100);
                if dial == 0 {
                    at_zero += 1;
                }
            }
        }
        println!("{}", at_zero);
    }
}
