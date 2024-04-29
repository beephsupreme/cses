use std::fmt::Write;
use std::io::{BufReader, Read};
use std::str::SplitAsciiWhitespace;

fn main() {
    let mut buffer = String::new();
    let mut tokens = load_tokens(&mut buffer);
    let n: u64 = get_token(&mut tokens);
    buffer.clear();
    match n {
        1 => println!("1"),
        0 | 2 | 3 => write!(buffer, "NO SOLUTION").unwrap(),
        _ => {
            let half = n / 2;
            for i in 0..half {
                write!(buffer, "{} ", 2 * i + 2).unwrap();
            }
            for i in 0..n - half {
                write!(buffer, "{} ", 2 * i + 1).unwrap();
            }
        }
    };
    println!("{buffer}");
}

fn get_token<T: std::str::FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    if let Some(token) = tokens.next() {
        match token.parse::<T>() {
            Ok(r) => r,
            Err(_) => panic!("PARSE ERROR"),
        }
    } else {
        panic!("EXPECTED SOME, GOT NONE");
    }
}

fn load_tokens(buffer: &mut String) -> SplitAsciiWhitespace {
    let mut reader = BufReader::new(std::io::stdin());
    reader.read_to_string(buffer).expect("READ ERROR");
    buffer.split_ascii_whitespace()
}
