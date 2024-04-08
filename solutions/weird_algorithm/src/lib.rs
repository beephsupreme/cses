use std::ops::RangeInclusive;

use utils::error::CsesError;

const VALID_RANGE: RangeInclusive<u64> = 1..=1_000_000;

pub fn weird_algorithm(mut n: u64) -> Result<Vec<u64>, CsesError> {
    let _ = valid_input(n)?;
    let mut v: Vec<u64> = Vec::with_capacity(4096);
    loop {
        v.push(n);
        if n == 1 {
            break;
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    Ok(v)
}

fn valid_input(n: u64) -> Result<bool, CsesError> {
    match VALID_RANGE.contains(&n) {
        true => Ok(true),
        false => Err(CsesError::InvalidInput(format!(
            "n is {} but expected {} ≤ n ≤ {}",
            n,
            VALID_RANGE.start(),
            VALID_RANGE.end()
        ))),
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn weird_algorithm_unit_0() {
        let n: u64 = 0;
        let expected: CsesError =
            CsesError::InvalidInput("n is 0 but expected 1 ≤ n ≤ 1000000".to_string());
        let result = weird_algorithm(n);
        assert_eq!(result, Err(expected));
    }

    #[test]
    fn weird_algorithm_unit_1_000_001() {
        let n: u64 = 1_000_001;
        let expected: CsesError =
            CsesError::InvalidInput("n is 1000001 but expected 1 ≤ n ≤ 1000000".to_string());
        let result = weird_algorithm(n);
        assert_eq!(result, Err(expected));
    }

    #[test]
    fn weird_algorithm_unit_1() {
        let n: u64 = 1;
        let expected: Vec<u64> = vec![1];
        let result = weird_algorithm(n);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn weird_algorithm_unit_23() {
        let n: u64 = 23;
        let expected: Vec<u64> = vec![23, 70, 35, 106, 53, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1];
        let result = weird_algorithm(n);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn weird_algorithm_unit_1_000_000() {
        let n: u64 = 1000000;
        let expected: Vec<u64> = vec![
            1000000, 500000, 250000, 125000, 62500, 31250, 15625, 46876, 23438, 11719, 35158,
            17579, 52738, 26369, 79108, 39554, 19777, 59332, 29666, 14833, 44500, 22250, 11125,
            33376, 16688, 8344, 4172, 2086, 1043, 3130, 1565, 4696, 2348, 1174, 587, 1762, 881,
            2644, 1322, 661, 1984, 992, 496, 248, 124, 62, 31, 94, 47, 142, 71, 214, 107, 322, 161,
            484, 242, 121, 364, 182, 91, 274, 137, 412, 206, 103, 310, 155, 466, 233, 700, 350,
            175, 526, 263, 790, 395, 1186, 593, 1780, 890, 445, 1336, 668, 334, 167, 502, 251, 754,
            377, 1132, 566, 283, 850, 425, 1276, 638, 319, 958, 479, 1438, 719, 2158, 1079, 3238,
            1619, 4858, 2429, 7288, 3644, 1822, 911, 2734, 1367, 4102, 2051, 6154, 3077, 9232,
            4616, 2308, 1154, 577, 1732, 866, 433, 1300, 650, 325, 976, 488, 244, 122, 61, 184, 92,
            46, 23, 70, 35, 106, 53, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1,
        ];
        let result = weird_algorithm(n);
        assert_eq!(result.unwrap(), expected);
    }
}
