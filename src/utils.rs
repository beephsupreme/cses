use std::{fmt::Display, fs, ops::RangeInclusive};

use crate::prelude::*;

pub fn validate_range<T>(n: T, rng: RangeInclusive<T>) -> Result<bool>
where
    T: Display + PartialOrd + PartialEq,
{
    match rng.contains(&n) {
        true => Ok(true),
        false => Err(Error::InvalidInput(f!(
            "n is {} but expected {} ≤ n ≤ {}",
            n,
            rng.start(),
            rng.end()
        ))),
    }
}

pub fn get_test_filenames() -> Result<(Vec<String>, Vec<String>)> {
    let paths = fs::read_dir("./data/")?;
    let mut questions: Vec<String> = Vec::new();
    let mut answers: Vec<String> = Vec::new();
    for p in paths {
        let p = p?;
        if !p.path().is_file() {
            continue;
        }
        let file = p.path().display().to_string();
        if file.contains("input") {
            questions.push(file);
        } else if file.contains("output") {
            answers.push(file);
        }
    }
    if questions.len() != answers.len() {
        return Err(Error::SizeMismatch(questions.len(), answers.len()));
    }
    questions.sort();
    answers.sort();
    Ok((questions, answers))
}
