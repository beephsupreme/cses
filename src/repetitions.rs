use crate::prelude::*;
use crate::utils::validate_range;

/// The problem is described in detail at https://cses.fi/problemset/task/1069
/// Given a DNA sequence, find the length of the longest repetition in the sequence.
/// # Parameters
/// * input: A string of n characters such that 1 ≤ n ≤ 1_000_000.
/// * The string contains only the characters 'A', 'C', 'G', and 'T'.
/// # Returns
/// The length of the longest repetition in the sequence.
/// # Performance
/// The time complexity of this solution is O(n).
/// The space complexity of this solution is O(1).
pub fn repetitions(input: String) -> u64 {
    let mut streak = 1u64;
    let mut longest = 1u64;
    let mut prev = b'@';
    input.as_bytes().iter().for_each(|&ch| match ch == prev {
        true => {
            streak += 1;
            longest = std::cmp::max(longest, streak);
        }
        false => {
            streak = 1;
            prev = ch;
        }
    });
    longest
}

/// Validates the input for the repetitions problem as follows:
/// * n must be in the range 1 ≤ n ≤ 1_000_000.
/// * input must contain only the characters 'A', 'C', 'G', and 'T'.
#[allow(dead_code)]
pub fn validate_repetitions_input(input: &str) -> Result<()> {
    let range = 1..=1_000_000;
    let n = input.len() as u64;
    validate_range("n".into(), n, &range)?;
    for ch in input.chars() {
        match ch {
            'A' | 'C' | 'G' | 'T' => continue,
            _ => return Err(LibraryError::InvalidInput(f!("Invalid character: {ch}",))),
        }
    }
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_repetitions_left() {
        let input = "AAAATTCGGGA".to_string();
        assert_eq!(repetitions(input), 4);
    }

    #[test]
    fn test_repetitions_right() -> Result<()> {
        let input = "AATTCGGGAAAA".to_string();
        assert_eq!(repetitions(input), 4);
        Ok(())
    }

    #[test]
    fn test_repetitions_mid() -> Result<()> {
        let input = "TTAAAACGG".to_string();
        assert_eq!(repetitions(input), 4);
        Ok(())
    }

    #[test]
    fn test_get_repetitions_input_pass() {
        let input = "AAAAATTCGGGA".to_string();
        assert!(validate_repetitions_input(&input).is_ok());
    }
    #[test]
    fn test_get_repetitions_input_err_empty() {
        let input = "".to_string();
        assert!(validate_repetitions_input(&input).is_err());
    }

    #[test]
    fn test_get_repetitions_input_err_large() {
        let input = String::from_utf8(vec![b'A'; 1_000_001]).unwrap();
        assert!(validate_repetitions_input(&input).is_err());
    }

    #[test]
    fn test_get_repetitions_input_err_lowercase_valid_char() {
        let input = "aaccggtt".to_string();
        assert!(validate_repetitions_input(&input).is_err());
    }

    #[test]
    fn test_get_repetitions_input_err_invalid_char() {
        let input = "AACCXGGTTT".to_string();
        assert!(validate_repetitions_input(&input).is_err());
    }
}
