use std::collections::VecDeque;
use std::fmt::Write;
use std::io::{BufReader, Read};

fn main() {
    let mut tokens = Scanner::default();
    let n: u64 = tokens.next();
    let mut output: String = String::new();
    for _ in 0..n {
        let a: u64 = tokens.next();
        let b: u64 = tokens.next();
        if a > 2 * b || b > 2 * a || (a + b) % 3 != 0 {
            writeln!(output, "NO").unwrap();
        } else {
            writeln!(output, "YES").unwrap();
        }
    }
    print!("{}", output);
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
