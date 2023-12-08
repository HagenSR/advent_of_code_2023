use crate::day_05::stupid_map::StupidMap;
use crate::util;
use rand::Rng;

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
    let maps = read_maps(lines);
    let mut rng = rand::thread_rng();
    let mut lowest = (0i128, i128::MAX);
    for (start, range) in &grouped_seeds {
        let mut cur_count = 0;
        while cur_count < 100000000 {
            let rand: i128 = rng.gen_range(-1..*range);
            let res = determine_end_value(start + rand, &maps);
            if res < lowest.1 {
                lowest = (start + rand, res);
                cur_count = -1
            }
            cur_count += 1;
        }
    }
    let mut frontier: Vec<i128> = Vec::new();
    frontier.push(lowest.0);
    while let Some(elem) = frontier.pop() {
        let p1 = elem + 1;
        if valid(p1, &grouped_seeds) {
            let up_1 = determine_end_value(elem + 1, &maps);
            if up_1 < lowest.1 {
                lowest = (elem + 1, up_1);
                frontier.push(elem + 1);
            }
        }
        let d1 = elem - 1;
        if valid(d1, &grouped_seeds) {
            let down_1 = determine_end_value(elem - 1, &maps);
            if down_1 < lowest.1 {
                lowest = (elem - 1, down_1);
                frontier.push(elem - 1);
            }
        }
    }
    //            178159714
    // (55039969, 100170604)
    // (55035774, 100166409)
    // (55035073, 100165708)
    //  (55034884, 100165519)
    // (2295838616, 100165129)
    // (2295838616, 100165129)
    // (2295838616, 100165129)
    // (2295838616, 100165129)

    // 100165370
    println!("{:?}", lowest)
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
        let cur = determine_end_value(seed, &maps);
        lowest_location = lowest_location.min(cur);
    }
    println!("{}", lowest_location)
}

fn valid(val: i128, seeds: &Vec<(i128, i128)>) -> bool {
    for (start, range) in seeds {
        if val > *start && val - start < *range {
            return true;
        }
    }
    return false;
}

fn determine_end_value(seed: i128, maps: &Vec<StupidMap>) -> i128 {
    let mut cur = seed;
    for bmap in maps {
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
    return cur;
}
