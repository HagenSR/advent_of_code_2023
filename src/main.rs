mod util;
mod a_star_search;
mod day_01;
mod day_02;
mod day_03;
mod day_04;

fn main() {
    let day_override = 4;
    match day_override {
        1 => day_01::day_01::main(),
        2 => day_02::day_02::main(),
        3 => day_03::day_03::main(),
        4 => day_04::day_04::main(),
        _ => println!("Not a valid day")
    }
}