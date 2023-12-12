use crate::day_10::pipe_map::PipeMap;
use crate::util;

pub fn main() {
    // part_1();
    part_2();
}

fn part_1(){
    let file = util::utils::read_file_to_string("src/day_10/data.txt");
    let map = util::utils::parse_to_grid::<char>(&file);
    let mut pip_map = PipeMap::new(map);
    pip_map.find_dists();
}

fn part_2(){
    let file = util::utils::read_file_to_string("src/day_10/data.txt");
    let map = util::utils::parse_to_grid::<char>(&file);
    let mut pip_map = PipeMap::new(map);
    pip_map.find_dists();
    pip_map.print_filtered();
}
