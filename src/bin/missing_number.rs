//! Driver for the missing number problem from CSES problem set (https://cses.fi/problemset/task/1083).

#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::{BufReader, Cursor, Stdin};

use cses::io::get_value_and_vector;
use cses::missing_number::*;
use cses::prelude::*;

/// Select a Reader type to use for the input.
fn main() -> Result<()> {
    #![allow(unused_variables)]
    // let filename = "./data/missing_number/test_input_002.txt".to_string();
    // let (n, v) = get_value_and_vector(BufReader::new(File::open(filename)?))?;
    // let (n, v) = get_value_and_vector(Cursor::new("5\n2 3 4 5\n"))?;
    let (n, v) = get_value_and_vector(BufReader::new(std::io::stdin()))?;
    validate_missing_number_input(n, &v)?;
    let r: u64 = missing_number(n, v);
    println!("{}", r);
    Ok(())
}