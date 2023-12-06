use crate::util;

pub fn main() {
    part_01();
    part_02();
}

fn part_01() {
    let file = util::utils::read_file_to_string("src/day_06/data.txt");
    let mut lines = file.split("\n");
    let times: Vec<i32> = read_to_vec(lines.next());
    let distances: Vec<i32> = read_to_vec(lines.next());
    let mut prod = 1;
    for (time, distance) in times.iter().zip(distances) {
        let mut valid_count = 0;
        for speed in 1..*time {
            if speed * (time - speed) > distance {
                valid_count += 1;
            }
        }
        prod *= valid_count;
    }
    println!("{}", prod)
}

fn read_to_vec(st: Option<&str>) -> Vec<i32> {
    return st
        .unwrap()
        .trim()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .map(|elem| elem.trim().parse::<i32>())
        .filter(|elem| elem.is_ok())
        .map(|elem| elem.unwrap())
        .collect();
}

fn part_02() {
    let file = util::utils::read_file_to_string("src/day_06/data.txt");
    let mut lines = file.split("\n");
    let time: i64 = read_to_single_num(lines.next());
    let distance: i64 = read_to_single_num(lines.next());
    let mut valid_count = 0;
    for speed in 1..time {
        if speed * (time - speed) > distance {
            valid_count += 1;
        }
    }
    println!("{}", valid_count)
}

fn read_to_single_num(st: Option<&str>) -> i64 {
    return st
        .unwrap()
        .trim()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();
}
