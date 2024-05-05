use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{BufReader, Read};

fn main() {
    let mut buffer = String::new();
    let mut tokens = Scanner::default();
    let n: i64 = tokens.next();
    buffer.clear();
    for i in 1..=n {
        writeln!(buffer, "{}", (i.pow(4) - 9 * i.pow(2) + 24 * i - 16) / 2).unwrap();
    }
    print!("{}", buffer);
}

#[derive(Default)]
struct Scanner {
    buffer: VecDeque<String>,
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop_front() {
                return token.parse().ok().expect("PARSE ERROR");
            }
            let mut input = String::new();
            let mut reader = BufReader::new(std::io::stdin());
            reader.read_to_string(&mut input).expect("READ ERROR");
            self.buffer = input.split_ascii_whitespace().map(String::from).collect();
        }
    }
}
