use crate::day_02::game::Game;
use crate::util;
use std::collections::BTreeMap;

pub fn main() {
    part_01();
    part_02();
}

fn part_01() {
    let games = util::utils::read_file_to_lines::<Game>("src/day_02/data.txt", "\n");
    let max_values = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut good_ids: Vec<i32> = Vec::new();
    for game in games {
        let mut good = true;
        for turn in game.turns {
            for color in turn {
                let max = max_values.get(&*color.0).unwrap();
                if *max < color.1 {
                    good = false;
                }
            }
        }
        if good {
            good_ids.push(game.id);
        }
    }
    println!("{}", good_ids.iter().sum::<i32>())
}

fn part_02() {
    let games = util::utils::read_file_to_lines::<Game>("src/day_02/data.txt", "\n");
    let mut powers: Vec<i32> = Vec::new();
    for game in games {
        let mut max_values = BTreeMap::new();
        for turn in game.turns {
            for color in turn {
                let max = max_values.get(&*color.0).unwrap_or(&0);
                if *max < color.1 {
                    max_values.insert(color.0, color.1);
                }
            }
        }
        let prod: i32 = max_values
            .values()
            .copied()
            .reduce(|acc, e| acc * e)
            .unwrap();
        powers.push(prod);
    }
    println!("{}", powers.iter().sum::<i32>())
}
