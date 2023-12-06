use crate::day_05::stupid_map::StupidMap;
use crate::util;

pub fn main() {
    part_01();
    part_02();
}

fn part_01() {
    let mut lines = util::utils::read_file_to_lines::<String>("src/day_05/data.txt", "\n\r\n");
    let seeds: Vec<i128> = read_seeds(lines.remove(0));
    let maps = read_maps(lines);
    find_lowest_location(seeds, maps)
}

fn part_02() {
    let mut lines = util::utils::read_file_to_lines::<String>("src/day_05/data.txt", "\n\r\n");
    let seeds: Vec<i128> = read_seeds(lines.remove(0));
    let mut grouped_seeds: Vec<(i128, i128)> = Vec::new();
    for index in (0..seeds.len()).step_by(2) {
        grouped_seeds.push((seeds[index], seeds[index + 1]))
    }
    grouped_seeds.sort_by(|e1, e2| (e1.0.cmp(&e2.0)));
    let mut maps = read_maps(lines);
    let mut pot_seeds: Vec<i128> = Vec::new();
    let start_map = maps.get(0).unwrap();
    let mut cur_seed = grouped_seeds[0].0;
    for (key, val) in start_map.map.iter() {
        if cur_seed < *key && valid(cur_seed, &grouped_seeds) {
            pot_seeds.push(cur_seed);
        }
        let over = has_overlap((*key, val.1), &grouped_seeds);
        if over != -1 {
            pot_seeds.push(over)
        }
        cur_seed = *key + val.1 - 1;
        if valid(cur_seed, &grouped_seeds) {
            pot_seeds.push(cur_seed);
        }
    }
    println!("{:?}", pot_seeds);
    find_lowest_location(pot_seeds, maps);
}

fn valid(val: i128, seeds: &Vec<(i128, i128)>) -> bool {
    for (start, range) in seeds {
        if val > *start && val - start < *range {
            return true;
        }
    }
    return false;
}

fn has_overlap(map: (i128, i128), seeds: &Vec<(i128, i128)>) -> i128 {
    for seed in seeds {
        let mut pot = map.0;
        if map.0 <= seed.0 + seed.1 && seed.0 <= map.0 + map.1 {
            while pot < seed.0 {
                pot += 1;
            }
            return pot;
        }
    }

    return -1;
}

fn read_seeds(line: String) -> Vec<i128> {
    return line
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .map(|elem| elem.parse().unwrap())
        .collect();
}

fn read_maps(lines: Vec<String>) -> Vec<StupidMap> {
    let mut maps: Vec<StupidMap> = Vec::new();
    for line in lines {
        maps.push(line.parse::<StupidMap>().unwrap());
    }
    return maps;
}

fn find_lowest_location(seeds: Vec<i128>, maps: Vec<StupidMap>) {
    let mut lowest_location = i128::MAX;
    for seed in seeds {
        let mut cur = seed;
        for bmap in &maps {
            let mut closest = i128::MIN + cur + 1;
            for key in bmap.map.keys() {
                if (cur - *key).abs() < (cur - closest).abs() && *key < cur {
                    closest = *key;
                }
            }
            if bmap.map.contains_key(&closest) {
                let val = bmap.map.get(&closest).unwrap();
                let dif = cur - closest;
                if dif < val.1 {
                    cur = val.0 + dif
                }
            }
        }
        lowest_location = lowest_location.min(cur);
    }
    println!("{}", lowest_location)
}
