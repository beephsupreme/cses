use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use log::info;

use cses::io::{get_token, get_vector, load_all_tokens};
use cses::utils::get_test_filenames;
use cses::weird_algorithm::weird_algorithm;

#[test]
fn weird_algorithm_integration() -> Result<()> {
    env_logger::init();
    let (questions, answers) = get_test_filenames()?;
    for i in 0..questions.len() {
        info!("{}: {}", i + 1, questions[i]);
        let mut q_reader = BufReader::new(File::open(&questions[i])?);
        let mut a_reader = BufReader::new(File::open(&answers[i])?);
        let mut q_buffer: String = String::new();
        let mut a_buffer: String = String::new();
        let mut q_tokens = load_all_tokens(&mut q_reader, &mut q_buffer)?;
        let mut a_tokens = load_all_tokens(&mut a_reader, &mut a_buffer)?;
        let n: u64 = get_token(&mut q_tokens)?;
        let a: Vec<u64> = get_vector(&mut a_tokens)?;
        let r: Vec<u64> = weird_algorithm(n)?;
        assert_eq!(a, r);
    }
    Ok(())
}
