use std::fmt::Debug;
use std::str::FromStr;

pub struct Game {
    pub id: i32,
    pub turns: Vec<Vec<(String, i32)>>,
}

impl FromStr for Game {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let id = parts
            .next()
            .unwrap()
            .replace("Game ", "")
            .parse::<i32>()
            .expect("");
        let turns_string = parts.next().unwrap().split(";");
        let mut turns: Vec<Vec<(String, i32)>> = Vec::new();
        for game in turns_string {
            let mut turn: Vec<(String, i32)> = Vec::new();
            let colors_and_numbers = game.split(",");
            for pul in colors_and_numbers {
                let mut color_and_number = pul.trim().split(" ");
                let num = color_and_number
                    .next()
                    .unwrap()
                    .trim()
                    .parse::<i32>()
                    .unwrap();
                turn.push((color_and_number.next().unwrap().trim().to_string(), num));
            }
            turns.push(turn);
        }
        return Ok(Game { id, turns });
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Game")
            .field("id", &self.id)
            .field("turns", &self.turns)
            .finish()
    }
}
