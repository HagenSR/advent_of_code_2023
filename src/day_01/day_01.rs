use crate::util;
use std::collections::BTreeMap;

pub fn main() {
    part_01();
    part_02();
}

fn part_01() {
    let file = util::utils::read_file_to_string("src/day_01/data.txt");
    let lines: Vec<&str> = file.split("\n").collect();
    let mut sum = 0;
    for line in lines {
        let mut is_first = false;
        let mut first_char = "".to_string();
        let mut last_char = "".to_string();
        for char in line.chars() {
            if char.is_numeric() {
                if !is_first {
                    is_first = !is_first;
                    first_char = char.to_string();
                    last_char = char.to_string();
                } else {
                    last_char = char.to_string()
                }
            }
        }
        let concat = first_char.to_string() + &last_char.to_string();
        let parse: i32 = concat.parse().unwrap();
        sum += parse;
    }
    println!("{}", sum);
}

fn part_02() {
    let file = util::utils::read_file_to_string("src/day_01/data.txt");
    let lines: Vec<&str> = file.split("\n").collect();
    let mut sum = 0;
    let num_dict = BTreeMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    for line in lines {
        let mut is_first = false;
        let mut first_char = "".to_string();
        let mut last_char = "".to_string();
        for (ind, char) in line.chars().enumerate() {
            let num_from_sub = substring_contains_num_starting_from_ind(ind, line, &num_dict);
            if char.is_numeric() || num_from_sub != -1 {
                let mut potential_num = char.to_string();
                if num_from_sub != -1 {
                    potential_num = num_from_sub.to_string();
                }
                if !is_first {
                    is_first = !is_first;
                    first_char = potential_num.to_owned();
                    last_char = potential_num.to_owned();
                } else {
                    last_char = potential_num.to_owned();
                }
            }
        }
        let concat = first_char.to_string() + &last_char.to_string();
        let parsed: i32 = concat.parse().unwrap();
        sum += parsed;
    }
    println!("{}", sum)
}

fn substring_contains_num_starting_from_ind(
    ind: usize,
    line: &str,
    str_to_num: &BTreeMap<&str, i32>,
) -> i32 {
    for (key, real) in str_to_num.iter() {
        let ln = key.len();
        if ind + ln < line.len() && line[ind..ind + ln] == **key {
            return *real;
        }
    }
    return -1;
}
