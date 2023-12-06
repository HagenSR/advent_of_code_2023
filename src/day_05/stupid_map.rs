use std::collections::BTreeMap;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone)]
pub struct StupidMap {
    pub from: String,
    pub to: String,
    pub map: BTreeMap<i128, (i128, i128)>,
}

impl StupidMap {}

impl FromStr for StupidMap {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split("\n");
        let mut name_parts = parts.next().unwrap().split("-");
        let mut map: BTreeMap<i128, (i128, i128)> = BTreeMap::new();
        while let Some(part) = parts.next() {
            let ls: Vec<i128> = parse_list(part);
            map.insert(ls[1], (ls[0], ls[2]));
        }
        let from = name_parts.next().unwrap().to_string();
        let to = name_parts.skip(1).next().unwrap().to_string();
        return Ok(StupidMap { to, from, map });
    }
}

fn parse_list(elem: &str) -> Vec<i128> {
    return elem
        .trim()
        .split(" ")
        .map(|el| el.trim().parse::<i128>())
        .filter(|wrap| wrap.is_ok())
        .map(|ok| ok.unwrap())
        .collect();
}

impl Debug for StupidMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StupidMap")
            .field("To", &self.to)
            .field("From", &self.from)
            .field("Map", &self.map)
            .finish()
    }
}
