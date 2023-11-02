mod utils;

// mod day1;
// use day1::solve;

// mod day2;
// use day2::solve;

// mod day3;
// use day3::solve;

// mod day4;
// use day4::solve;

mod day5;
use day5::solve;

fn main() {
    match solve() {
        Err(err) => println!("{}", err),
        _ => {}
    }
}
