use crate::prelude::*;
use crate::utils::validate_range;

/// The problem is described in detail at https://cses.fi/problemset/task/1094
/// You are given an array of n integers. You want to modify the array so that it is increasing.
/// An array is increasing if the next element is greater than or equal to the current element.
/// # Parameters
/// * n: The number of elements in the array such that 1 ≤ n ≤ 2 * 10^5.
/// * v: A vector of n integers such that 1 ≤ v[i] ≤ 10^9.
/// # Returns
/// The minimum number of operations needed to make the array increasing.
/// # Performance
/// The time complexity of this solution is O(n).
/// The space complexity of this solution is O(1).
pub fn increasing_array(v: Vec<u64>) -> u64 {
    let mut previous = v[0];
    let mut current: u64 = 0;
    for e in v {
        if e < previous {
            current += previous - e;
        } else {
            previous = e;
        }
    }
    current
}

/// Validate the input for the increasing_array function as follows:
/// * n must be in the range 1 ≤ n ≤ 200_000.
/// * v must contain n elements.
/// * v must contain elements e in the range 1 ≤ e ≤ 1_000_000_000.
pub fn validate_increasing_array_input(n: u64, v: &[u64]) -> Result<()> {
    let range = 1..=200_000;
    validate_range("n".into(), n, &range)?;
    if n != v.len() as u64 {
        return Err(LibraryError::InvalidInput(format!(
            "Expected {} elements, but got {}",
            n,
            v.len()
        )));
    }
    let range = 1..=1_000_000_000;
    for e in v.iter() {
        validate_range("v".into(), *e, &range)?;
    }
    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(increasing_array(vec![3, 2, 5, 1, 7]), 5)
    }

    #[test]
    fn unit_02() {
        assert_eq!(increasing_array(vec![6, 10, 4, 10, 2, 8, 9, 2, 7, 7]), 31)
    }

    #[test]
    fn unit_03() {
        assert_eq!(increasing_array(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0)
    }

    #[test]
    fn unit_04() {
        assert_eq!(
            increasing_array(vec![1000000000, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            8999999991
        )
    }
}
