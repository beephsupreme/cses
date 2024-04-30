use std::fmt::Write;
use std::io::{BufReader, Read};
use std::str::SplitAsciiWhitespace;

fn main() {
    let mut buffer = String::new();
    let mut tokens = load_tokens(&mut buffer);
    let n: usize = get_token(&mut tokens);
    buffer.clear();
    for i in 0..(1 << n) {
        let gray_code = i ^ (i >> 1);
        write!(buffer, "{:0width$b}\n", gray_code, width = n).unwrap();
    } 
    println!("{}", buffer);
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

