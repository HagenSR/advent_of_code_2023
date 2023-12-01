mod util;
mod a_star_search;
mod day_01;
mod day_02;

fn main() {
    let day_override = 2;
    match day_override{
        1 => day_01::day_01::main(),
        2 => day_02::day_02::main(),
        _ => println!("Not a valid day")
    }
}