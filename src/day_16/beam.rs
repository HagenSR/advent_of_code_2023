use std::collections::BTreeSet;
use std::fmt::Debug;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Beam {
    pub square: (i32, i32),
    pub direction: (i32, i32),
}

impl Beam {
    pub fn next_square(&mut self, grid: &Vec<Vec<char>>) -> (Vec<Beam>, BTreeSet<(i32, i32)>) {
        let mut next_beams: Vec<Beam> = Vec::new();
        let mut seen_squares: BTreeSet<(i32, i32)> = BTreeSet::new();
        let next_square = (
            self.square.0 + self.direction.0,
            self.square.1 + self.direction.1,
        );
        if next_square.0 >= 0
            && next_square.0 < grid.len() as i32
            && next_square.1 >= 0
            && next_square.1 < grid[0].len() as i32
        {
            seen_squares.insert(next_square);
            let next_char = grid[next_square.0 as usize][next_square.1 as usize];
            if vec!['.', '\\', '/'].contains(&next_char) {
                let mut next_dir = self.direction;
                if next_char == '/' {
                    next_dir = (-self.direction.1, -self.direction.0)
                }
                if next_char == '\\' {
                    next_dir = (self.direction.1, self.direction.0)
                }
                next_beams.push(Beam {
                    square: next_square.clone(),
                    direction: next_dir.clone(),
                })
            } else {
                if next_char == '|' {
                    if self.direction == (0, 1) || self.direction == (0, -1) {
                        next_beams.push(Beam {
                            square: next_square.clone(),
                            direction: (1, 0),
                        });
                        next_beams.push(Beam {
                            square: next_square.clone(),
                            direction: (-1, 0),
                        });
                    } else {
                        next_beams.push(Beam {
                            square: next_square.clone(),
                            direction: self.direction.clone(),
                        })
                    }
                }
                if next_char == '-' {
                    if self.direction == (1, 0) || self.direction == (-1, 0) {
                        next_beams.push(Beam {
                            square: next_square.clone(),
                            direction: (0, 1),
                        });
                        next_beams.push(Beam {
                            square: next_square.clone(),
                            direction: (0, -1),
                        });
                    } else {
                        next_beams.push(Beam {
                            square: next_square.clone(),
                            direction: self.direction.clone(),
                        })
                    }
                }
            }
        }
        return (next_beams, seen_squares);
    }
}

impl Debug for Beam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Beam")
            .field("square", &self.square)
            .field("direction", &self.direction)
            .finish()
    }
}
