use std::collections::VecDeque;
use std::io::{BufReader, Read};

fn main() {
    let mut tokens = Scanner::default();
    let n: u64 = tokens.next();
    let mut sum: u64 = n * (n + 1) / 2;
    for _ in 0..n - 1 {
        sum -= tokens.next::<u64>();
    }
    println!("{sum}");
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
