use std::{fmt::Display, fs, ops::RangeInclusive};

use crate::error::LibraryError;
use crate::prelude::*;

/// Checks if a number n is within the range [start, end]. A name for the variable n is provided for error messages.
pub fn validate_range<T>(name: String, n: T, rng: &RangeInclusive<T>) -> Result<()>
where
    T: Display + PartialOrd + PartialEq,
{
    match rng.contains(&n) {
        true => Ok(()),
        false => {
            let expected = format!("expected: {} ≤ {} ≤ {}", rng.start(), name, rng.end());
            let found = f!("found: {}", n);
            Err(LibraryError::OutOfRange(name, expected, found))
        }
    }
}

/// Retrieves the filenames contained in the given subfolder of _data_ located in the project root.
/// Returns a tuple of sorted vectors: one for the input files and one for the output files.
pub fn get_test_filenames(loc: &mut String) -> Result<(Vec<String>, Vec<String>)> {
    loc.insert_str(0, "./data/");
    let paths = fs::read_dir(loc)?;
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
        return Err(LibraryError::SizeMismatch(questions.len(), answers.len()));
    }
    questions.sort();
    answers.sort();
    Ok((questions, answers))
}

/// Retrieves the filenames of the binary files located in the _src/bin_ directory.
/// Returns a sorted vector of the filenames. Excludes the _welcome_ binary.
pub fn get_binary_filenames() -> Result<Vec<String>> {
    let paths = fs::read_dir("./src/bin")?;
    let mut filenames: Vec<String> = Vec::new();
    for p in paths {
        let p = p?;
        if !p.path().is_file() {
            continue;
        }
        let file = p.path().display().to_string();
        if !file.contains("welcome") && file.contains(".rs") {
            let filename = file
                .strip_prefix("./src/bin/")
                .unwrap()
                .strip_suffix(".rs")
                .unwrap();
            filenames.push(filename.to_string());
        }
    }
    filenames.sort();
    Ok(filenames)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_validate_range_ok_left_edge() {
        let rng = 1..=10;
        let n = 1;
        let name = "n";
        let result = validate_range(name.into(), n, &rng);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_range_ok_mid() {
        let rng = 1..=10;
        let n = 5;
        let name = "n";
        let result = validate_range(name.into(), n, &rng);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_range_ok_right_edge() {
        let rng = 1..=10;
        let n = 10;
        let name = "n";
        let result = validate_range(name.into(), n, &rng);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_range_err_low() {
        let rng = 1..=10;
        let n = 0;
        let name = "n";
        let result = validate_range(name.into(), n, &rng);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_range_err_high() {
        let rng = 1..=10;
        let n = 11;
        let name = "n";
        let result = validate_range(name.into(), n, &rng);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_test_filenames() {
        let mut loc = "number_spiral".to_string();
        let (questions, answers) = get_test_filenames(&mut loc).unwrap();
        let expected_questions = [
            "./data/number_spiral/test_input_001.txt".to_string(),
            "./data/number_spiral/test_input_002.txt".to_string(),
            "./data/number_spiral/test_input_003.txt".to_string(),
        ];
        assert_eq!(questions[0], expected_questions[0]);
        assert_eq!(questions[1], expected_questions[1]);
        assert_eq!(questions[2], expected_questions[2]);
        assert_eq!(questions.len(), expected_questions.len());
        let expected_answers = [
            "./data/number_spiral/test_output_001.txt".to_string(),
            "./data/number_spiral/test_output_002.txt".to_string(),
            "./data/number_spiral/test_output_003.txt".to_string(),
        ];
        assert_eq!(answers[0], expected_answers[0]);
        assert_eq!(answers[1], expected_answers[1]);
        assert_eq!(answers[2], expected_answers[2]);
        assert_eq!(answers.len(), expected_answers.len());
    }

    #[test]
    fn test_get_binary_filenames() {
        let result = get_binary_filenames().unwrap();
        let expected_results = [
            "missing_number".to_string(),
            "weird_algorithm".to_string(),
            "repetitions".to_string(),
        ];
        for r in expected_results.iter() {
            assert!(result.contains(r));
        }
    }
}
