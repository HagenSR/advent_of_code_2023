mod util;
mod a_star_search;
mod day_01;
mod day_02;
mod day_03;

fn main() {
    let day_override = 3;
    match day_override {
        1 => day_01::day_01::main(),
        2 => day_02::day_02::main(),
        3 => day_03::day_03::main(),
        _ => println!("Not a valid day")
    }
}