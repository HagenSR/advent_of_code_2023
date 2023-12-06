use crate::util;

pub fn main() {
    part_01();
    part_02();
}

fn part_01() {
    let lines = util::utils::read_file_to_string("src/day_03/data.txt");
    let grid = util::utils::parse_to_grid::<String>(&lines);
    let nums = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut sums = Vec::new();
    let mut x = 0;
    let mut run: Vec<String> = Vec::new();
    for line in grid.iter() {
        let mut y = 0;
        let mut good = false;
        for char in line.iter() {
            if nums.contains(&&**char) {
                run.push(char.to_string());
            } else {
                if run.len() > 0 && good {
                    sums.push(run.join("").parse::<i32>().unwrap());
                }
                good = false;
                run.clear();
            }
            if !good && run.len() > 0 {
                good = determine_touching(x, y, &grid, nums, false);
            }
            y += 1;
        }
        if run.len() > 0 && good {
            sums.push(run.join("").parse::<i32>().unwrap());
        }
        run.clear();
        x += 1;
    }
    println!("{:?}", sums.iter().sum::<i32>())
}

fn determine_touching(
    x: i32,
    y: i32,
    grid: &Vec<Vec<String>>,
    nums: [&str; 10],
    gear_only: bool,
) -> bool {
    for i in -1..2i32 {
        for j in -1..2i32 {
            if 0 <= x + i
                && x + i < grid.len() as i32
                && 0 <= y + j
                && y + j < grid[(x + i) as usize].len() as i32
            {
                if gear_only {
                    if grid[(x + i) as usize][(y + j) as usize] == "*" {
                        return true;
                    }
                } else {
                    if grid[(x + i) as usize][(y + j) as usize] != "."
                        && !nums.contains(&&*grid[(x + i) as usize][(y + j) as usize])
                    {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

fn part_02() {
    let lines = util::utils::read_file_to_string("src/day_03/data.txt");
    let grid = util::utils::parse_to_grid::<String>(&lines);
    let nums = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut gears: Vec<(i32, i32)> = Vec::new();
    let mut grid_nums = Vec::new();
    let mut x = 0;
    let mut run: Vec<String> = Vec::new();
    let mut left_most = (0, 0);
    for line in grid.iter() {
        let mut y = 0;
        let mut good = false;
        for char in line.iter() {
            if nums.contains(&&**char) {
                if run.is_empty() {
                    left_most = (x, y);
                }
                run.push(char.to_string());
            } else {
                if run.len() > 0 && good {
                    grid_nums.push((left_most, run.join("").parse::<i32>().unwrap()));
                }
                good = false;
                run.clear();
            }
            if char == "*" {
                gears.push((x, y));
            }
            if !good && run.len() > 0 {
                good = determine_touching(x, y, &grid, nums, true);
            }
            y += 1;
        }
        if run.len() > 0 && good {
            grid_nums.push(((left_most), run.join("").parse::<i32>().unwrap()));
        }
        run.clear();
        x += 1;
    }
    let mut sum = 0;
    for gear in gears {
        let mut touches = Vec::new();
        for (cord, num) in &grid_nums {
            for i in 0..num.to_string().len() {
                if determine_touching_gear(cord.0, cord.1 + i as i32, gear.0, gear.1, &grid) {
                    touches.push(num);
                    break;
                }
            }
        }
        if touches.len() == 2 {
            sum += touches[0] * touches[1];
        }
    }
    println!("{:?}", sum)
}

fn determine_touching_gear(x: i32, y: i32, x2: i32, y2: i32, grid: &Vec<Vec<String>>) -> bool {
    for i in -1..2i32 {
        for j in -1..2i32 {
            if 0 <= x + i
                && x + i < grid.len() as i32
                && 0 <= y + j
                && y + j < grid[(x + i) as usize].len() as i32
            {
                if x + i == x2 && y + j == y2 {
                    return true;
                }
            }
        }
    }
    return false;
}
