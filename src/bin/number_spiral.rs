//! Driver for the number spiral problem from CSES problem set (https://cses.fi/problemset/task/1071).

#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::{BufReader, Cursor, Stdin};

use cses::io::vector_to_string;
use cses::number_spiral::*;
use cses::prelude::*;

/// Select a Reader type to use for the input.
fn main() -> Result<()> {
    #![allow(unused_variables)]
    // let filename = "./data/number_spiral/test_input_002.txt".to_string();
    // let (n, v) = number_spiral_get_input(BufReader::new(File::open(filename)?))?;
    // let (n, v) = number_spiral_get_input(Cursor::new("5\n2 3 4 5\n"))?;
    let (n, v) = number_spiral_get_input(BufReader::new(std::io::stdin()))?;
    validate_number_spiral_input(n, &v)?;
    let r: Vec<u64> = number_spiral(n, v);
    println!("{}", vector_to_string(r, Some("\n")));
    Ok(())
}
