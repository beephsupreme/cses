//! To run the code, use cargo run --bin increasing_array < data/increasing_array/test_input_005.txt
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::{BufReader, Cursor, Stdin};

use cses::increasing_array::*;
use cses::io::get_value_and_vector;
use cses::prelude::*;

fn main() -> Result<()> {
    #![allow(unused_variables)]
    // let filename = "./data/increasing_array/test_input_002.txt".to_string();
    // let (n, v) = get_value_and_vector(BufReader::new(File::open(filename)?))?;
    // let (n, v) = get_value_and_vector(Cursor::new("5\n3 2 5 1 7\n"))?;
    let (n, v) = get_value_and_vector(BufReader::new(std::io::stdin()))?;
    validate_increasing_array_input(n, &v)?;
    let r: u64 = increasing_array(v);
    println!("{}", r);
    Ok(())
}
