mod util;
mod a_star_search;
mod day_01;

fn main() {
    let day_override = 1;
    match day_override{
        1 => day_01::day_01::main(),
        _ => println!("Not a valid day")
    }
}