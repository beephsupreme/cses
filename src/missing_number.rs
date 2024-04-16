use std::collections::HashSet;

use crate::prelude::*;
use crate::utils::validate_range;

/// The problem is described in detail at https://cses.fi/problemset/task/1083.
/// Given a sequence of n - 1 distinct integers in the range 1, 2, ..., n, find the missing number.
/// # Parameters
/// * n: A positive number n such that 2 <= n <= 200_000.
/// * v: A vector of n - 1 distinct elements e such that 1 <= e <= n.
/// # Returns
/// A Result which contains the missing number in the sequence or an error.
/// # Performance
/// The time complexity of this solution is O(n).
/// The space complexity of this solution is O(n).
pub fn missing_number(n: u64, v: Vec<u64>) -> u64 {
    let sum: u64 = n * (n + 1) / 2;
    let partial_sum: u64 = v.iter().sum::<u64>();
    sum - partial_sum
}

/// Validates the input for missing_number as follows:
/// * n must be in the range 2 ≤ n ≤ 200_000.
/// * v must contain n - 1 elements.
/// * v must contain distinct elements e in the range 1 ≤ e ≤ n.
/// * elements e in v must be 1 ≤ e ≤ n.
pub fn validate_missing_number_input(n: u64, v: &[u64]) -> Result<()> {
    let n_range = 2..=200_000;
    let v_range = 1..=n;
    validate_range("n".into(), n, &n_range)?;
    if v.len() != (n - 1) as usize {
        return Err(LibraryError::SizeMismatch(v.len(), (n - 1) as usize));
    }
    for e in v.iter() {
        validate_range("e".into(), *e, &v_range)?;
    }
    let mut unique: HashSet<u64> = HashSet::new();
    v.iter().all(|x| unique.insert(*x));
    if unique.len() != (n - 1) as usize {
        return Err(LibraryError::SizeMismatch(unique.len(), (n - 1) as usize));
    }
    Ok(())
}

#[cfg(test)]

mod tests {
    use crate::io::get_value_and_vector;

    use super::*;

    #[test]
    fn missing_number_edge_left() {
        assert_eq!(missing_number(5, vec![2, 3, 4, 5]), 1)
    }

    #[test]
    fn missing_number_edge_right() {
        assert_eq!(missing_number(5, vec![1, 2, 4, 5]), 3)
    }

    #[test]
    fn missing_number_mid() {
        assert_eq!(missing_number(5, vec![1, 2, 3, 4]), 5)
    }

    #[test]
    fn test_validate_err_n_small() {
        let input = "0\n1 2 3 4\n";
        let reader = std::io::BufReader::new(std::io::Cursor::new(input));
        let (n, v) = get_value_and_vector(reader).unwrap();
        let r = validate_missing_number_input(n, &v);
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            "n, expected: 2 ≤ n ≤ 200000, found: 0"
        );
    }

    #[test]
    fn test_validate_err_n_large() {
        let input = "200001\n1 2 3 4\n";
        let reader = std::io::BufReader::new(std::io::Cursor::new(input));
        let (n, v) = get_value_and_vector(reader).unwrap();
        let r = validate_missing_number_input(n, &v);
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            "n, expected: 2 ≤ n ≤ 200000, found: 200001"
        );
    }

    #[test]
    fn test_validate_err_vector_size() {
        let input = "5\n1 2 3\n";
        let reader = std::io::BufReader::new(std::io::Cursor::new(input));
        let (n, v) = get_value_and_vector(reader).unwrap();
        let r = validate_missing_number_input(n, &v);
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().to_string(), "expected 3, found: 4");
    }

    #[test]
    fn test_validate_err_duplicate_elements() {
        let input = "5\n1 2 3 3\n";
        let reader = std::io::BufReader::new(std::io::Cursor::new(input));
        let (n, v) = get_value_and_vector(reader).unwrap();
        let r = validate_missing_number_input(n, &v);
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().to_string(), "expected 3, found: 4");
    }

    #[test]
    fn test_validate_err_vector_element_too_large() {
        let input = "5\n1 2 3 6\n";
        let reader = std::io::BufReader::new(std::io::Cursor::new(input));
        let (n, v) = get_value_and_vector(reader).unwrap();
        let r = validate_missing_number_input(n, &v);
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            "e, expected: 1 ≤ e ≤ 5, found: 6"
        );
    }

    #[test]
    fn test_validate_err_vector_element_0() {
        let input = "5\n1 2 3 0\n";
        let reader = std::io::BufReader::new(std::io::Cursor::new(input));
        let (n, v) = get_value_and_vector(reader).unwrap();
        let r = validate_missing_number_input(n, &v);
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            "e, expected: 1 ≤ e ≤ 5, found: 0"
        );
    }
}
