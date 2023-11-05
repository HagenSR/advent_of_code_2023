use std::{collections::btree_map::BTreeMap};

#[allow(dead_code)]
pub struct AStarSearch {
    pub goal: (i32, i32),
    pub start: (i32, i32),
    pub max: i32,
    pub gscore: BTreeMap<(i32, i32), i64>,
    pub grid: Vec<Vec<i32>>,
    pub came_from: BTreeMap<(i32, i32), (i32, i32)>,
}
#[allow(dead_code)]
impl AStarSearch {
    pub fn perform_search(&mut self) -> Result<Vec<(i32, i32)>, &'static str> {
        let mut open_set: Vec<(i64, (i32, i32))> = Vec::new();
        open_set.push((0, self.start));
        self.gscore = self.initialize_gscore();
        let mut fscore = self.initialize_fscore();

        while open_set.len() > 0 {
            let current = open_set.remove(0);
            if current.1 == self.goal {
                return Ok(self.construct_path(current.1));
            }
            let neighbors = self.expand(current.1);
            for neighbor in neighbors {
                let tentative_gscore = self.gscore.get(&current.1).expect("No gscore")
                    + self.get_weight(neighbor, &self.grid);
                let cur_score = self.gscore.get(&neighbor);
                if cur_score.is_none() || tentative_gscore < *self.gscore.get(&neighbor).unwrap() {
                    self.came_from.insert(neighbor, current.1);
                    self.gscore.insert(neighbor, tentative_gscore);
                    let fscore_temp = tentative_gscore + self.evaluate_fscore(neighbor, self.max);
                    fscore.insert(neighbor, fscore_temp);
                    let search_result = open_set.binary_search(&(fscore_temp, neighbor));
                    if search_result.is_err() {
                        open_set.insert(search_result.err().unwrap(), (fscore_temp, neighbor))
                    }
                }
            }
        }
        return Err("Failed to find path");
    }

    fn initialize_gscore(&self) -> BTreeMap<(i32, i32), i64> {
        let mut gscore: BTreeMap<(i32, i32), i64> = BTreeMap::new();
        gscore.insert(self.start, 0);
        return gscore;
    }

    fn initialize_fscore(&self) -> BTreeMap<(i32, i32), i64> {
        let mut fscore: BTreeMap<(i32, i32), i64> = BTreeMap::new();
        let start_val = self.evaluate_fscore(self.start, self.max)
            + self.gscore.get(&self.start).expect("No key for Start");
        fscore.insert(self.start, start_val);
        return fscore;
    }

    fn expand(&self, node: (i32, i32)) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                if node.0 + x > 0 && node.0 + x < self.grid.len() as i32 {
                    neighbors.push((node.0 + x, node.1));
                }
                if node.0 + x > 0
                    && node.0 + x < self.grid.len() as i32
                    && node.1 + y > 0
                    && node.1 + y < self.grid[0].len() as i32
                {
                    neighbors.push((node.0 + x, node.1 + y));
                }
                if node.1 + y > 0 && node.1 + y < self.grid[0].len() as i32 {
                    neighbors.push((node.0, node.1 + y));
                }
            }
        }
        return neighbors;
    }

    fn get_weight(&self, elem: (i32, i32), grid: &Vec<Vec<i32>>) -> i64 {
        return grid[elem.0 as usize][elem.1 as usize] as i64;
    }

    fn evaluate_fscore(&self, node: (i32, i32), max: i32) -> i64 {
        return ((max - node.0) + (max - node.1)) as i64;
    }

    fn construct_path(&self, start: (i32, i32)) -> Vec<(i32, i32)> {
        let mut path: Vec<(i32, i32)> = Vec::new();
        path.push(start);
        let mut current: (i32, i32) = start;
        while self.came_from.contains_key(&current) {
            current = *self.came_from.get(&current).expect("Fail");
            path.push(current);
        }
        return path;
    }
}