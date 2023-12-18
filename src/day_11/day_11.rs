use crate::util;
use std::collections::BTreeSet;

pub fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let sum = find_sum(false);
    println!("{}", sum)
}

fn part_2() {
    let sum = find_sum(true);
    println!("{}", sum)
}

fn find_sum(is_part_2: bool) -> u128 {
    let file = util::utils::read_file_to_string("src/day_11/data.txt");
    let mut grid = util::utils::parse_to_grid::<char>(&file);
    mark_double(&mut grid);
    let stars = find_stars(&grid);
    let mut sum: u128 = 0;
    for i in 0..stars.len() {
        for j in i..stars.len() {
            sum += find_distance(&grid, stars[i], stars[j], is_part_2);
        }
    }
    return sum;
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

fn find_stars(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut rtn: Vec<(usize, usize)> = Vec::new();
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '#' {
                rtn.push((x, y))
            }
        }
    }
    return rtn;
}

fn find_distance(
    grid: &Vec<Vec<char>>,
    star1: (usize, usize),
    star2: (usize, usize),
    is_part_two: bool,
) -> u128 {
    let mut dist = 0;
    let mut cur = star1.0;
    while cur != star2.0 {
        if (cur as i32 - star2.0 as i32) < 0 {
            cur += 1
        } else {
            cur -= 1
        };
        if grid[cur][star1.1] == '2' && !is_part_two {
            dist += 2;
        } else if grid[cur][star1.1] == '2' && is_part_two {
            dist += 1000000;
        } else {
            dist += 1
        }
    }
    cur = star1.1;
    while cur != star2.1 {
        if (cur as i32 - star2.1 as i32) < 0 {
            cur += 1
        } else {
            cur -= 1
        };
        if grid[star2.0][cur] == '2' && !is_part_two {
            dist += 2;
        } else if grid[star2.0][cur] == '2' && is_part_two  {
            dist += 1000000;
        } else{
            dist += 1
        }
    }
    return dist;
}
