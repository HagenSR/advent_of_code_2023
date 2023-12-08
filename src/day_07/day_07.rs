use crate::day_07::hand_p1::HandP1;
use crate::day_07::hand_p2::HandP2;
use crate::util;

pub fn main() {
    part_01();
    part_02();
}

fn part_01() {
    let mut hands = util::utils::read_file_to_lines::<HandP1>("src/day_07/data.txt", "\n");
    hands.sort();
    hands.sort_by(|e1, e2| e1.hand_type.partial_cmp(&e2.hand_type).unwrap());
    hands.reverse();
    let sum: i32 = hands
        .iter()
        .zip(1i32..hands.len() as i32 + 1)
        .map(|e1| e1.0.bid * e1.1)
        .sum();
    println!("{:?}", sum);
}

fn part_02() {
    let mut hands = util::utils::read_file_to_lines::<HandP2>("src/day_07/data.txt", "\n");
    hands.sort();
    hands.sort_by(|e1, e2| e1.hand_type.partial_cmp(&e2.hand_type).unwrap());
    hands.reverse();
    let sum: i32 = hands
        .iter()
        .zip(1i32..hands.len() as i32 + 1)
        .map(|e1| e1.0.bid * e1.1)
        .sum();
    println!("{:?}", sum);
}
