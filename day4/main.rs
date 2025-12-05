use std::{fs, iter::zip};

use grid::Grid;

#[derive(Clone)]
enum Cell {
    Empty,
    Filled,
}

fn is_visible(map: &Grid<Cell>, row: usize, col: usize) -> bool {
    let directions = [
        (-1, 0),  // Up
        (1, 0),   // Down
        (0, -1),  // Left
        (0, 1),   // Right
        (-1, -1), // Up-Left
        (-1, 1),  // Up-Right
        (1, -1),  // Down-Left
        (1, 1),   // Down-Right
    ];
    let filled_nearby_count = directions
        .iter()
        .filter(|(dr, dc)| {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            let nearby_cell = map.get(new_row as usize, new_col as usize);
            match nearby_cell {
                Some(Cell::Filled) => true,
                _ => false,
            }
        })
        .count();
    filled_nearby_count < 4
}

fn main() {
    // Read input
    let contents = fs::read_to_string("input.txt").unwrap();
    let column_count = contents.lines().next().unwrap().len();
    let map = Grid::from_vec(
        contents
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '@' => Cell::Filled,
                    '.' => Cell::Empty,
                    _ => panic!("Unexpected character"),
                })
            })
            .collect::<Vec<_>>(),
        column_count,
    );

    // Part 1
    {
        let total = map
            .indexed_iter()
            .filter(|((row, col), cell)| match cell {
                Cell::Filled => is_visible(&map, *row, *col),
                Cell::Empty => false,
            })
            .count();
        println!("{}", total);
    }

    // Part 1
    {
        let mut map = map.clone();
        let mut total = 0;
        loop {
            let accessible_positions = map
                .indexed_iter()
                .map(|((row, col), cell)| match cell {
                    Cell::Filled => is_visible(&map, row, col),
                    Cell::Empty => false,
                })
                .collect::<Vec<_>>();
            let accessible_positions_count = accessible_positions
                .iter()
                .filter(|is_accessible| **is_accessible)
                .count();
            if accessible_positions_count == 0 {
                break;
            }
            total += accessible_positions_count;
            map = Grid::from_vec(
                zip(map, accessible_positions)
                    .map(
                        |(cell, accessible_position)| match (cell, accessible_position) {
                            (Cell::Filled, false) => Cell::Filled,
                            (Cell::Filled, true) => Cell::Empty,
                            (Cell::Empty, _) => Cell::Empty,
                        },
                    )
                    .collect::<Vec<_>>(),
                column_count,
            );
        }
        println!("{}", total);
    }
}
