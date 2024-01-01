use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Dijkstra {
    pub edges: BTreeMap<String, Vec<String>>,
    pub start: String,
    pub end: String,
    pub dist: BTreeMap<String, i64>,
    pub came_from: BTreeMap<String, String>,
}

impl Dijkstra {
    pub fn new(edges: BTreeMap<String, Vec<String>>, start: String, end: String) -> Self {
        return Dijkstra {
            edges,
            start,
            end,
            dist: BTreeMap::new(),
            came_from: BTreeMap::new(),
        };
    }

    pub fn perform_search(&mut self) -> Result<Vec<(String, String)>, &'static str> {
        let mut verticies: Vec<String> = self.edges.keys().cloned().into_iter().collect();
        self.dist = self.initialize_distance();

        while let Some(cur) = self.get_min_vert(&mut verticies) {
            if cur == self.end {
                return Ok(self.construct_path(cur));
            }
            let neighbors = self.edges.get(&cur).unwrap();
            for neighbor in neighbors {
                if !verticies.contains(neighbor) {
                    continue;
                }
                let tentative_gscore = self.dist.get(&cur).expect("No gscore") + 1;
                let cur_score = self.dist.get(&*neighbor);
                if cur_score.is_none() || tentative_gscore < *self.dist.get(&*neighbor).unwrap() {
                    self.came_from.insert(neighbor.clone(), cur.clone());
                    self.dist.insert(neighbor.clone(), tentative_gscore);
                }
            }
        }
        return Err("Failed to find path");
    }

    fn initialize_distance(&self) -> BTreeMap<String, i64> {
        let mut gscore: BTreeMap<String, i64> = BTreeMap::new();
        gscore.insert(self.start.clone(), 0);
        return gscore;
    }

    fn construct_path(&self, start: String) -> Vec<(String, String)> {
        let mut path: Vec<(String, String)> = Vec::new();
        let mut prev = start.clone();
        let mut current = start;
        while self.came_from.contains_key(&current) {
            current = self.came_from.get(&current).expect("Fail").clone();
            if current < prev {
                path.push((current.clone(), prev.clone()));
            } else {
                path.push((prev.clone(), current.clone()));
            }
            prev = current.clone();
        }
        return path;
    }

    fn get_min_vert(&mut self, verticies: &mut Vec<String>) -> Option<String> {
        let mut rtn = Option::None;
        let mut min_dist = i64::MAX;
        let mut ind = 0;
        for (cur_ind, vert) in verticies.iter().enumerate() {
            let cur_dist = *self.dist.get(vert).unwrap_or(&i64::MAX);
            if cur_dist <= min_dist {
                rtn = Option::from(vert.clone());
                min_dist = cur_dist;
                ind = cur_ind;
            }
        }
        if rtn.is_some() {
            verticies.remove(ind);
        }
        return rtn;
    }
}

impl Debug for Dijkstra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AStar").field("Grid", &self.edges).finish()
    }
}
