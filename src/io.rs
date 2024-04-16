use std::str::{FromStr, SplitAsciiWhitespace};

use crate::prelude::*;

/// Converts a vector of type T to a string.
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

/// Parses a token of SplitAsciiWhitespace into a type T.
pub fn get_token<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> Result<T> {
    if let Some(token) = tokens.next() {
        match token.parse::<T>() {
            Ok(r) => Ok(r),
            Err(_) => Err(LibraryError::ParseError(format!("get_token: {}", token))),
        }
    } else {
        Err(LibraryError::ParseError(
            "get_token(): expected Some, got None".to_string(),
        ))
    }
}

/// Reads all from standard input and returns a SplitAsciiWhitespace.
/// Must send EOF (Ctrl-D Mac & Linux, Ctrl+Z Windows) to signal the end of input when using stdin as Reader.
pub fn load_all_tokens<R>(mut reader: R, buffer: &mut String) -> Result<SplitAsciiWhitespace>
where
    R: std::io::BufRead,
{
    reader.read_to_string(buffer)?;
    Ok(buffer.split_ascii_whitespace())
}

/// Reads a line standard input and returns a SplitAsciiWhitespace.
pub fn load_tokens<R>(mut reader: R, buffer: &mut String) -> Result<SplitAsciiWhitespace>
where
    R: std::io::BufRead,
{
    reader.read_line(buffer)?;
    Ok(buffer.split_ascii_whitespace())
}

/// Parses a SplitAsciiWhitespace into a vector of type T.
pub fn get_vector<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> Result<Vec<T>> {
    let mut v: Vec<T> = Vec::new();
    for token in tokens.by_ref() {
        match token.parse::<T>() {
            Ok(r) => v.push(r),
            Err(_) => {
                return Err(LibraryError::ParseError(format!(
                    "vector_from_tokens: {}",
                    token
                )))
            }
        }
    }
    Ok(v)
}

/// Reads input from a BufRead and returns a tuple of type T and a vector of type T.
pub fn get_value_and_vector<R, T>(reader: R) -> Result<(T, Vec<T>)>
where
    R: std::io::BufRead,
    T: FromStr,
{
    let mut buffer: String = String::new();
    let mut tokens = load_all_tokens(reader, &mut buffer)?;
    let n: T = get_token(&mut tokens)?;
    let v: Vec<T> = get_vector(&mut tokens)?;
    Ok((n, v))
}

/// Reads input from a BufRead and returns a single value of type T.
pub fn get_value<R, T>(reader: R) -> Result<T>
where
    R: std::io::BufRead,
    T: FromStr,
{
    let mut buffer: String = String::new();
    let mut tokens = load_tokens(reader, &mut buffer)?;
    let n: T = get_token(&mut tokens)?;
    Ok(n)
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

    #[test]
    fn test_get_value() {
        let input = "ABCDEFGHIJKLMNOP\n".as_bytes();
        let reader = std::io::BufReader::new(input);
        let result: String = get_value(reader).unwrap();
        assert_eq!(result, "ABCDEFGHIJKLMNOP".to_string());
    }
}
