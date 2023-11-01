mod lib;

// mod day1;
// use day1::solve;

// mod day2;
// use day2::solve;

mod day3;
use day3::solve;

fn main() {
    match solve() {
        Err(err) => println!("{}", err),
        _ => {}
    }
}
