use crate::io::{get_token, load_all_tokens};
use crate::prelude::*;
use crate::utils::validate_range;

/// The problem is described in detail at https://cses.fi/problemset/task/1071.
/// Given a sequence of n coordinates, 1 distinct integers in the range 1, 2, ..., n, find the missing number.
/// # Parameters
/// * n: A positive number n such that 1 <= n <= 1e5.
/// * u: A vector of n elements e such that 1 <= e <= 1e9.
/// * v: A vector of n elements e such that 1 <= e <= 1e9.
/// # Returns
/// A vector containing the value of the number spiral at the given coordinate
/// # Performance
/// The time complexity of this solution is O(n).
/// The space complexity of this solution is O(n).
pub fn number_spiral(n: u64, v: Vec<(u64, u64)>) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();
    for i in 0..n {
        let (a, b) = v[i as usize];
        let max = u64::max(a, b);
        let d = 1 + max * (max - 1);
        if max & 1 == 0 {
            output.push(d + a - b);
        } else {
            output.push(d + b - a);
        }
    }
    output
}

/// get a tuple of (u64, Vec<(u64, u64)>) from stdin
pub fn number_spiral_get_input<R>(reader: R) -> Result<(u64, Vec<(u64, u64)>)> 
where
    R: std::io::BufRead,
{
    let mut buffer = String::new();
    let mut tokens = load_all_tokens(reader, &mut buffer)?;
    let n: u64 = get_token(&mut tokens)?;
    let mut v: Vec<(u64, u64)> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let a: u64 = get_token(&mut tokens)?;
        let b: u64 = get_token(&mut tokens)?;
        v.push((a, b));
    }
    Ok((n, v))
}

/// Validates the input for missing_number as follows:
/// * n must be in the range 1 ≤ n ≤ 1e5.
/// * v must contain n elements.
/// * elements e in v must be 1 ≤ e ≤ 1e9.
pub fn validate_number_spiral_input(n: u64, v: &[(u64, u64)]) -> Result<()> {
    let n_range = 1..=100_000;
    let v_range = 1..=1_000_000_000;
    validate_range("n".into(), n, &n_range)?;
    if v.len() != n as usize {
        return Err(LibraryError::SizeMismatch(v.len(), n as usize));
    }
    for e in v.iter() {
        let (a, b) = *e;
        validate_range("v_a".into(), a, &v_range)?;
        validate_range("v_b".into(), b, &v_range)?;
    }
    Ok(())
}

#[cfg(test)]

mod tests {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn number_spiral_test_edge_left() {
        let n: u64 = 1;
        let v: Vec<(u64, u64)> = vec![(1, 1)];
        let r: Vec<u64> = vec![1];
        assert_eq!(number_spiral(n, v), r);
    }

    #[test]
    fn number_spiral_test_edge_mid() {
        let n: u64 = 3;
        let v: Vec<(u64, u64)> = vec![(2, 3), (1, 1), (4, 2)];
        let r: Vec<u64> = vec![8, 1, 15];
        assert_eq!(number_spiral(n, v), r);
    }

    #[test]
    fn number_spiral_test_edge_right() {
        let n: u64 = 100_000;
        let v: Vec<(u64, u64)> = vec![(1_000_000_000, 1_000_000_000); 100_000];
        let r: Vec<u64> = vec![999999999000000001; 100_000];
        assert_eq!(number_spiral(n, v), r);
    }

    #[test]
    fn number_spiral_test_get_input() {
        let (n, v) = number_spiral_get_input(Cursor::new("3\n2 3\n1 1\n4 2\n")).unwrap();
        assert_eq!((n, v), (3, vec![(2, 3), (1, 1), (4, 2)]));
    }

    #[test]
    fn number_spiral_validate_input_err_n_too_small() {
        let n: u64 = 0;
        let r = validate_number_spiral_input(n, &[(1, 1)]);
        assert!(r.is_err());
    }

    #[test]
    fn number_spiral_validate_input_err_n_too_big() {
        let n: u64 = 100_001;
        let r = validate_number_spiral_input(n, &[(1, 1)]);
        assert!(r.is_err());
    }

    #[test]
    fn number_spiral_validate_input_err_length_mismatch() {
        let n: u64 = 5;
        let r = validate_number_spiral_input(n, &[(1, 1)]);
        assert!(r.is_err());
    }

    #[test]
    fn number_spiral_validate_input_ok_length_match() {
        let n: u64 = 5;
        let r = validate_number_spiral_input(n, &[(1, 1), (1, 1), (1, 1), (1, 1), (1, 1)]);
        assert!(r.is_ok());
    }

    #[test]
    fn number_spiral_validate_input_err_vector_element_too_small() {
        let n: u64 = 5;
        let r = validate_number_spiral_input(n, &[(1, 1), (1, 1), (1, 1), (1, 1), (0, 1)]);
        assert!(r.is_err());
    }


    #[test]
    fn number_spiral_validate_input_err_vector_element_too_large() {
        let n: u64 = 5;
        let r = validate_number_spiral_input(n, &[(1, 1), (1, 1), (1, 1), (1_000_000_001, 1), (1, 1)]);
        assert!(r.is_err());
    }
}
