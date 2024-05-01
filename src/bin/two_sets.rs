use std::fmt::Write;
use std::io::{BufReader, Read};
use std::str::SplitAsciiWhitespace;

fn main() {
    let mut buffer = String::new();
    let mut tokens = load_tokens(&mut buffer);
    let n: u64 = get_token(&mut tokens);
    buffer.clear();

    let (mut a, mut b) = (Vec::new(), Vec::new());
    let mut sum = n * (n + 1) / 2;
    if sum % 2 != 0 {
        println!("NO");
        return;
    } else {
        sum /= 2;
    }
    for i in (1..=n).rev() {
        if i <= sum {
            sum -= i;
            a.push(i);
        } else {
            b.push(i)
        }
    }
    writeln!(buffer, "YES").unwrap();
    writeln!(buffer, "{}", a.len()).unwrap();
    a.iter().for_each(|e| write!(buffer, "{} ", e).unwrap());
    writeln!(buffer, "\n{}", b.len()).unwrap();
    b.iter().for_each(|e| write!(buffer, "{} ", e).unwrap());
    writeln!(buffer).unwrap();
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
