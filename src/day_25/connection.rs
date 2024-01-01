use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Connection {
    pub name: String,
    pub connections: Vec<String>,
}

impl FromStr for Connection {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let name = parts.next().unwrap().trim().to_string();
        let connections: Vec<String> = parts
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|elem| elem.to_string())
            .collect();
        return Ok(Connection { name, connections });
    }
}

impl Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("name", &self.name)
            .field("connections", &self.connections)
            .finish()
    }
}
