use crate::prelude::*;
use crate::utils::validate_range;

const VALID_RANGE: std::ops::RangeInclusive<u64> = 1..=1_000_000;

pub fn repetitions(input: String) -> Result<u64> {
    let n = input.len() as u64;
    let _ = validate_range(n, VALID_RANGE)?;
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
    Ok(longest)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn repetitions_unit_attcggga() -> Result<()> {
        let input = "ATTCGGGA".to_string();
        assert_eq!(repetitions(input)?, 3);
        Ok(())
    }

    #[test]
    fn repetitions_unit_aattcgggaaaa() -> Result<()> {
        let input = "AATTCGGGAAAA".to_string();
        assert_eq!(repetitions(input)?, 4);
        Ok(())
    }

    #[test]
    fn repetitions_unit_aaattcggga() -> Result<()> {
        let input = "AAATTCGGGA".to_string();
        assert_eq!(repetitions(input)?, 3);
        Ok(())
    }

    #[test]
    fn repetitions_unit_aaaaattcggga() -> Result<()> {
        let input = "AAAAATTCGGGA".to_string();
        assert_eq!(repetitions(input)?, 5);
        Ok(())
    }
}
