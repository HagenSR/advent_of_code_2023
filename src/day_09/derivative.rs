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
    pub fn new(values: Vec<i32>) -> Self {
        Derivative {
            values,
            differences: vec![],
            all_0: false,
            next: 0,
            previous: 0,
        }
    }

    pub fn find_boundaries(derivative: &mut Derivative) -> (i32, i32) {
        let mut found = &mut vec![derivative.clone()];
        find_differences(&mut found);
        find_boundaries(&mut found);
        let orig = found.first().unwrap();
        return (orig.previous, orig.next);
    }
}

fn find_differences(found: &mut Vec<Derivative>) {
    let mut last = found.last_mut().unwrap();
    find_difference(last);
    while !last.all_0 {
        let next = &mut Derivative::new(last.differences.clone());
        find_difference(next);
        found.push(next.clone());
        last = found.last_mut().unwrap();
    }
}

fn find_difference(der: &mut Derivative) {
    let mut all_0 = true;
    for ind in 1..der.values.len() {
        let diff = der.values[ind] - der.values[ind - 1];
        if diff != 0 {
            all_0 = false;
        }
        der.differences.push(diff);
        der.all_0 = all_0
    }
}

fn find_boundaries(found: &mut Vec<Derivative>) {
    for ind in (0..found.len()).rev() {
        if ind == found.len() - 1 {
            found[ind].next = *found[ind].values.last().unwrap();
            found[ind].previous = *found[ind].values.first().unwrap();
        } else {
            found[ind].next = found[ind].values.last().unwrap() + found[ind + 1].next;
            found[ind].previous = found[ind].values.first().unwrap() - found[ind + 1].previous;
        }
    }
}

impl FromStr for Derivative {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<i32> = s
            .trim()
            .split(" ")
            .map(|elem| elem.trim().parse::<i32>().unwrap())
            .collect();
        return Ok(Derivative::new(values));
    }
}

impl Debug for Derivative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Derivative")
            .field("values", &self.values)
            .field("differences", &self.differences)
            .field("next", &self.next)
            .field("previous", &self.previous)
            .finish()
    }
}
