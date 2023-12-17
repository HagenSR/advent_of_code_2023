use crate::util;

pub fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let parts: Vec<String> = util::utils::read_file_to_lines("src/day_15/data.txt", ",");
    let mut sum: u128 = 0;
    for part in parts {
        sum += hash(&part);
    }
    println!("{}", sum)
}

fn part_2() {
    let parts: Vec<String> = util::utils::read_file_to_lines("src/day_15/data.txt", ",");
    let mut map: Vec<Vec<(String, u128)>> = vec![Vec::new(); 256];
    for part in parts {
        let ind = hash(&part.split("=").next().unwrap().replace("-", "")) as usize;
        if part.contains("=") {
            let mut label = part.split("=");
            let entry = (
                label.next().unwrap().to_string(),
                label.next().unwrap().parse::<u128>().unwrap(),
            );
            let position = map[ind].iter().position(|elem| elem.0 == entry.0);
            if position.is_some() {
                map[ind][position.unwrap()] = entry;
            } else {
                map[ind].push(entry.clone());
            }
        } else {
            let position = map[ind]
                .iter()
                .position(|elem| elem.0 == part[..part.len() - 1]);
            if position.is_some() {
                map[ind].remove(position.unwrap());
            }
        }
    }

    let mut sum: u128 = 0;
    for (boxInd, val) in map.iter().enumerate() {
        for (lensInd, lens) in val.iter().enumerate() {
            let m = lens.1;
            sum += (boxInd as u128 + 1) * (lensInd as u128 + 1) * m;
        }
    }
    println!("{}", sum)
}

fn hash(ras: &str) -> u128 {
    let mut cur_val = 0;
    for char in ras.chars() {
        cur_val += (char as u8) as u128;
        cur_val *= 17;
        cur_val = cur_val % 256;
    }
    return cur_val;
}
