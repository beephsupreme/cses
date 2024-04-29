use std::io::{BufReader, Read};
use std::str::SplitAsciiWhitespace;

fn main() {
    let mut buffer: String = String::new();
    let mut tokens = load_tokens(&mut buffer);
    let input: String = get_token(&mut tokens);
    let mut streak: u64 = 1;
    let mut longest: u64 = 1;
    let mut prev: u8 = b'@';
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
    println!("{longest}");
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
