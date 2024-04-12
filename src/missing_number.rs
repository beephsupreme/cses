use std::collections::HashSet;
use std::ops::RangeInclusive;

use crate::prelude::*;
use crate::utils::validate_range;

const VALID_RANGE: RangeInclusive<u64> = 2..=200_000;

/// The problem is described in detail at https://cses.fi/problemset/task/1083.
/// Given a sequence of n - 1 distinct integers in the range 1, 2, ..., n, find the missing number.
/// #
/// # Parameters
/// * n: A positive number n such that 2 <= n <= 200_000.
/// * v: A vector of n - 1 distinct elements such that 1 <= v[i] <= n.
/// #
/// # Returns
/// A Result which contains the missing number in the sequence or an error.  
/// #
/// # Performance
/// The time complexity of this solution is O(n).
/// The space complexity of this solution is O(n).

pub fn missing_number(n: u64, v: Vec<u64>) -> Result<u64> {
    let _ = validate_range(n, VALID_RANGE)?;
    if v.len() != (n - 1) as usize {
        return Err(Error::SizeMismatch(v.len(), (n - 1) as usize));
    }
    let vmax = *v.iter().max().unwrap();
    if vmax > n {
        return Err(Error::OutOfRange(n.to_string()));
    }
    let vmin = *v.iter().min().unwrap();
    if vmin == 0 {
        return Err(Error::OutOfRange(vmin.to_string()));
    }
    let mut unique: HashSet<u64> = HashSet::new();
    v.iter().all(|x| unique.insert(*x));
    if unique.len() != (n - 1) as usize {
        return Err(Error::InvalidInput(f!(
            "{} / {}",
            unique.len(),
            (n - 1) as usize
        )));
    }
    let sum: u64 = n * (n + 1) / 2;
    let partial_sum: u64 = v.iter().sum::<u64>();
    Ok(sum - partial_sum)
}

#[cfg(test)]

mod tests {
    use super::*;

    // #[test]
    // fn missing_number_0() {
    //     let expected: Error = Error::OutOfRange(0.to_string());
    //     assert_eq!(
    //         missing_number(5, vec![0, 1, 2, 3]).unwrap(),
    //         Err(expected).unwrap()
    //     );
    // }
    #[test]
    fn missing_number_1() {
        assert_eq!(missing_number(5, vec![2, 3, 4, 5]).unwrap(), 1)
    }

    #[test]
    fn missing_number_3() {
        assert_eq!(missing_number(5, vec![1, 2, 4, 5]).unwrap(), 3)
    }

    #[test]
    fn missing_number_5() {
        assert_eq!(missing_number(5, vec![1, 2, 3, 4]).unwrap(), 5)
    }

    // #[test]
    // fn missing_number_overflow() {
    //     let n: u64 = 5;
    //     let expected: Error = Error::OutOfRange(n.to_string());
    //     assert_eq!(
    //         missing_number(n, vec![1, 2, 3, 6]).unwrap(),
    //         Err(expected).unwrap()
    //     );
    // }
    //
    // #[test]
    // fn missing_number_200001() {
    //     let expected: Error =
    //         Error::InvalidInput("n is 200001 but expected 2 ≤ n ≤ 200000".to_string());
    //     let mut v: Vec<u64> = Vec::new();
    //     (1..200_001).for_each(|i| v.push(i as u64));
    //     assert_eq!(missing_number(200_001, v).unwrap(), Err(expected).unwrap());
    // }
}
