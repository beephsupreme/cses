//! Driver for the "Repetitions" problem (https://cses.fi/problemset/task/1069)

use cses::prelude::*;
use cses::repetitions::repetitions;

fn main() -> Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let buffer = buffer.trim().to_string();
    let n: u64 = repetitions(buffer)?;
    println!("{}", n);
    Ok(())
}
