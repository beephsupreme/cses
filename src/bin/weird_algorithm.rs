//! Driver for the "Weird Algorithm" problem (https://cses.fi/problemset/task/1068)

#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::{BufReader, Cursor};

use cses::io::{get_value, vector_to_string};
use cses::prelude::*;
#[allow(unused_imports)]
use cses::weird_algorithm::{validate_weird_algorithm_input, weird_algorithm};

/// Select a Reader type to use for the input.
fn main() -> Result<()> {
    #![allow(unused_variables)]
    // let filename = "./data/weird_algorithm/test_input_002.txt".to_string();
    // let n: u64 = get_value(BufReader::new(File::open(filename)?))?;
    // let n: u64 = get_value(Cursor::new("17\n"))?;
    let n = get_value(BufReader::new(std::io::stdin()))?;
    validate_weird_algorithm_input(n)?;
    let r: Vec<u64> = weird_algorithm(n);
    let s: String = vector_to_string(r, Some(" "));
    println!("{}", s);
    Ok(())
}
