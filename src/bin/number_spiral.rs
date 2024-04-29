use std::fmt::Write;
use std::io::{BufReader, Read};
use std::str::SplitAsciiWhitespace;

fn main() {
    let mut buffer = String::new();
    let mut output = String::new();
    let mut tokens = load_tokens(&mut buffer);
    let n: u64 = get_token(&mut tokens);
    for _ in 0..n {
        let a: u64 = get_token(&mut tokens);
        let b: u64 = get_token(&mut tokens);
        let max = u64::max(a, b);
        let d = 1 + max * (max - 1);
        if max & 1 == 0 {
            writeln!(output, "{}", d + a - b).unwrap();
        } else {
            writeln!(output, "{}", d + b - a).unwrap();
        }
    }
    println!("{}", output);
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
