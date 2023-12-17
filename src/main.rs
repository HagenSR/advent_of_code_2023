mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_15;
mod day_16;

mod util;

fn main() {
    let day_override = 15;
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
        10 => day_10::day_10::main(),
        11 => day_11::day_11::main(),
        15 => day_15::day_15::main(),
        16 => day_16::day_16::main(),
        _ => println!("Not a valid day"),
    }
}
