use std::fs;

use ndarray::Array2;

fn main() {
    // Read input
    let contents = fs::read_to_string("input-example.txt").unwrap();
    let parts = contents
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let parts = Array2::from_shape_vec(
        (parts.len(), parts.get(0).unwrap().len()),
        parts.iter().flatten().collect(),
    )
    .unwrap();
    let parts = parts.t();

    // Part 1
    {
        let total = parts
            .rows()
            .into_iter()
            .map(|problem| {
                let operator = problem.last().unwrap();
                let init = match **operator {
                    "+" => 0,
                    "*" => 1,
                    _ => panic!("wrong operator"),
                };
                problem
                    .into_iter()
                    .take(problem.len() - 1)
                    .fold(init, |accumulator, number| match **operator {
                        "+" => accumulator + number.parse::<u64>().unwrap(),
                        "*" => accumulator * number.parse::<u64>().unwrap(),
                        _ => panic!("wrong operator"),
                    })
            })
            .sum::<u64>();
        println!("{}", total);
    }
}
