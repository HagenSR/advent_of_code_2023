use crate::day_07::hand_type_enum::HandType;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::str::{FromStr, Split};

#[derive(Clone)]
pub struct HandP1 {
    pub hand: Vec<char>,
    pub hand_type: HandType,
    pub bid: i32,
}

impl FromStr for HandP1 {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(" ");
        let mut hand: Vec<char> = parse_list(&mut parts);
        let bid: i32 = parts.next().unwrap().trim().parse().unwrap();
        let hand_type = determine_hand_type(&hand);
        return Ok(HandP1 {
            hand,
            bid,
            hand_type,
        });
    }
}

pub fn determine_hand_type(hand: &Vec<char>) -> HandType {
    let num_of_kind = get_num_of_kind(&hand);
    let mut max_of_kind = get_max_of_kind(&num_of_kind);
    if max_of_kind >= 5 {
        return HandType::FiveOfAKind;
    } else if max_of_kind == 4 {
        return HandType::FourOfAKind;
    } else if max_of_kind == 3 && num_of_kind.len() == 2 {
        return HandType::FullHouse;
    } else if max_of_kind == 3 && num_of_kind.len() == 3 {
        return HandType::ThreeOfAKind;
    } else if max_of_kind == 2 && num_of_kind.len() == 3 {
        return HandType::TwoOfAKind;
    } else if max_of_kind == 2 && num_of_kind.len() == 4 {
        return HandType::OneOfAKind;
    }
    return HandType::HighCard;
}

fn get_num_of_kind(hand: &Vec<char>) -> BTreeMap<&char, i32> {
    return hand.iter().fold(
        BTreeMap::new(),
        |mut acc: BTreeMap<&char, i32>, elem: &char| {
            let cur_count = acc.entry(elem).or_insert(0);
            *cur_count += 1;
            return acc;
        },
    );
}

fn get_max_of_kind(map: &BTreeMap<&char, i32>) -> i32 {
    return *map
        .iter()
        .fold((&&' ', &0i32), |a, b| if a.1 > b.1 { a } else { b })
        .1;
}

fn parse_list(elem: &mut Split<&str>) -> Vec<char> {
    return elem.next().unwrap().trim().chars().collect();
}

impl Debug for HandP1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Card")
            .field("Hand", &self.hand)
            .field("Bid", &self.bid)
            .field("Hand Type", &self.hand_type)
            .field("\n", &"")
            .finish()
    }
}

impl PartialOrd<Self> for HandP1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let card_to_val = BTreeMap::from([
            ('A', 13),
            ('K', 12),
            ('Q', 11),
            ('J', 10),
            ('T', 9),
            ('9', 8),
            ('8', 7),
            ('7', 6),
            ('6', 5),
            ('5', 4),
            ('4', 3),
            ('3', 2),
            ('2', 1),
            ('L', -1),
        ]);
        for (h1, h2) in self.hand.iter().zip(&other.hand) {
            if h1 != h2 {
                let n1 = card_to_val.get(h1).unwrap();
                let n2 = card_to_val.get(h2).unwrap();
                return n2.partial_cmp(n1);
            }
        }
        return Option::from(Ordering::Equal);
    }
}

impl Ord for HandP1 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialEq for HandP1 {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for HandP1 {}