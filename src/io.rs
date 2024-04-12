use std::str::{FromStr, SplitAsciiWhitespace};

use crate::prelude::*;

/// Converts a vector of type T to a string.
/// # Arguments
/// * `v` - A vector of type T.
/// * `sep` - An optional separator string.
/// # Returns
/// A string containing the elements of the vector separated by the separator.
pub fn vector_to_string<T: ToString>(v: Vec<T>, sep: Option<&str>) -> String {
    match sep {
        Some(sep) => v
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(sep),
        None => v.into_iter().map(|e| e.to_string()).collect(),
    }
}

/// Reads n lines from standard input and parses them into a vector of tuples of type (u64, u64).
/// # Arguments
/// * `n` - The number of lines to read.
/// # Returns
/// A Result containing a vector of tuples of type (u64, u64) or an error.
pub fn get_tuple_vector(n: u64) -> Result<Vec<(u64, u64)>> {
    let mut v: Vec<(u64, u64)> = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let mut it = input.split_ascii_whitespace();
        let token = it.next().unwrap();
        let a: u64 = match token.parse() {
            Ok(r) => r,
            Err(_) => return Err(Error::ParseError(format!("get_tuple_vector: {}", token))),
        };
        let token = it.next().unwrap();
        let b: u64 = match token.parse() {
            Ok(r) => r,
            Err(_) => return Err(Error::ParseError(format!("get_tuple_vector: {}", token))),
        };
        v.push((a, b));
    }
    Ok(v)
}

/// # get_vector_vector_bool()
/// Reads 8 lines from standard input and parses them into a vector of vectors of bool.
/// # Returns
/// A Result containing a vector of vectors of bool or an error.
pub fn get_vector_vector_bool() -> Result<Vec<Vec<bool>>> {
    let mut v: Vec<Vec<bool>> = Vec::new();
    let mut b: Vec<bool> = Vec::new();
    for _ in 0..8 {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input)?;
        let chars = input.chars();
        for c in chars {
            b.push(c == '.');
        }
        v.push(b.clone());
        b.clear();
    }
    Ok(v)
}

/// Parses a token into a type T.
/// # Arguments
/// * `tokens` - A mutable reference to a SplitAsciiWhitespace.
/// # Returns
/// A Result containing a type T or an error.
pub fn get_token<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> Result<T> {
    if let Some(token) = tokens.next() {
        match token.parse::<T>() {
            Ok(r) => Ok(r),
            Err(_) => Err(Error::ParseError(format!("get_token: {}", token))),
        }
    } else {
        Err(Error::ParseError(
            "get_token(): expected Some, got None".to_string(),
        ))
    }
}

/// Reads all from standard input and returns a SplitAsciiWhitespace.
/// Must send EOF (Ctrl-D Mac & Linux, Ctrl+Z Windows) to signal the end of input when using stdin as Reader.
/// # Arguments
/// * `reader` - A BufReader.
/// * `b` - A mutable reference to a string.
/// # Returns
/// A Result containing a SplitAsciiWhitespace or an error.
pub fn load_all_tokens<R>(mut reader: R, buffer: &mut String) -> Result<SplitAsciiWhitespace>
where
    R: std::io::BufRead,
{
    reader.read_to_string(buffer)?;
    Ok(buffer.split_ascii_whitespace())
}

/// Reads a line standard input and returns a SplitAsciiWhitespace.
/// # Arguments
/// * `reader` - A BufReader.
/// * `buffer` - A mutable reference to a string.
/// # Returns
/// A Result containing a SplitAsciiWhitespace or an error.
pub fn load_tokens<R>(mut reader: R, buffer: &mut String) -> Result<SplitAsciiWhitespace>
where
    R: std::io::BufRead,
{
    reader.read_line(buffer)?;
    Ok(buffer.split_ascii_whitespace())
}

pub fn get_vector<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> Result<Vec<T>> {
    let mut v: Vec<T> = Vec::new();
    for token in tokens.by_ref() {
        match token.parse::<T>() {
            Ok(r) => v.push(r),
            Err(_) => return Err(Error::ParseError(format!("vector_from_tokens: {}", token))),
        }
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_to_string() {
        let v = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let u = "a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z"
            .to_string();
        assert_eq!(vector_to_string(v, Some(", ")), u);
    }

    // #[test]
    // fn test_string_to_vector() {
    //     let s = String::from("1 2 3 4 5");
    //     let v: Vec<u64> = string_to_vector(s);
    //     assert_eq!(v, vec![1, 2, 3, 4, 5]);
    // }
    //
    // #[test]
    // fn test_vector_to_string() {
    //     let v = vec!['a', 'b', 'c', 'd', 'e'];
    //     assert_eq!("a, b, c, d, e".to_string(), vector_to_string(v, Some(", ")));
    // }
    //
    // #[test]
    // fn test_get_vector() {
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     let expected = vec![1, 2, 3, 4, 5];
    //     assert_eq!(get_vector::<u64>().unwrap(), expected);
    // }
}
