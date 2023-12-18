use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Search {
    pub grid: Vec<Vec<i64>>,
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub gscore: BTreeMap<(usize, usize), i64>,
    pub came_from: BTreeMap<(usize, usize), (usize, usize)>,
}

impl Search {
    pub fn new(grid: Vec<Vec<i64>>, start: (usize, usize), end: (usize, usize)) -> Self {
        return Search {
            grid,
            start,
            end,
            gscore: BTreeMap::new(),
            came_from: BTreeMap::new(),
        };
    }

    pub fn perform_search(&mut self) -> Result<Vec<(usize, usize)>, &'static str> {
        let mut open_set: Vec<(i64, (usize, usize))> = Vec::new();
        open_set.push((0, self.start));
        self.gscore = self.initialize_gscore();
        while open_set.len() > 0 {
            let current = open_set.remove(0);
            let neighbors = self.expand(current.1);
            for neighbor in neighbors {
                let tentative_gscore = self.gscore.get(&current.1).expect("No gscore")
                    + self.get_weight(neighbor, &self.grid);
                let cur_score = self.gscore.get(&neighbor);
                if cur_score.is_none() || tentative_gscore < *self.gscore.get(&neighbor).unwrap() {
                    self.came_from.insert(neighbor, current.1);
                    self.gscore.insert(neighbor, tentative_gscore);
                    let search_result = open_set.binary_search(&(0, neighbor));
                    if search_result.is_err() {
                        open_set.insert(search_result.err().unwrap(), (0, neighbor))
                    }
                }
            }
        }
        return Err("Failed to find path");
    }

    fn initialize_gscore(&self) -> BTreeMap<(usize, usize), i64> {
        let mut gscore: BTreeMap<(usize, usize), i64> = BTreeMap::new();
        gscore.insert(self.start, 0);
        return gscore;
    }

    pub fn expand(&mut self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let (run_length, prev_dir) = self.get_num_in_same_dir(start);
        let directions: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut potential: Vec<(usize, usize)> = vec![];
        let dir_ind = directions.iter().position(|elem| elem == &prev_dir).unwrap_or(1) as i32;
        for i in -1i32..2 {
            let ind = if dir_ind == 0 && i == -1 { 3 } else { ((dir_ind + i) % 4i32) as usize };
            let dir = directions[ind];
            let dif_res = (start.0 as i32 + dir.0 , start.1 as i32 + dir.1);
            if (dif_res.0 >= 0
                && dif_res.0 < self.grid.len() as i32
                && dif_res.1 >= 0
                && dif_res.1 < self.grid.len() as i32)
                && (run_length < 4 || prev_dir != dir)
            {
                potential.push((dif_res.0 as usize, dif_res.1 as usize));
            }
        }
        return potential;
    }

    fn get_num_in_same_dir(&self, start: (usize, usize)) -> (usize, (i32, i32)) {
        let mut res = 0;
        let mut cur = start;
        let mut prev = self.came_from.get(&cur);
        let mut wrap_dif = Option::None;
        if let Some(unwr) = prev {
            let dif = (cur.0 as i32 - unwr.0 as i32, cur.1 as i32 - unwr.1 as i32);
            wrap_dif = Option::from(dif);
            while let Some(unwr) = prev {
                res += 1;
                let new_dif = (cur.0 as i32 - unwr.0 as i32, cur.1 as i32 - unwr.1 as i32);
                if new_dif != dif {
                    break;
                }
                prev = self.came_from.get(unwr);
                cur = *unwr;
            }
        }
        return (res, wrap_dif.unwrap_or((-1, -1)));
    }

    fn get_weight(&self, elem: (usize, usize), grid: &Vec<Vec<i64>>) -> i64 {
        return grid[elem.0][elem.1];
    }

    fn construct_path(&self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut path: Vec<(usize, usize)> = Vec::new();
        path.push(start);
        let mut current = start;
        while self.came_from.contains_key(&current) {
            current = *self.came_from.get(&current).expect("Fail");
            path.push(current);
        }
        return path;
    }

    pub fn print_distances(&self, path: Vec<(usize, usize)>) {
        let mut grid: Vec<Vec<String>> =
            vec![vec![" ".to_string(); self.grid.len()]; self.grid.len()];
        // for x in 0..self.grid.len(){
        //     for y in 0..self.grid[0].len(){
        //         grid[x][y] = self.grid[x][y].to_string();
        //     }
        // }
        for tup in &path {
            grid[tup.0][tup.1] = "*".parse().unwrap();
        }
        for line in grid {
            println!("{:?}", line.join(""))
        }
    }
}

impl Debug for Search {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AStar").field("Grid", &self.grid).finish()
    }
}
