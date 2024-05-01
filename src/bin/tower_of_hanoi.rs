use std::fmt::Write;
use std::io::{BufReader, Read};
use std::str::SplitAsciiWhitespace;

fn main() {
    let mut buffer = String::new();
    let mut tokens = load_tokens(&mut buffer);
    let n: u8 = get_token(&mut tokens);
    let mut v: Vec<(u8, u8)> = Vec::new();
    hanoi(n, 1, 3, &mut v);
    buffer.clear();
    writeln!(buffer, "{}", v.len()).unwrap();
    v.iter().for_each(|(a, b)| writeln!(buffer, "{} {}", a, b).unwrap());
    print!("{}", buffer)
}

fn hanoi(n: u8, a: u8, b: u8, v: &mut Vec<(u8, u8)>) {
    if n == 1 {
        v.push((a, b));
    } else {
        let c: u8 = 6 - (a + b);
        hanoi(n - 1, a, c, v);
        v.push((a, b));
        hanoi(n - 1, c, b, v)
    }
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
