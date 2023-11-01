mod lib;

// mod day1;
// use day1::solve;

mod day2;
use day2::solve;

fn main() {
    match solve() {
        Err(err) => println!("{}", err),
        _ => {}
    }
}
