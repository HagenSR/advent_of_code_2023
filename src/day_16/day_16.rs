use crate::day_16::beam::Beam;
use crate::util;
use std::collections::BTreeSet;

pub fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let file = util::utils::read_file_to_string("src/day_16/data.txt");
    let grid = util::utils::parse_to_grid(&file);
    let seen_squares = run_simulation(
        Beam {
            square: (0, -1),
            direction: (0, 1),
        },
        &grid,
    );
    println!("{}", seen_squares)
}

fn part_2() {
    let file = util::utils::read_file_to_string("src/day_16/data.txt");
    let grid = util::utils::parse_to_grid(&file);
    let mut starting: Vec<Beam> = Vec::new();
    for dir in vec![1, -1] {
        for i in 0..grid.len() {
            starting.push(Beam {
                square: (i as i32, -1),
                direction: (0, dir),
            });
            starting.push(Beam {
                square: (-1, i as i32),
                direction: (dir, 0),
            });
        }
    }
    let mut max = 0;
    for start in starting {
        let ms = run_simulation(start, &grid);
        max = max.max(ms);
    }
    println!("{}", max);
}

fn run_simulation(beam: Beam, grid: &Vec<Vec<char>>) -> usize {
    let mut positions: Vec<Beam> = vec![beam];
    let mut seen_squares: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut seen_beams: BTreeSet<Beam> = BTreeSet::new();
    while let Some(mut position) = positions.pop() {
        let result = position.next_square(&grid);
        for beam in result.0 {
            if !seen_beams.contains(&beam) {
                seen_beams.insert(beam.clone());
                positions.push(beam.clone());
            }
        }
        seen_squares.extend(&result.1);
    }
    return seen_squares.len();
}
