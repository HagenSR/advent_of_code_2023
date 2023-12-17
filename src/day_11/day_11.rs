use crate::util;
use std::collections::BTreeSet;

pub fn main() {
    part_1();
}

fn part_1() {
    let file = util::utils::read_file_to_string("src/day_11/data.txt");
}

fn mark_double(map: &mut Vec<Vec<char>>) {
    let mut rows: BTreeSet<usize> = (0..map.len()).collect();
    let mut cols: BTreeSet<usize> = (0..map.len()).collect();
    for i in 0..map.len() {
        for j in 0..map.len() {
            if map[i][j] == '#' {
                rows.remove(&i);
                cols.remove(&j);
            }
        }
    }
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '#' {
                if rows.contains(&i) || cols.contains(&j) {
                    map[i][j] = '2';
                } else {
                    map[i][j] = '1';
                }
            }
        }
    }
}
