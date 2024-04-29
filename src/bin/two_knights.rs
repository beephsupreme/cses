use std::fmt::Write;
use std::io::{BufReader, Read};
use std::str::SplitAsciiWhitespace;

fn main() {
    let mut buffer = String::new();
    let mut tokens = load_tokens(&mut buffer);
    let n: i64 = get_token(&mut tokens);
    buffer.clear();
    for i in 1..=n {
        writeln!(buffer, "{}", (i.pow(4) - 9 * i.pow(2) + 24 * i - 16) / 2).unwrap();
    }
    print!("{}", buffer);
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
