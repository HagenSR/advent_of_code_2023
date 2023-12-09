use crate::day_09::derivative::Derivative;
use crate::util;

pub fn main() {
    let lines = util::utils::read_file_to_lines::<Derivative>("src/day_09/data.txt", "\n");
    let mut next_values = 0i32;
    let mut previous_values = 0i32;
    for mut line in lines {
        let (previous, next) = Derivative::find_boundaries(&mut line);
        next_values += next;
        previous_values += previous;
    }
    println!("{}", next_values);
    println!("{}", previous_values)
}
