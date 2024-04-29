use std::fmt::Write;
use std::io::{BufReader, Read};
use std::str::SplitAsciiWhitespace;

fn main() {
    let mut buffer = String::new();
    let mut tokens = load_tokens(&mut buffer);
    let n: u64 = get_token(&mut tokens);
    let mut output: String = String::new();
    for _ in 0..n {
        let a: u64 = get_token(&mut tokens);
        let b: u64 = get_token(&mut tokens);
        if a > 2 * b || b > 2 * a || (a + b) % 3 != 0 {
            writeln!(output, "NO").unwrap();
        } else {
            writeln!(output, "YES").unwrap();
        }
    }
    print!("{}", output);
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
