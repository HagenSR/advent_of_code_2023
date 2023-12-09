use crate::day_08::node::Node;
use crate::util;
use std::collections::BTreeMap;

pub fn main() {
    // part_01();
    part_02();
}

fn part_01() {
    let file = util::utils::read_file_to_string("src/day_08/data.txt");
    let mut parts = file.split("\n");
    let instructions: Vec<char> = parts.next().unwrap().trim().chars().collect();
    let mut map: BTreeMap<String, Node> = BTreeMap::new();
    parts.next();
    while let Some(line) = parts.next() {
        let node: Node = line.parse().unwrap();
        map.insert(node.name.to_string(), node.clone());
    }
    let mut steps = 0;
    let mut cur_node = "AAA".to_string();
    while cur_node != "ZZZ" {
        let next_step = instructions.get(steps % instructions.len()).unwrap();
        let node = map.get(&*cur_node).unwrap();
        if next_step == &'L' {
            cur_node = node.left_right.0.clone();
        } else {
            cur_node = node.left_right.1.clone();
        }
        steps += 1;
    }
    println!("{}", steps)
}

fn part_02() {
    let file = util::utils::read_file_to_string("src/day_08/data.txt");
    let mut parts = file.split("\n");
    let instructions: Vec<char> = parts.next().unwrap().trim().chars().collect();
    let mut map: BTreeMap<String, Node> = BTreeMap::new();
    let mut starts: Vec<String> = Vec::new();
    parts.next();
    while let Some(line) = parts.next() {
        let node: Node = line.parse().unwrap();
        map.insert(node.name.to_string(), node.clone());
        if node.name.ends_with('A') {
            starts.push(node.name.to_string());
        }
    }
    let mut steps: i128 = 0;
    let mut ends_with_z_count = 0;
    let expected_count = starts.len();
    while ends_with_z_count != expected_count {
        ends_with_z_count = 0;
        let mut next_keys: Vec<String> = Vec::new();
        for key in &starts {
            let next_step = instructions
                .get((steps % instructions.len() as i128) as usize)
                .unwrap();
            let node = map.get(&*key).unwrap();
            let next;
            if next_step == &'L' {
                next = node.left_right.0.clone();
            } else {
                next = node.left_right.1.clone();
            }
            if next.ends_with('Z') {
                ends_with_z_count += 1;
            }
            next_keys.push(next);
        }
        starts = next_keys;
        steps += 1;
    }
    println!("{}", steps)
}
