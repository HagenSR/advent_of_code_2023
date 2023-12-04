use crate::day_04::card::Card;
use crate::util;

pub fn main() {
    part_01();
    part_02();
}

fn part_01() {
    let cards = util::utils::read_file_to_lines::<Card>("src/day_04/data.txt", "\n");
    let points: i128 = cards.iter().map(|card| card.determine_points()).sum();
    println!("{}", points)
}

fn part_02() {
    let cards = util::utils::read_file_to_lines::<Card>("src/day_04/data.txt", "\n");
    let mut frontier: Vec<Card> = cards.clone();
    let mut count: i128 = 0;
    while frontier.len() > 0 {
        let card = frontier.remove(0);
        count += 1;
        for i in 0..card.number_correct {
            frontier.push(
                cards[(card.id + i) as usize].clone()
            )
        }
    }
    println!("{}", count)
}