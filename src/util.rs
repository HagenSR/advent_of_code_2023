#[allow(dead_code)]
#[allow(unused_imports)]
pub mod utils {
    use std::collections::BTreeMap;
    use std::fmt::Debug;
    use std::fs;
    use std::str::FromStr;
    use crate::a_star_search::AStarSearch;

    pub fn read_file_to_string(filepath: &str) -> String {
        return fs::read_to_string(filepath).expect("Failed to read file");
    }

    pub fn read_file_to_lines<T: FromStr>(filepath: &str, delimiter: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
        return read_file_to_string(filepath)
            .split(delimiter)
            .filter(|p| !p.is_empty())
            .map(|r| r.trim().parse().unwrap())
            .collect();
    }

    pub fn group_by() {
        let letters = "abcabcde";
        let groups = letters.chars().fold(
            BTreeMap::new(),
            |mut acc: BTreeMap<char, i32>, elem: char| {
                let cur_count = acc.entry(elem).or_insert(0);
                *cur_count += 1;
                return acc;
            },
        );
        println!("{:?}", groups)
    }

    pub fn parse_to_object<T: FromStr>(line: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
        return line.split("\n").map(|row| row.parse().unwrap()).collect();
    }

    pub fn parse_to_grid<T: FromStr>(file: &str) -> Vec<Vec<T>> where <T as FromStr>::Err: Debug {
        let grid: Vec<Vec<T>> = file
            .split("\n")
            .map(|row| {
                row.trim().chars()
                    .map(|char| char.to_string())
                    .map(|char| char.parse::<T>().unwrap())
                    .collect()
            })
            .collect();
        return grid;
    }

    // pub fn a_star_search<T>(grid: Vec<Vec<T>>) {
    //     let mut astar: AStarSearch = AStarSearch {
    //         goal: (3, 3),
    //         start: (0, 0),
    //         max: 3,
    //         gscore: BTreeMap::new(),
    //         grid: grid,
    //         came_from: BTreeMap::new(),
    //     };
    //     let result = astar.perform_search();
    //     if !result.is_err() {
    //         let mut rev = result.unwrap();
    //         rev.reverse();
    //         println!("{:?}", rev)
    //     }
    // }
}