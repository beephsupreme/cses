//! Driver for the "Weird Algorithm" problem (https://cses.fi/problemset/task/1068)

use std::io::BufReader;

use cses::io::{get_token, load_tokens, vector_to_string};
use cses::prelude::*;
use cses::weird_algorithm::*;

fn main() -> Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut buffer: String = String::new();
    let mut tokens = load_tokens(reader, &mut buffer)?;
    let n: u64 = get_token(&mut tokens)?;
    let r: Vec<u64> = weird_algorithm(n)?;
    let s: String = vector_to_string(r, Some(" "));
    println!("{}", s);
    Ok(())
}
