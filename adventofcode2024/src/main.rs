use std::env;

mod day01;

fn main() {
    let day = env::args().nth(1).expect("Please run with a day number");

    match day.parse() {
        Ok(1) => day01::run(),

        Ok(_) => println!("Invalid day number"),
        Err(_) => println!("Failed to parse day number"),
    }
}