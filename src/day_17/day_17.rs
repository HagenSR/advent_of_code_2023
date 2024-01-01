use crate::day_17::a_star::AStar;
use crate::util;

pub fn main() {
    part_1();
    // part_2();
}

fn part_1() {
    let file = util::utils::read_file_to_string("src/day_17/data.txt");
    let map = util::utils::parse_to_grid::<i64>(&file);
    let mut astar = AStar::new(map.clone(), (0, 0), (map.len() - 1, map[0].len() - 1));
    let mut res = astar.perform_search().unwrap_or(Vec::new());
    res.remove(res.len() - 1);
    let sum: i64 = res.iter().map(|tup| map[tup.0][tup.1]).sum();
    println!("{}", sum);
    println!("{:?}", astar.print_distances(res))
}

// 760 too high

// fn part_2() {}
