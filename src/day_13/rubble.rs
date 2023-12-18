use crate::util;
use std::fmt::Debug;
use std::str::FromStr;

pub struct Rubble {
    pub id: i32,
    pub grid: Vec<Vec<char>>,
}

impl Rubble {
    pub fn find_reflection(&self, expected_diff: usize) -> (usize, usize) {
        let mut rtn = (0, 0);
        for x in 1..self.grid[0].len() {
            if self.vertical_reflection_diff_count(x) == expected_diff {
                rtn = (0, x)
            }
        }
        for x in 1..self.grid.len() {
            if self.horizontal_reflection_diff_count(x) == expected_diff {
                rtn = (x, 0)
            }
        }
        return rtn;
    }

    fn vertical_reflection_diff_count(&self, split: usize) -> usize {
        let mut count = 0;
        for x in 0..self.grid.len() {
            for y_ind in 1..split + 1 {
                if (split + y_ind - 1) < self.grid[0].len() {
                    if self.grid[x][split - y_ind] != self.grid[x][split + y_ind - 1] {
                        count += 1;
                    }
                }
            }
        }
        return count;
    }

    fn horizontal_reflection_diff_count(&self, split: usize) -> usize {
        let mut good = 0;
        for y in 0..self.grid[0].len() {
            for x_ind in 1..split + 1 {
                if (split + x_ind - 1) < self.grid.len() {
                    if self.grid[split - x_ind][y] != self.grid[split + x_ind - 1][y] {
                        good += 1;
                    }
                }
            }
        }
        return good;
    }
}

impl FromStr for Rubble {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = util::utils::parse_to_grid::<char>(s);
        return Ok(Rubble { id: 0, grid });
    }
}

impl Debug for Rubble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rubble")
            .field("id", &self.id)
            .field("grid", &self.grid)
            .finish()
    }
}
