use std::io::{BufReader, Read};
use std::str::SplitAsciiWhitespace;

fn main() {
    let mut buffer: String = String::new();
    let mut tokens = load_tokens(&mut buffer);
    let n: u64 = get_token(&mut tokens);
    let mut previous: u64 = get_token(&mut tokens);
    let mut current: u64 = 0;
    for _ in 1..n {
        let e: u64 = get_token(&mut tokens);
        if e < previous {
            current += previous - e;
        } else {
            previous = e;
        }
    }
    println!("{}", current);
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
