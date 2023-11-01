mod day1;
mod lib;

use std::io;

use crate::day1::solve;

fn main() -> Result<(), io::Error> {
    solve()?;

    Ok(())
}
