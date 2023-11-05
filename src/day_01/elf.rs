use std::fmt::Debug;
use std::str::FromStr;

pub struct Elf {
    pub calories: Vec<i32>
}

impl FromStr for Elf {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories: Vec<i32> = s.split("\r\n")
            .filter(|p| !p.is_empty())
            .map(|p| p.parse().unwrap()).collect();
        return Ok(Elf { calories });
    }
}

impl Debug for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("People")
            .field("Calories", &self.calories)
            .finish()
    }
}
