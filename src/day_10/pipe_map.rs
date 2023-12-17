use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Clone)]
pub struct PipeMap {
    pub grid: Vec<Vec<char>>,
    pub filtered: Vec<Vec<char>>,
    pub starting: (usize, usize),
    pub distances: BTreeMap<(usize, usize), i32>,
    pub fscore: BTreeMap<(usize, usize), i32>,
    pub came_from: BTreeMap<(usize, usize), (usize, usize)>,
    pub char_map: BTreeMap<char, Vec<(i32, i32)>>,
}

impl PipeMap {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let mut starting = (0, 0);
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y] == 'S' {
                    starting = (x, y);
                }
            }
        }
        let char_map = BTreeMap::from([
            ('|', vec![(-1, 0), (1, 0)]),
            ('-', vec![(0, 1), (0, -1)]),
            ('L', vec![(-1, 0), (0, 1)]),
            ('J', vec![(-1, 0), (0, -1)]),
            ('7', vec![(1, 0), (0, -1)]),
            ('F', vec![(1, 0), (0, 1)]),
        ]);
        let grid_len = grid.len();
        return PipeMap {
            grid,
            filtered: vec![vec!['X'; grid_len]; grid_len],
            starting,
            distances: BTreeMap::new(),
            fscore: BTreeMap::new(),
            came_from: BTreeMap::new(),
            char_map
        };
    }

    pub fn find_dists(&mut self) {
        let mut frontier = vec![self.starting];
        let mut seen: Vec<((usize, usize), (usize, usize))> = Vec::new();
        while let Some(cur) = self.get_lowest_fscore(&mut frontier) {
            let pots = self.get_surrounding(cur);
            self.filtered[cur.0][cur.1] = self.grid[cur.0][cur.1];
            for pot in pots {
                if self.are_connected(cur, pot) {
                    let tentative = self.distances.get(&cur).unwrap_or(&0) + 1;
                    if tentative < *self.distances.get(&pot).unwrap_or(&i32::MAX) {
                        self.came_from.insert(pot, cur);
                        self.distances.insert(pot, tentative);
                        self.fscore.insert(pot, tentative + 1);
                    }
                    if !seen.contains(&(cur, pot)) {
                        frontier.push(pot);
                        seen.push((cur, pot));
                    }
                }
            }
        }
        println!("{:?}", self.distances.values().max().unwrap())
    }
    pub fn find_islands(&mut self) {}

    pub fn get_surrounding(&mut self, start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut potential: Vec<(usize, usize)> = vec![];
        if start.0 > 0 {
            potential.push((start.0 - 1, start.1));
        }
        if start.0 < self.grid.len() - 1 {
            potential.push((start.0 + 1, start.1))
        }
        if start.1 > 0 {
            potential.push((start.0, start.1 - 1))
        }
        if start.1 < self.grid.len() - 1 {
            potential.push((start.0, start.1 + 1))
        }
        return potential;
    }

    fn are_connected(&mut self, one: (usize, usize), two: (usize, usize)) -> bool {
        let con_pipe = self.char_map.get(&self.grid[two.0][two.1]);
        if con_pipe.is_some() {
            for con in con_pipe.unwrap() {
                let res = (
                    (two.0 as i32 + con.0) as usize,
                    (two.1 as i32 + con.1) as usize,
                );
                if res == one {
                    return true;
                }
            }
        }
        return false;
    }

    fn get_lowest_fscore(&self, frontier: &mut Vec<(usize, usize)>) -> Option<(usize, usize)> {
        let mut lowest = Option::None;
        if frontier.is_empty() {
            return lowest;
        }
        let mut lowest_f = i32::MAX;
        let mut ind_remove = 0;
        for (ind, pot) in frontier.iter().enumerate() {
            let pot_f = self.fscore.get(&pot).unwrap_or(&0);
            if pot_f < &lowest_f {
                lowest_f = *pot_f;
                lowest = Option::from(pot.clone());
                ind_remove = ind;
            }
        }
        frontier.remove(ind_remove);
        return lowest;
    }

    pub fn print_distances(&self) {
        let mut grid: Vec<Vec<String>> =
            vec![vec![" ".to_string(); self.grid.len()]; self.grid.len()];
        for (tup, dist) in &self.distances {
            grid[tup.0][tup.1] = dist.to_string();
        }
        for line in grid {
            println!("{:?}", line)
        }
    }

    pub fn print_filtered(&self) {
        for line in &self.filtered {
            println!("{:?}", line)
        }
    }
}

impl Debug for PipeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipeMap").field("Grid", &self.grid).finish()
    }
}
