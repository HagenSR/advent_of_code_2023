use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone)]
pub struct Node {
    pub name: String,
    pub left_right: (String, String),
}

impl FromStr for Node {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("=");
        let name = parts.next().unwrap().trim().to_string();
        let spl = parts
            .next()
            .unwrap()
            .trim()
            .replace("(", "")
            .replace(")", "");
        let mut binding = spl.split(",");
        let left_right = (
            binding.next().unwrap().trim().to_string(),
            binding.next().unwrap().trim().to_string(),
        );
        return Ok(Node { name, left_right });
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field("left_right", &self.left_right)
            .finish()
    }
}
