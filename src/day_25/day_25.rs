use crate::day_25::connection::Connection;
use crate::day_25::djikstras::Dijkstra;
use crate::util;
use rand::seq::SliceRandom;
use std::collections::{BTreeMap, BTreeSet};

pub fn main() {
    part_1();
}

fn part_1() {
    let connections = util::utils::read_file_to_lines::<Connection>("src/day_25/data.txt", "\n");
    let map = create_map(&connections);
    let names: Vec<String> = connections.iter().map(|elem| elem.name.clone()).collect();
    let mut counts: BTreeMap<(String, String), i32> = BTreeMap::new();
    let mut rand = rand::thread_rng();
    let mut is_good = false;
    while !is_good {
        for _ in 0..100 {
            let start = names.choose(&mut rand).unwrap();
            let end = names.choose(&mut rand).unwrap();
            let mut djik = Dijkstra::new(map.clone(), start.clone(), end.clone());
            let shortest = djik.perform_search().unwrap();
            for key in shortest {
                let entry = counts.entry(key).or_insert(0);
                *entry += 1;
            }
        }
        let cut_edges = get_cut_edges(&counts);
        let sizes = get_group_sizes(&cut_edges, &map);
        is_good = sizes.iter().sum::<usize>() == map.len();
        println!("{}", is_good);
        println!("{}", sizes[0] * sizes[1])
    }
}

fn create_map(connections: &Vec<Connection>) -> BTreeMap<String, Vec<String>> {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for connection in connections {
        for con in &connection.connections {
            let cur_vec = map.entry(connection.name.clone()).or_insert(Vec::new());
            cur_vec.push(con.clone());
            let other_vec = map.entry(con.clone()).or_insert(Vec::new());
            other_vec.push(connection.name.clone());
        }
    }
    return map;
}

fn get_group_sizes(
    cut_edges: &Vec<&(String, String)>,
    edges: &BTreeMap<String, Vec<String>>,
) -> Vec<usize> {
    let roots = vec![&cut_edges[0].0, &cut_edges[0].1];
    let mut sizes = vec![];
    for root in roots {
        let mut frontier: Vec<&String> = vec![&root];
        let mut seen: BTreeSet<&String> = BTreeSet::from([root]);
        while !frontier.is_empty() {
            let mut expansion: Vec<&String> = Vec::new();
            for node in frontier {
                let pot = edges.get(node).unwrap();
                for neighbor in pot {
                    if !seen.contains(&neighbor.to_string())
                        && !cut_edges.contains(&&(node.to_string(), neighbor.to_string()))
                        && !cut_edges.contains(&&(neighbor.to_string(), node.to_string()))
                    {
                        seen.insert(neighbor);
                        expansion.push(neighbor);
                    }
                }
            }
            frontier = expansion;
        }
        sizes.push(seen.len())
    }
    return sizes;
}

fn get_cut_edges(counts: &BTreeMap<(String, String), i32>) -> Vec<&(String, String)> {
    let mut entries: Vec<(&(String, String), &i32)> = counts.iter().collect();
    entries.sort_by(|e1, e2| e1.1.cmp(&e2.1));
    let removed_counts: Vec<&(String, String)> = entries.iter().map(|elem| elem.0).collect();
    let cut_edges = &removed_counts[removed_counts.len() - 3..];
    return cut_edges.iter().cloned().collect();
}
