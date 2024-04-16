//! Driver for the "Permutations" problem (https://cses.fi/problemset/task/1070)

#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::{BufReader, Cursor};

use cses::io::{get_value, vector_to_string};
use cses::permutations::*;
#[allow(unused_imports)]
use cses::prelude::*;

/// Select a Reader type to use for the input.
fn main() -> Result<()> {
    #![allow(unused_variables)]
    // let filename = "./data/permutations/test_input_002.txt".to_string();
    // let n: u64 = get_value(BufReader::new(File::open(filename)?))?;
    // let n: u64 = get_value(Cursor::new("5\n"))?;
    let n = get_value(BufReader::new(std::io::stdin()))?;
    validate_permutations_input(n)?;
    let r: Vec<u64> = match permutations(n) {
        Some(v) => v,
        None => {
            println!("NO SOLUTION");
            return Ok(());
        }
    };
    let s: String = vector_to_string(r, Some(" "));
    println!("{}", s);
    Ok(())
}
