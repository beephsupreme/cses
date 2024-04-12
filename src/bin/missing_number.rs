//! Driver for the missing number problem from CSES problem set (https://cses.fi/problemset/task/1083).

use cses::io::{get_token, get_vector, load_all_tokens};
use cses::missing_number::*;
use cses::prelude::*;

fn main() -> Result<()> {
    let reader = std::io::BufReader::new(std::io::stdin());
    let mut buffer: String = String::new();
    let mut tokens = load_all_tokens(reader, &mut buffer)?;
    let n: u64 = get_token(&mut tokens)?;
    let v: Vec<u64> = get_vector(&mut tokens)?;
    let r: u64 = missing_number(n, v)?;
    println!("{}", r);
    Ok(())
}
