use crate::util;
use elf::Elf;
use crate::day_01::elf;

pub fn main() {
    part_01();
    part_02();
}

fn part_01() {
    let file = util::utils::read_file_to_string("src/day_01/data.txt");
    let ps: Vec<&str> = file.split("\n\r\n").map(|r| r.trim()).collect();
    let elfs: Vec<Elf> = ps.iter().map(|el| el.trim().parse().unwrap()).collect();
    println!("{}", elfs.iter().map(|el| el.calories.iter().sum::<i32>()).max().unwrap());
}

fn part_02() {
    let file = util::utils::read_file_to_string("src/day_01/data.txt");
    let ps: Vec<&str> = file.split("\n\r\n").map(|r| r.trim()).collect();
    let elfs: Vec<Elf> = ps.iter().map(|el| el.trim().parse().unwrap()).collect();
    let mut calories: Vec<i32> = elfs.iter().map(|r| r.calories.iter().sum::<i32>()).collect();
    calories.sort();
    println!("{}", calories.iter().rev().take(3).sum::<i32>())
}
