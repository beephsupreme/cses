//! Driver for the "Repetitions" problem (https://cses.fi/problemset/task/1069)

#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::{BufReader, Cursor};

use cses::io::get_value;
use cses::prelude::*;
use cses::repetitions::*;

/// Select a Reader type to use for the input.
fn main() -> Result<()> {
    #![allow(unused_variables)]
    // let filename = "./data/repetitions/test_input_002.txt".to_string();
    // let input: String = get_value(BufReader::new(File::open(filename)?))?;
    // let inupt: String = get_value(Cursor::new("AAAGGTCC\n"))?;
    let input: String = get_value(BufReader::new(std::io::stdin()))?;
    validate_repetitions_input(&input)?;
    let r: u64 = repetitions(input);
    println!("{}", r);
    Ok(())
}
