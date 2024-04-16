use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use log::info;

use cses::increasing_array::increasing_array;
use cses::io::{get_token, get_vector, load_all_tokens};
use cses::utils::get_test_filenames;

#[cfg(test)]

/// Runs the missing number problem from CSES Problem Set.
/// Uses the same data files as the CSES Grader.
#[test]
fn missing_number_integration() -> Result<()> {
    env_logger::init();
    let mut loc = "increasing_array".to_string();
    let (questions, answers) = get_test_filenames(&mut loc)?;
    for i in 0..questions.len() {
        info!("{}: {}", i + 1, questions[i]);
        let mut q_reader = BufReader::new(File::open(&questions[i])?);
        let mut a_reader = BufReader::new(File::open(&answers[i])?);
        let mut q_buffer: String = String::new();
        let mut a_buffer: String = String::new();
        let mut q_tokens = load_all_tokens(&mut q_reader, &mut q_buffer)?;
        let mut a_tokens = load_all_tokens(&mut a_reader, &mut a_buffer)?;
        let _: u64 = get_token(&mut q_tokens)?;
        let v: Vec<u64> = get_vector(&mut q_tokens)?;
        let a: u64 = get_token(&mut a_tokens)?;
        let r: u64 = increasing_array(v);
        assert_eq!(a, r);
    }
    Ok(())
}
