use std::fmt::Debug;
use std::str::FromStr;

#[derive(Clone)]
pub struct Derivative {
    pub values: Vec<i32>,
    pub differences: Vec<i32>,
    pub all_0: bool,
    pub next: i32,
    pub previous: i32,
}

impl Derivative {
    pub fn find_boundaries(derivative: &mut Derivative) -> (i32, i32) {
        construct_next(derivative);
        let mut found: Vec<Derivative> = vec![derivative.clone()];
        let mut last = found.last().unwrap();
        while !last.all_0 {
            let next = &mut Derivative {
                values: last.differences.clone(),
                differences: vec![],
                all_0: false,
                next: 0,
                previous: 0,
            };
            construct_next(next);
            found.push(next.clone());
            last = found.last().unwrap();
        }
        for ind in (0..found.len()).rev() {
            if ind == found.len() - 1 {
                found[ind].next = *found[ind].values.last().unwrap();
                found[ind].previous = *found[ind].values.last().unwrap();
            } else {
                found[ind].next = found[ind].values.last().unwrap() + found[ind + 1].next;
                found[ind].previous = found[ind].values.first().unwrap() - found[ind + 1].previous;
            }
        }
        let orig = found.first().unwrap();
        return (orig.previous, orig.next);
    }
}

fn construct_next(derivative: &mut Derivative) -> &mut Derivative {
    let mut all_0 = true;
    for ind in 1..derivative.values.len() {
        let diff = derivative.values[ind] - derivative.values[ind - 1];
        if diff != 0 {
            all_0 = false;
        }
        derivative.differences.push(diff);
        derivative.all_0 = all_0
    }
    return derivative;
}

impl FromStr for Derivative {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values: Vec<i32> = s
            .trim()
            .split(" ")
            .map(|elem| elem.trim().parse::<i32>().unwrap())
            .collect();
        return Ok(Derivative {
            values,
            differences: Vec::new(),
            all_0: false,
            next: 0,
            previous: 0,
        });
    }
}

impl Debug for Derivative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Derivative")
            .field("values", &self.values)
            .field("differences", &self.differences)
            .field("next", &self.next)
            .finish()
    }
}
