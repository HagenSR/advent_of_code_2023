mod a_star_search;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

mod util;

fn main() {
    let day_override = 9;
    match day_override {
        1 => day_01::day_01::main(),
        2 => day_02::day_02::main(),
        3 => day_03::day_03::main(),
        4 => day_04::day_04::main(),
        5 => day_05::day_05::main(),
        6 => day_06::day_06::main(),
        7 => day_07::day_07::main(),
        8 => day_08::day_08::main(),
        9 => day_09::day_09::main(),
        _ => println!("Not a valid day"),
    }
}
