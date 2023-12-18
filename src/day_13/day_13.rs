use crate::day_13::rubble::Rubble;
use crate::util;

pub fn main() {
    part_1();
    part_2();
}

fn part_1() {
    println!("{}", run_reflection(0));
}

fn part_2() {
    println!("{}", run_reflection(1));
}

fn run_reflection(expected_diff: usize) -> usize {
    let fl = util::utils::read_file_to_lines::<String>("src/day_13/data.txt", "\n\r\n");
    let grids: Vec<Rubble> = fl.iter().map(|elem| elem.parse().unwrap()).collect();
    let mut sum = 0;
    for grid in grids {
        let reflection = grid.find_reflection(expected_diff);
        if reflection.0 != 0 {
            sum += reflection.0 * 100
        } else {
            sum += reflection.1
        }
    }
    return sum;
}
