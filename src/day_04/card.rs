use std::fmt::Debug;
use std::str::{FromStr, Split};

#[derive(Clone)]
pub struct Card {
    pub id: i32,
    pub number_correct: i32,
}

impl Card {
    pub fn determine_points(&self) -> i128 {
        if self.number_correct == 0 {
            return 0;
        }
        let mut sum = 1;
        for _ in 1..self.number_correct {
            sum *= 2
        }
        return sum;
    }
}

pub fn determine_matching_count(numbers: Vec<i32>, winning: Vec<i32>) -> usize {
    return numbers
        .iter()
        .map(|num| if winning.contains(num) { 1 } else { 0 })
        .sum();
}

impl FromStr for Card {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(":");
        let id = parts
            .next()
            .unwrap()
            .replace("Card ", "")
            .trim()
            .parse::<i32>()
            .expect("");
        let mut winning_and_correct = parts.next().unwrap().split("|");
        let numbers: Vec<i32> = parse_list(&mut winning_and_correct);
        let winning: Vec<i32> = parse_list(&mut winning_and_correct);
        let matching = determine_matching_count(numbers, winning);
        return Ok(Card {
            id,
            number_correct: matching as i32,
        });
    }
}

fn parse_list(elem: &mut Split<&str>) -> Vec<i32> {
    return elem
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .map(|el| el.trim().parse::<i32>())
        .filter(|wrap| wrap.is_ok())
        .map(|ok| ok.unwrap())
        .collect();
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Card")
            .field("id", &self.id)
            .field("numbers", &self.number_correct)
            .finish()
    }
}
