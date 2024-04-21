use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use log::info;

use cses::io::{get_token, get_vector, load_all_tokens};
use cses::permutations::permutations;
use cses::utils::get_test_filenames;

#[cfg(test)]

/// Runs the weird algorithm problem from CSES Problem Set.
/// Uses the same data files as the CSES Grader.
#[test]
fn permutations_integration() -> Result<()> {
    env_logger::init();
    let mut loc = "permutations".to_string();
    let (questions, answers) = get_test_filenames(&mut loc)?;
    for i in 0..questions.len() {
        info!("{}: {}", i + 1, questions[i]);
        let mut q_reader = BufReader::new(File::open(&questions[i])?);
        let mut a_reader = BufReader::new(File::open(&answers[i])?);
        let mut q_buffer: String = String::new();
        let mut q_tokens = load_all_tokens(&mut q_reader, &mut q_buffer)?;
        let n: u64 = get_token(&mut q_tokens)?;
        let r: Option<Vec<u64>> = permutations(n);
        let mut a_buffer: String = String::new();
        let mut a_tokens = load_all_tokens(&mut a_reader, &mut a_buffer)?;
        match &r {
            Some(_) => {
                let a: Vec<u64> = get_vector(&mut a_tokens)?;
                assert_eq!(a, r.unwrap());
            }
            None => {
                let s: String = get_token(&mut a_tokens)?;
                assert_eq!(s, "NO".to_string());
            }
        }
    }
    Ok(())
}
